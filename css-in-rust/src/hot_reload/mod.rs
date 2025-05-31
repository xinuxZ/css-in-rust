//! 热更新模块
//!
//! 提供文件监控和自动重新编译功能，支持开发时的实时更新

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub mod change_detector;
pub mod file_watcher;
pub mod reload_manager;
pub mod websocket_server;

// 重新导出主要类型
pub use change_detector::{ChangeDetector, ChangeType, FileChange};
pub use file_watcher::{FileWatcher, FileWatcherConfig, WatchEvent, WatchEventType};
pub use reload_manager::{
    BuildResult as ReloadBuildResult, BuildType, ReloadConfig, ReloadEvent, ReloadManager,
};
pub use websocket_server::{
    BuildStatus, LogLevel, WebSocketConfig, WebSocketMessage, WebSocketServer,
};

/// 热更新配置
#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    /// 是否启用热更新
    pub enabled: bool,
    /// 监控的目录列表
    pub watch_directories: Vec<PathBuf>,
    /// 监控的文件扩展名
    pub watch_extensions: Vec<String>,
    /// 忽略的文件模式
    pub ignore_patterns: Vec<String>,
    /// 防抖延迟（毫秒）
    pub debounce_delay_ms: u64,
    /// WebSocket服务器端口
    pub websocket_port: u16,
    /// 是否启用浏览器自动刷新
    pub auto_refresh_browser: bool,
    /// 是否启用CSS注入（无需刷新页面）
    pub enable_css_injection: bool,
    /// 最大重试次数
    pub max_retries: usize,
    /// 重试间隔（毫秒）
    pub retry_interval_ms: u64,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_directories: vec![PathBuf::from("src"), PathBuf::from("assets")],
            watch_extensions: vec![
                "rs".to_string(),
                "css".to_string(),
                "scss".to_string(),
                "less".to_string(),
                "html".to_string(),
                "js".to_string(),
                "ts".to_string(),
            ],
            ignore_patterns: vec![
                "target/**".to_string(),
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                "**/*.tmp".to_string(),
                "**/*.swp".to_string(),
                "**/.DS_Store".to_string(),
            ],
            debounce_delay_ms: 300,
            websocket_port: 3001,
            auto_refresh_browser: true,
            enable_css_injection: true,
            max_retries: 3,
            retry_interval_ms: 1000,
        }
    }
}

/// 热更新状态
#[derive(Debug, Clone, PartialEq)]
pub enum HotReloadStatus {
    /// 未启动
    Stopped,
    /// 启动中
    Starting,
    /// 运行中
    Running,
    /// 暂停
    Paused,
    /// 错误
    Error(String),
}

/// 热更新事件
#[derive(Debug, Clone)]
pub enum HotReloadEvent {
    /// 文件变更
    FileChanged {
        path: PathBuf,
        change_type: ChangeType,
        timestamp: SystemTime,
    },
    /// 编译开始
    CompilationStarted {
        files: Vec<PathBuf>,
        timestamp: SystemTime,
    },
    /// 编译完成
    CompilationCompleted {
        success: bool,
        duration: Duration,
        errors: Vec<String>,
        timestamp: SystemTime,
    },
    /// 浏览器刷新
    BrowserRefresh { timestamp: SystemTime },
    /// CSS注入
    CssInjection {
        css_content: String,
        timestamp: SystemTime,
    },
    /// 错误发生
    Error {
        message: String,
        timestamp: SystemTime,
    },
}

/// 热更新统计
#[derive(Debug, Clone, Default)]
pub struct HotReloadStats {
    /// 总文件变更数
    pub total_file_changes: usize,
    /// 总编译次数
    pub total_compilations: usize,
    /// 成功编译次数
    pub successful_compilations: usize,
    /// 失败编译次数
    pub failed_compilations: usize,
    /// 总浏览器刷新次数
    pub total_browser_refreshes: usize,
    /// 总CSS注入次数
    pub total_css_injections: usize,
    /// 平均编译时间
    pub average_compilation_time: Duration,
    /// 最后更新时间
    pub last_update_time: Option<SystemTime>,
    /// 启动时间
    pub start_time: Option<SystemTime>,
}

impl HotReloadStats {
    /// 创建新的统计
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录文件变更
    pub fn record_file_change(&mut self) {
        self.total_file_changes += 1;
        self.last_update_time = Some(SystemTime::now());
    }

    /// 记录编译开始
    pub fn record_compilation_start(&mut self) {
        self.total_compilations += 1;
    }

    /// 记录编译完成
    pub fn record_compilation_complete(&mut self, success: bool, duration: Duration) {
        if success {
            self.successful_compilations += 1;
        } else {
            self.failed_compilations += 1;
        }

        // 更新平均编译时间
        let total_time = self.average_compilation_time * self.total_compilations as u32 + duration;
        self.average_compilation_time = total_time / (self.total_compilations as u32);

        self.last_update_time = Some(SystemTime::now());
    }

    /// 记录浏览器刷新
    pub fn record_browser_refresh(&mut self) {
        self.total_browser_refreshes += 1;
        self.last_update_time = Some(SystemTime::now());
    }

    /// 记录CSS注入
    pub fn record_css_injection(&mut self) {
        self.total_css_injections += 1;
        self.last_update_time = Some(SystemTime::now());
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_compilations == 0 {
            return 0.0;
        }
        self.successful_compilations as f64 / self.total_compilations as f64
    }

    /// 获取运行时间
    pub fn uptime(&self) -> Option<Duration> {
        self.start_time
            .map(|start| SystemTime::now().duration_since(start).unwrap_or_default())
    }
}

/// 热更新管理器
pub struct HotReloadManager {
    config: HotReloadConfig,
    status: Arc<Mutex<HotReloadStatus>>,
    file_watcher: Option<FileWatcher>,
    change_detector: ChangeDetector,
    reload_manager: ReloadManager,
    websocket_server: Option<WebSocketServer>,
    stats: Arc<Mutex<HotReloadStats>>,
    event_handlers: Vec<Box<dyn Fn(&HotReloadEvent) + Send + Sync>>,
    pending_changes: Arc<Mutex<HashMap<PathBuf, Instant>>>,
}

impl HotReloadManager {
    /// 创建新的热更新管理器
    pub fn new(config: HotReloadConfig) -> Self {
        let mut stats = HotReloadStats::new();
        stats.start_time = Some(SystemTime::now());

        Self {
            change_detector: ChangeDetector::new(),
            reload_manager: ReloadManager::new(ReloadConfig::default()),
            file_watcher: None,
            websocket_server: None,
            status: Arc::new(Mutex::new(HotReloadStatus::Stopped)),
            stats: Arc::new(Mutex::new(stats)),
            event_handlers: Vec::new(),
            pending_changes: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// 启动热更新
    pub fn start(&mut self) -> Result<(), HotReloadError> {
        if !self.config.enabled {
            return Err(HotReloadError::Disabled);
        }

        *self.status.lock().unwrap() = HotReloadStatus::Starting;

        // 启动文件监控
        let mut file_watcher = FileWatcher::new();
        for dir in &self.config.watch_directories {
            file_watcher
                .watch_directory(dir.clone())
                .map_err(|e| HotReloadError::FileWatchFailed(e.to_string()))?;
        }

        // 设置文件变更回调
        let status = self.status.clone();
        let stats = self.stats.clone();
        let pending_changes = self.pending_changes.clone();
        let debounce_delay = Duration::from_millis(self.config.debounce_delay_ms);

        file_watcher.set_event_handler(Box::new(move |event: &WatchEvent| {
            Self::handle_file_event(&*event, &status, &stats, &pending_changes, debounce_delay);
        }));

        self.file_watcher = Some(file_watcher);

        // 启动WebSocket服务器
        if self.config.auto_refresh_browser || self.config.enable_css_injection {
            let mut websocket_server = WebSocketServer::new(WebSocketConfig::default());
            websocket_server.start();
            self.websocket_server = Some(websocket_server);
        }

        // 启动防抖处理线程
        self.start_debounce_processor();

        *self.status.lock().unwrap() = HotReloadStatus::Running;

        println!("🔥 热更新已启动，监控端口: {}", self.config.websocket_port);
        self.emit_event(HotReloadEvent::CompilationStarted {
            files: vec![],
            timestamp: SystemTime::now(),
        });

        Ok(())
    }

    /// 停止热更新
    pub fn stop(&mut self) {
        *self.status.lock().unwrap() = HotReloadStatus::Stopped;

        if let Some(mut watcher) = self.file_watcher.take() {
            watcher.stop();
        }

        if let Some(mut server) = self.websocket_server.take() {
            server.stop();
        }

        println!("🛑 热更新已停止");
    }

    /// 暂停热更新
    pub fn pause(&mut self) {
        *self.status.lock().unwrap() = HotReloadStatus::Paused;
        println!("⏸️ 热更新已暂停");
    }

    /// 恢复热更新
    pub fn resume(&mut self) {
        *self.status.lock().unwrap() = HotReloadStatus::Running;
        println!("▶️ 热更新已恢复");
    }

    /// 手动触发重新加载
    pub fn trigger_reload(&mut self) -> Result<(), HotReloadError> {
        let start_time = Instant::now();

        self.stats.lock().unwrap().record_compilation_start();

        // 执行重新编译
        let result = self.reload_manager.reload_all();

        let duration = start_time.elapsed();
        let success = result.is_ok();

        self.stats
            .lock()
            .unwrap()
            .record_compilation_complete(success, duration);

        match result {
            Ok(_) => {
                self.emit_event(HotReloadEvent::CompilationCompleted {
                    success: true,
                    duration,
                    errors: vec![],
                    timestamp: SystemTime::now(),
                });

                // 通知浏览器刷新
                if self.config.auto_refresh_browser {
                    self.refresh_browser();
                }

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                self.emit_event(HotReloadEvent::CompilationCompleted {
                    success: false,
                    duration,
                    errors: vec![error_msg.clone()],
                    timestamp: SystemTime::now(),
                });

                Err(HotReloadError::CompilationFailed(error_msg))
            }
        }
    }

    /// 注入CSS
    pub fn inject_css(&mut self, css_content: String) -> Result<(), HotReloadError> {
        if !self.config.enable_css_injection {
            return Err(HotReloadError::CssInjectionDisabled);
        }

        if let Some(server) = &mut self.websocket_server {
            server
                .broadcast(WebSocketMessage::CssHotReload {
                    files: vec!["gloable.css".to_string()],
                    css_content: css_content.clone(),
                    timestamp: SystemTime::now(),
                })
                .map_err(|e| HotReloadError::NetworkError(e.to_string()));

            self.stats.lock().unwrap().record_css_injection();

            self.emit_event(HotReloadEvent::CssInjection {
                css_content,
                timestamp: SystemTime::now(),
            });
        }

        Ok(())
    }

    /// 刷新浏览器
    pub fn refresh_browser(&mut self) {
        if let Some(server) = &mut self.websocket_server {
            let _ = server.broadcast(WebSocketMessage::FullReload {
                reason: "refresh_browser 方法调用导致的刷新".to_string(),
                timestamp: SystemTime::now(),
            });

            self.stats.lock().unwrap().record_browser_refresh();

            self.emit_event(HotReloadEvent::BrowserRefresh {
                timestamp: SystemTime::now(),
            });
        }
    }

    /// 添加事件处理器
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&HotReloadEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// 获取状态
    pub fn get_status(&self) -> HotReloadStatus {
        self.status.lock().unwrap().clone()
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> HotReloadStats {
        self.stats.lock().unwrap().clone()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: HotReloadConfig) {
        self.config = config;
        // 如果正在运行，重新启动以应用新配置
        if matches!(self.get_status(), HotReloadStatus::Running) {
            self.stop();
            let _ = self.start();
        }
    }

    /// 处理文件事件
    fn handle_file_event(
        event: &WatchEvent,
        status: &Arc<Mutex<HotReloadStatus>>,
        stats: &Arc<Mutex<HotReloadStats>>,
        pending_changes: &Arc<Mutex<HashMap<PathBuf, Instant>>>,
        debounce_delay: Duration,
    ) {
        // 检查是否暂停
        if matches!(*status.lock().unwrap(), HotReloadStatus::Paused) {
            return;
        }

        // 记录待处理的变更
        {
            let mut pending = pending_changes.lock().unwrap();
            pending.insert(event.path.clone(), Instant::now());
        }

        stats.lock().unwrap().record_file_change();
    }

    /// 启动防抖处理器
    fn start_debounce_processor(&self) {
        let pending_changes = self.pending_changes.clone();
        let status = self.status.clone();
        let debounce_delay = Duration::from_millis(self.config.debounce_delay_ms);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(100));

                // 检查是否停止
                if matches!(*status.lock().unwrap(), HotReloadStatus::Stopped) {
                    break;
                }

                let now = Instant::now();
                let mut changes_to_process = Vec::new();

                {
                    let mut pending = pending_changes.lock().unwrap();
                    pending.retain(|path, timestamp| {
                        if now.duration_since(*timestamp) >= debounce_delay {
                            changes_to_process.push(path.clone());
                            false
                        } else {
                            true
                        }
                    });
                }

                if !changes_to_process.is_empty() {
                    // 这里应该触发重新编译
                    // 由于我们在静态方法中，需要通过其他方式通知主线程
                    println!("🔄 检测到文件变更，准备重新编译: {:?}", changes_to_process);
                }
            }
        });
    }

    /// 发送事件
    fn emit_event(&self, event: HotReloadEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }

    /// 检查文件是否应该被监控
    pub fn should_watch_file(&self, path: &Path) -> bool {
        // 检查扩展名
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if !self.config.watch_extensions.contains(&ext_str.to_string()) {
                    return false;
                }
            }
        }

        // 检查忽略模式
        let path_str = path.to_string_lossy();
        for pattern in &self.config.ignore_patterns {
            if Self::matches_pattern(&path_str, pattern) {
                return false;
            }
        }

        true
    }

    /// 检查路径是否匹配模式
    fn matches_pattern(path: &str, pattern: &str) -> bool {
        // 简单的glob模式匹配
        if pattern.contains("**") {
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return path.starts_with(prefix) && path.ends_with(suffix);
            }
        }

        if pattern.contains('*') {
            // 简单的通配符匹配
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return path.starts_with(parts[0]) && path.ends_with(parts[1]);
            }
        }

        path == pattern
    }
}

/// 热更新错误
#[derive(Debug, Clone)]
pub enum HotReloadError {
    /// 热更新被禁用
    Disabled,
    /// 文件监控失败
    FileWatchFailed(String),
    /// WebSocket服务器启动失败
    WebSocketServerFailed(String),
    /// 编译失败
    CompilationFailed(String),
    /// CSS注入被禁用
    CssInjectionDisabled,
    /// 网络错误
    NetworkError(String),
    /// 配置错误
    ConfigError(String),
    /// IO错误
    IoError(String),
}

impl std::fmt::Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotReloadError::Disabled => write!(f, "热更新被禁用"),
            HotReloadError::FileWatchFailed(msg) => write!(f, "文件监控失败: {}", msg),
            HotReloadError::WebSocketServerFailed(msg) => {
                write!(f, "WebSocket服务器启动失败: {}", msg)
            }
            HotReloadError::CompilationFailed(msg) => write!(f, "编译失败: {}", msg),
            HotReloadError::CssInjectionDisabled => write!(f, "CSS注入被禁用"),
            HotReloadError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            HotReloadError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            HotReloadError::IoError(msg) => write!(f, "IO错误: {}", msg),
        }
    }
}

impl std::error::Error for HotReloadError {}

/// 热更新工具
pub struct HotReloadTools {
    manager: HotReloadManager,
}

impl HotReloadTools {
    /// 创建新的热更新工具
    pub fn new(config: HotReloadConfig) -> Self {
        Self {
            manager: HotReloadManager::new(config),
        }
    }

    /// 启动开发服务器
    pub fn start_dev_server(&mut self) -> Result<(), HotReloadError> {
        println!("🚀 启动开发服务器...");

        // 添加默认事件处理器
        self.manager.add_event_handler(|event| match event {
            HotReloadEvent::FileChanged {
                path, change_type, ..
            } => {
                println!("📁 文件变更: {:?} ({:?})", path, change_type);
            }
            HotReloadEvent::CompilationStarted { files, .. } => {
                if !files.is_empty() {
                    println!("🔨 开始编译: {} 个文件", files.len());
                }
            }
            HotReloadEvent::CompilationCompleted {
                success,
                duration,
                errors,
                ..
            } => {
                if *success {
                    println!("✅ 编译完成 ({:.2}ms)", duration.as_millis());
                } else {
                    println!("❌ 编译失败 ({:.2}ms)", duration.as_millis());
                    for error in errors {
                        println!("   错误: {}", error);
                    }
                }
            }
            HotReloadEvent::BrowserRefresh { .. } => {
                println!("🔄 浏览器已刷新");
            }
            HotReloadEvent::CssInjection { .. } => {
                println!("💉 CSS已注入");
            }
            HotReloadEvent::Error { message, .. } => {
                println!("❌ 错误: {}", message);
            }
        });

        self.manager.start()?;

        println!("✅ 开发服务器已启动");
        println!("   - 文件监控: 已启用");
        println!("   - 热更新: 已启用");
        println!("   - WebSocket端口: {}", self.manager.config.websocket_port);

        Ok(())
    }

    /// 停止开发服务器
    pub fn stop_dev_server(&mut self) {
        self.manager.stop();
        println!("✅ 开发服务器已停止");
    }

    /// 获取状态信息
    pub fn get_status_info(&self) -> String {
        let status = self.manager.get_status();
        let stats = self.manager.get_stats();

        format!(
            r#"
📊 热更新状态信息

状态: {:?}
文件变更: {} 次
编译次数: {} 次
成功率: {:.1}%
平均编译时间: {:.2}ms
浏览器刷新: {} 次
CSS注入: {} 次
运行时间: {}
"#,
            status,
            stats.total_file_changes,
            stats.total_compilations,
            stats.success_rate() * 100.0,
            stats.average_compilation_time.as_millis(),
            stats.total_browser_refreshes,
            stats.total_css_injections,
            stats
                .uptime()
                .map(|d| format!("{:.1}s", d.as_secs_f64()))
                .unwrap_or_else(|| "未知".to_string())
        )
    }

    /// 获取帮助信息
    pub fn get_help() -> String {
        r#"
🔥 CSS-in-Rust 热更新工具

功能:
  • 实时文件监控
  • 自动重新编译
  • 浏览器自动刷新
  • CSS热注入（无需刷新页面）
  • WebSocket通信
  • 防抖处理
  • 错误报告

使用方法:
  1. 启动开发服务器: HotReloadTools::start_dev_server()
  2. 修改CSS文件，自动触发重新编译
  3. 浏览器自动刷新或CSS热注入
  4. 查看状态: get_status_info()

配置选项:
  • watch_directories: 监控目录
  • watch_extensions: 监控文件扩展名
  • ignore_patterns: 忽略文件模式
  • debounce_delay_ms: 防抖延迟
  • websocket_port: WebSocket端口
  • auto_refresh_browser: 自动刷新浏览器
  • enable_css_injection: 启用CSS注入

更多信息请查看文档。
"#
        .to_string()
    }
}
