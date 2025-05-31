//! 重新加载管理器模块
//!
//! 负责协调编译和重新加载流程

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use super::change_detector::{ChangeDetector, ChangeType, FileChange};

/// 重新加载配置
#[derive(Debug, Clone)]
pub struct ReloadConfig {
    /// 构建命令
    pub build_command: String,
    /// 构建参数
    pub build_args: Vec<String>,
    /// 工作目录
    pub working_directory: PathBuf,
    /// 构建超时时间（秒）
    pub build_timeout_seconds: u64,
    /// 是否启用并行构建
    pub enable_parallel_build: bool,
    /// 最大并行任务数
    pub max_parallel_tasks: usize,
    /// 是否启用增量构建
    pub enable_incremental_build: bool,
    /// 是否在构建失败时重试
    pub retry_on_failure: bool,
    /// 最大重试次数
    pub max_retries: usize,
    /// 重试间隔（毫秒）
    pub retry_interval_ms: u64,
    /// 是否启用详细输出
    pub verbose_output: bool,
    /// 环境变量
    pub environment_variables: HashMap<String, String>,
}

impl Default for ReloadConfig {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("RUST_LOG".to_string(), "info".to_string());

        Self {
            build_command: "cargo".to_string(),
            build_args: vec!["build".to_string()],
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            build_timeout_seconds: 300, // 5分钟
            enable_parallel_build: true,
            max_parallel_tasks: num_cpus::get(),
            enable_incremental_build: true,
            retry_on_failure: true,
            max_retries: 3,
            retry_interval_ms: 1000,
            verbose_output: false,
            environment_variables: env_vars,
        }
    }
}

/// 重新加载事件
#[derive(Debug, Clone)]
pub enum ReloadEvent {
    /// 构建开始
    BuildStarted {
        files: Vec<PathBuf>,
        build_type: BuildType,
        timestamp: SystemTime,
    },
    /// 构建进度
    BuildProgress {
        current: usize,
        total: usize,
        message: String,
        timestamp: SystemTime,
    },
    /// 构建完成
    BuildCompleted {
        success: bool,
        duration: Duration,
        output: String,
        errors: Vec<String>,
        warnings: Vec<String>,
        timestamp: SystemTime,
    },
    /// 热重载完成
    HotReloadCompleted {
        affected_files: Vec<PathBuf>,
        timestamp: SystemTime,
    },
    /// 错误发生
    Error {
        message: String,
        error_type: ReloadErrorType,
        timestamp: SystemTime,
    },
}

/// 构建类型
#[derive(Debug, Clone, PartialEq)]
pub enum BuildType {
    /// 完整构建
    Full,
    /// 增量构建
    Incremental,
    /// 热重载（仅CSS）
    HotReload,
    /// 测试构建
    Test,
    /// 发布构建
    Release,
}

/// 重新加载错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ReloadErrorType {
    /// 构建失败
    BuildFailed,
    /// 超时
    Timeout,
    /// 依赖错误
    DependencyError,
    /// 配置错误
    ConfigError,
    /// 系统错误
    SystemError,
}

/// 构建任务
#[derive(Debug, Clone)]
struct BuildTask {
    /// 任务ID
    id: String,
    /// 文件列表
    files: Vec<PathBuf>,
    /// 构建类型
    build_type: BuildType,
    /// 优先级
    priority: u8,
    /// 创建时间
    created_at: Instant,
    /// 重试次数
    retry_count: usize,
}

impl BuildTask {
    fn new(files: Vec<PathBuf>, build_type: BuildType, priority: u8) -> Self {
        Self {
            id: format!("{:?}_{}", build_type, Instant::now().elapsed().as_nanos()),
            files,
            build_type,
            priority,
            created_at: Instant::now(),
            retry_count: 0,
        }
    }
}

/// 构建结果
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// 是否成功
    pub success: bool,
    /// 构建时间
    pub duration: Duration,
    /// 标准输出
    pub stdout: String,
    /// 标准错误
    pub stderr: String,
    /// 退出代码
    pub exit_code: Option<i32>,
    /// 错误列表
    pub errors: Vec<String>,
    /// 警告列表
    pub warnings: Vec<String>,
}

/// 构建统计
#[derive(Debug, Clone, Default)]
pub struct BuildStats {
    /// 总构建次数
    pub total_builds: usize,
    /// 成功构建次数
    pub successful_builds: usize,
    /// 失败构建次数
    pub failed_builds: usize,
    /// 平均构建时间
    pub average_build_time: Duration,
    /// 最快构建时间
    pub fastest_build_time: Option<Duration>,
    /// 最慢构建时间
    pub slowest_build_time: Option<Duration>,
    /// 热重载次数
    pub hot_reload_count: usize,
    /// 增量构建次数
    pub incremental_build_count: usize,
    /// 完整构建次数
    pub full_build_count: usize,
}

impl BuildStats {
    fn record_build(&mut self, build_type: &BuildType, duration: Duration, success: bool) {
        self.total_builds += 1;

        if success {
            self.successful_builds += 1;
        } else {
            self.failed_builds += 1;
        }

        // 更新平均构建时间
        let total_time = self.average_build_time * self.total_builds as u32 + duration;
        self.average_build_time = total_time / self.total_builds as u32;

        // 更新最快/最慢时间
        if self.fastest_build_time.is_none() || duration < self.fastest_build_time.unwrap() {
            self.fastest_build_time = Some(duration);
        }
        if self.slowest_build_time.is_none() || duration > self.slowest_build_time.unwrap() {
            self.slowest_build_time = Some(duration);
        }

        // 按类型统计
        match build_type {
            BuildType::Full => self.full_build_count += 1,
            BuildType::Incremental => self.incremental_build_count += 1,
            BuildType::HotReload => self.hot_reload_count += 1,
            _ => {}
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_builds == 0 {
            return 0.0;
        }
        self.successful_builds as f64 / self.total_builds as f64
    }
}

/// 重新加载管理器
pub struct ReloadManager {
    config: ReloadConfig,
    change_detector: ChangeDetector,
    build_queue: Arc<Mutex<VecDeque<BuildTask>>>,
    is_building: Arc<Mutex<bool>>,
    stats: Arc<Mutex<BuildStats>>,
    event_handlers: Vec<Box<dyn Fn(&ReloadEvent) + Send + Sync>>,
    worker_handles: Vec<thread::JoinHandle<()>>,
    is_running: Arc<Mutex<bool>>,
}

impl ReloadManager {
    /// 创建新的重新加载管理器
    pub fn new(config: ReloadConfig) -> Self {
        Self {
            change_detector: ChangeDetector::new(),
            build_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_building: Arc::new(Mutex::new(false)),
            stats: Arc::new(Mutex::new(BuildStats::default())),
            event_handlers: Vec::new(),
            worker_handles: Vec::new(),
            is_running: Arc::new(Mutex::new(false)),
            config,
        }
    }

    /// 启动重新加载管理器
    pub fn start(&mut self) -> Result<(), ReloadManagerError> {
        if *self.is_running.lock().unwrap() {
            return Err(ReloadManagerError::AlreadyRunning);
        }

        *self.is_running.lock().unwrap() = true;

        // 启动构建工作线程
        self.start_build_workers();

        println!("🔄 重新加载管理器已启动");
        Ok(())
    }

    /// 停止重新加载管理器
    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;

        // 等待所有工作线程完成
        for handle in self.worker_handles.drain(..) {
            let _ = handle.join();
        }

        println!("🛑 重新加载管理器已停止");
    }

    /// 处理文件变更
    pub fn handle_file_changes(&mut self, files: Vec<PathBuf>) -> Result<(), ReloadManagerError> {
        if files.is_empty() {
            return Ok(());
        }

        // 分析文件变更
        let mut changes = Vec::new();
        for file in &files {
            match self.change_detector.analyze_change(file) {
                Ok(change) => changes.push(change),
                Err(e) => {
                    self.emit_event(ReloadEvent::Error {
                        message: format!("分析文件变更失败: {}", e),
                        error_type: ReloadErrorType::SystemError,
                        timestamp: SystemTime::now(),
                    });
                }
            }
        }

        if changes.is_empty() {
            return Ok(());
        }

        // 根据变更类型决定构建策略
        let build_type = self.determine_build_type(&changes);
        let priority = self.calculate_priority(&changes);

        // 创建构建任务
        let task = BuildTask::new(files, build_type, priority);

        // 添加到构建队列
        {
            let mut queue = self.build_queue.lock().unwrap();

            // 如果是热重载，移除队列中的其他热重载任务
            if task.build_type == BuildType::HotReload {
                queue.retain(|t| t.build_type != BuildType::HotReload);
            }

            // 按优先级插入
            let insert_pos = queue
                .iter()
                .position(|t| t.priority < task.priority)
                .unwrap_or(queue.len());
            queue.insert(insert_pos, task);
        }

        Ok(())
    }

    /// 手动触发重新加载
    pub fn reload_all(&mut self) -> Result<BuildResult, ReloadManagerError> {
        let task = BuildTask::new(vec![], BuildType::Full, 10);
        self.execute_build_task(&task)
    }

    /// 触发增量构建
    pub fn incremental_build(
        &mut self,
        files: Vec<PathBuf>,
    ) -> Result<BuildResult, ReloadManagerError> {
        let task = BuildTask::new(files, BuildType::Incremental, 8);
        self.execute_build_task(&task)
    }

    /// 触发热重载
    pub fn hot_reload(&mut self, files: Vec<PathBuf>) -> Result<(), ReloadManagerError> {
        // 检查是否所有文件都支持热重载
        for file in &files {
            if let Ok(change) = self.change_detector.analyze_change(file) {
                if !change.supports_hot_reload {
                    return Err(ReloadManagerError::HotReloadNotSupported(file.clone()));
                }
            }
        }

        // 执行热重载
        self.emit_event(ReloadEvent::HotReloadCompleted {
            affected_files: files,
            timestamp: SystemTime::now(),
        });

        Ok(())
    }

    /// 添加事件处理器
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&ReloadEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// 获取构建统计
    pub fn get_stats(&self) -> BuildStats {
        self.stats.lock().unwrap().clone()
    }

    /// 检查是否正在构建
    pub fn is_building(&self) -> bool {
        *self.is_building.lock().unwrap()
    }

    /// 获取队列长度
    pub fn queue_length(&self) -> usize {
        self.build_queue.lock().unwrap().len()
    }

    /// 清空构建队列
    pub fn clear_queue(&mut self) {
        self.build_queue.lock().unwrap().clear();
    }

    /// 启动构建工作线程
    fn start_build_workers(&mut self) {
        let worker_count = if self.config.enable_parallel_build {
            self.config.max_parallel_tasks.min(4) // 最多4个构建线程
        } else {
            1
        };

        for i in 0..worker_count {
            let build_queue = self.build_queue.clone();
            let is_building = self.is_building.clone();
            let is_running = self.is_running.clone();
            let stats = self.stats.clone();
            let config = self.config.clone();

            let handle = thread::spawn(move || {
                Self::build_worker_loop(i, build_queue, is_building, is_running, stats, config);
            });

            self.worker_handles.push(handle);
        }
    }

    /// 构建工作线程循环
    fn build_worker_loop(
        worker_id: usize,
        build_queue: Arc<Mutex<VecDeque<BuildTask>>>,
        is_building: Arc<Mutex<bool>>,
        is_running: Arc<Mutex<bool>>,
        stats: Arc<Mutex<BuildStats>>,
        config: ReloadConfig,
    ) {
        while *is_running.lock().unwrap() {
            // 从队列中获取任务
            let task = {
                let mut queue = build_queue.lock().unwrap();
                queue.pop_front()
            };

            if let Some(task) = task {
                // 设置构建状态
                *is_building.lock().unwrap() = true;

                println!(
                    "🔨 工作线程 {} 开始构建任务: {:?}",
                    worker_id, task.build_type
                );

                // 执行构建
                let start_time = Instant::now();
                let result = Self::execute_build(&task, &config);
                let duration = start_time.elapsed();

                // 更新统计
                {
                    let mut stats = stats.lock().unwrap();
                    stats.record_build(&task.build_type, duration, result.is_ok());
                }

                if result.is_ok() {
                    println!("✅ 构建成功 ({:.2}ms)", duration.as_millis());
                } else {
                    println!(
                        "❌ 构建失败 ({:.2}ms): {}",
                        duration.as_millis(),
                        result.unwrap_err()
                    );

                    // 如果启用重试且未达到最大重试次数
                    if config.retry_on_failure && task.retry_count < config.max_retries {
                        let mut retry_task = task.clone();
                        retry_task.retry_count += 1;

                        // 等待重试间隔
                        thread::sleep(Duration::from_millis(config.retry_interval_ms));

                        // 重新加入队列
                        build_queue.lock().unwrap().push_front(retry_task);

                        println!(
                            "🔄 重试构建任务 ({}/{})",
                            task.retry_count + 1,
                            config.max_retries
                        );
                    }
                }

                // 清除构建状态
                *is_building.lock().unwrap() = false;
            } else {
                // 没有任务，短暂休眠
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    /// 执行构建任务
    fn execute_build_task(&self, task: &BuildTask) -> Result<BuildResult, ReloadManagerError> {
        Self::execute_build(task, &self.config)
    }

    /// 执行构建
    fn execute_build(
        task: &BuildTask,
        config: &ReloadConfig,
    ) -> Result<BuildResult, ReloadManagerError> {
        let start_time = Instant::now();

        // 构建命令和参数
        let mut cmd = Command::new(&config.build_command);
        cmd.args(&config.build_args);
        cmd.current_dir(&config.working_directory);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // 设置环境变量
        for (key, value) in &config.environment_variables {
            cmd.env(key, value);
        }

        // 根据构建类型调整参数
        match task.build_type {
            BuildType::Release => {
                cmd.arg("--release");
            }
            BuildType::Test => {
                cmd.args(&["test", "--no-run"]);
            }
            BuildType::Incremental => {
                // 增量构建的特殊处理
                if !task.files.is_empty() {
                    // 这里可以添加只构建特定文件的逻辑
                }
            }
            _ => {}
        }

        // 执行命令
        let output = cmd
            .output()
            .map_err(|e| ReloadManagerError::BuildFailed(e.to_string()))?;

        let duration = start_time.elapsed();

        // 检查超时
        if duration.as_secs() > config.build_timeout_seconds {
            return Err(ReloadManagerError::BuildTimeout);
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // 解析错误和警告
        let (errors, warnings) = Self::parse_build_output(&stderr);

        Ok(BuildResult {
            success: output.status.success(),
            duration,
            stdout,
            stderr,
            exit_code: output.status.code(),
            errors,
            warnings,
        })
    }

    /// 解析构建输出
    fn parse_build_output(stderr: &str) -> (Vec<String>, Vec<String>) {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for line in stderr.lines() {
            if line.contains("error:") || line.contains("error[") {
                errors.push(line.to_string());
            } else if line.contains("warning:") || line.contains("warning[") {
                warnings.push(line.to_string());
            }
        }

        (errors, warnings)
    }

    /// 确定构建类型
    fn determine_build_type(&self, changes: &[FileChange]) -> BuildType {
        // 检查是否有关键文件变更
        let has_critical_changes = changes.iter().any(|c| c.is_critical_change());

        if has_critical_changes {
            return BuildType::Full;
        }

        // 检查是否所有变更都支持热重载
        let all_support_hot_reload = changes.iter().all(|c| c.supports_hot_reload);

        if all_support_hot_reload {
            return BuildType::HotReload;
        }

        // 检查是否支持增量构建
        let supports_incremental = changes.iter().all(|c| c.supports_incremental_update());

        if supports_incremental && self.config.enable_incremental_build {
            BuildType::Incremental
        } else {
            BuildType::Full
        }
    }

    /// 计算优先级
    fn calculate_priority(&self, changes: &[FileChange]) -> u8 {
        changes.iter().map(|c| c.priority()).max().unwrap_or(0)
    }

    /// 发送事件
    fn emit_event(&self, event: ReloadEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Drop for ReloadManager {
    fn drop(&mut self) {
        self.stop();
    }
}

/// 重新加载管理器错误
#[derive(Debug, Clone)]
pub enum ReloadManagerError {
    /// 已经在运行
    AlreadyRunning,
    /// 构建失败
    BuildFailed(String),
    /// 构建超时
    BuildTimeout,
    /// 不支持热重载
    HotReloadNotSupported(PathBuf),
    /// 配置错误
    ConfigError(String),
    /// IO错误
    IoError(String),
    /// 系统错误
    SystemError(String),
}

impl std::fmt::Display for ReloadManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReloadManagerError::AlreadyRunning => {
                write!(f, "重新加载管理器已经在运行")
            }
            ReloadManagerError::BuildFailed(msg) => {
                write!(f, "构建失败: {}", msg)
            }
            ReloadManagerError::BuildTimeout => {
                write!(f, "构建超时")
            }
            ReloadManagerError::HotReloadNotSupported(path) => {
                write!(f, "文件不支持热重载: {:?}", path)
            }
            ReloadManagerError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
            ReloadManagerError::IoError(msg) => {
                write!(f, "IO错误: {}", msg)
            }
            ReloadManagerError::SystemError(msg) => {
                write!(f, "系统错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for ReloadManagerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_reload_manager_creation() {
        let config = ReloadConfig::default();
        let manager = ReloadManager::new(config);
        assert!(!manager.is_building());
        assert_eq!(manager.queue_length(), 0);
    }

    #[test]
    fn test_build_task_creation() {
        let files = vec![PathBuf::from("test.rs")];
        let task = BuildTask::new(files.clone(), BuildType::Incremental, 5);

        assert_eq!(task.files, files);
        assert_eq!(task.build_type, BuildType::Incremental);
        assert_eq!(task.priority, 5);
        assert_eq!(task.retry_count, 0);
    }

    #[test]
    fn test_build_stats() {
        let mut stats = BuildStats::default();

        stats.record_build(&BuildType::Full, Duration::from_millis(1000), true);
        stats.record_build(&BuildType::Incremental, Duration::from_millis(500), false);

        assert_eq!(stats.total_builds, 2);
        assert_eq!(stats.successful_builds, 1);
        assert_eq!(stats.failed_builds, 1);
        assert_eq!(stats.success_rate(), 0.5);
    }

    #[test]
    fn test_build_output_parsing() {
        let stderr = r#"
error: cannot find value `undefined_var` in this scope
 --> src/main.rs:5:13
warning: unused variable: `x`
 --> src/main.rs:3:9
"#;

        let (errors, warnings) = ReloadManager::parse_build_output(stderr);

        assert_eq!(errors.len(), 1);
        assert_eq!(warnings.len(), 1);
        assert!(errors[0].contains("cannot find value"));
        assert!(warnings[0].contains("unused variable"));
    }
}
