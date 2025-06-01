//! 文件监控器模块
//!
//! 提供跨平台的文件系统监控功能

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

/// 监控事件类型
#[derive(Debug, Clone, PartialEq)]
pub enum WatchEventType {
    /// 文件创建
    Created,
    /// 文件修改
    Modified,
    /// 文件删除
    Deleted,
    /// 文件重命名
    Renamed { from: PathBuf, to: PathBuf },
    /// 目录创建
    DirectoryCreated,
    /// 目录删除
    DirectoryDeleted,
    /// 权限变更
    PermissionChanged,
    /// 其他事件
    Other(String),
}

/// 监控事件
#[derive(Debug, Clone)]
pub struct WatchEvent {
    /// 事件类型
    pub event_type: WatchEventType,
    /// 文件路径
    pub path: PathBuf,
    /// 事件时间戳
    pub timestamp: SystemTime,
    /// 文件大小（如果适用）
    pub file_size: Option<u64>,
    /// 是否为目录
    pub is_directory: bool,
}

impl WatchEvent {
    /// 创建新的监控事件
    pub fn new(event_type: WatchEventType, path: PathBuf) -> Self {
        let metadata = fs::metadata(&path).ok();
        let file_size = metadata.as_ref().map(|m| m.len());
        let is_directory = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        Self {
            event_type,
            path,
            timestamp: SystemTime::now(),
            file_size,
            is_directory,
        }
    }

    /// 检查是否为文件事件
    pub fn is_file_event(&self) -> bool {
        !self.is_directory
    }

    /// 检查是否为目录事件
    pub fn is_directory_event(&self) -> bool {
        self.is_directory
    }

    /// 获取文件扩展名
    pub fn file_extension(&self) -> Option<String> {
        self.path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// 检查是否为特定扩展名的文件
    pub fn has_extension(&self, extensions: &[String]) -> bool {
        if let Some(ext) = self.file_extension() {
            extensions.iter().any(|e| e.to_lowercase() == ext)
        } else {
            false
        }
    }
}

/// 文件监控器配置
#[derive(Debug, Clone)]
pub struct FileWatcherConfig {
    /// 是否递归监控子目录
    pub recursive: bool,
    /// 监控的文件扩展名过滤器
    pub extension_filter: Option<Vec<String>>,
    /// 忽略的文件模式
    pub ignore_patterns: Vec<String>,
    /// 轮询间隔（毫秒）
    pub poll_interval_ms: u64,
    /// 是否启用详细日志
    pub verbose_logging: bool,
    /// 最大监控文件数
    pub max_files: Option<usize>,
    /// 缓冲区大小
    pub buffer_size: usize,
}

impl Default for FileWatcherConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            extension_filter: None,
            ignore_patterns: vec![
                "**/.git/**".to_string(),
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/*.tmp".to_string(),
                "**/*.swp".to_string(),
                "**/.DS_Store".to_string(),
            ],
            poll_interval_ms: 100,
            verbose_logging: false,
            max_files: Some(10000),
            buffer_size: 1024,
        }
    }
}

/// 文件状态信息
#[derive(Debug, Clone)]
struct FileState {
    /// 最后修改时间
    last_modified: SystemTime,
    /// 文件大小
    size: u64,
    /// 是否为目录
    is_directory: bool,
    /// 权限
    permissions: Option<u32>,
}

impl FileState {
    /// 从文件元数据创建状态
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        Self {
            last_modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            size: metadata.len(),
            is_directory: metadata.is_dir(),
            permissions: Self::extract_permissions(metadata),
        }
    }

    /// 提取文件权限信息
    fn extract_permissions(metadata: &fs::Metadata) -> Option<u32> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            Some(mode)
        }

        #[cfg(windows)]
        {
            // Windows 下简化处理，返回基本权限信息
            let permissions = metadata.permissions();
            let mode = if permissions.readonly() { 0o444 } else { 0o666 };
            Some(mode)
        }

        #[cfg(not(any(unix, windows)))]
        {
            // 其他平台的基本权限检查
            let permissions = metadata.permissions();
            let mode = if permissions.readonly() { 0o444 } else { 0o666 };
            Some(mode)
        }
    }

    #[cfg(windows)]
    /// 检查Windows文件是否可执行
    fn is_executable_on_windows(metadata: &fs::Metadata) -> bool {
        // 在Windows上，通过文件扩展名判断是否可执行
        // 这是一个简化的实现，实际可能需要更复杂的逻辑
        false // 默认不可执行，可以根据需要扩展
    }

    /// 检查是否有变化
    fn has_changed(&self, other: &FileState) -> bool {
        self.last_modified != other.last_modified
            || self.size != other.size
            || self.is_directory != other.is_directory
    }
}

/// 文件监控器
pub struct FileWatcher {
    config: FileWatcherConfig,
    watched_directories: Vec<PathBuf>,
    file_states: Arc<Mutex<HashMap<PathBuf, FileState>>>,
    event_handler: Option<Box<dyn Fn(&WatchEvent) + Send + Sync>>,
    is_running: Arc<Mutex<bool>>,
    worker_handle: Option<thread::JoinHandle<()>>,
}

impl FileWatcher {
    /// 创建新的文件监控器
    pub fn new() -> Self {
        Self::with_config(FileWatcherConfig::default())
    }

    /// 使用配置创建文件监控器
    pub fn with_config(config: FileWatcherConfig) -> Self {
        Self {
            config,
            watched_directories: Vec::new(),
            file_states: Arc::new(Mutex::new(HashMap::new())),
            event_handler: None,
            is_running: Arc::new(Mutex::new(false)),
            worker_handle: None,
        }
    }

    /// 添加监控目录
    pub fn watch_directory(&mut self, path: PathBuf) -> Result<(), FileWatcherError> {
        if !path.exists() {
            return Err(FileWatcherError::PathNotFound(path));
        }

        if !path.is_dir() {
            return Err(FileWatcherError::NotADirectory(path));
        }

        self.watched_directories.push(path.clone());

        // 初始化文件状态
        self.scan_directory(&path)?;

        if self.config.verbose_logging {
            println!("📁 开始监控目录: {:?}", path);
        }

        Ok(())
    }

    /// 移除监控目录
    pub fn unwatch_directory(&mut self, path: &Path) {
        self.watched_directories.retain(|p| p != path);

        // 清理相关的文件状态
        {
            let mut states = self.file_states.lock().unwrap();
            states.retain(|file_path, _| !file_path.starts_with(path));
        }

        if self.config.verbose_logging {
            println!("📁 停止监控目录: {:?}", path);
        }
    }

    /// 设置事件处理器
    pub fn set_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&WatchEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
    }

    /// 启动监控
    pub fn start(&mut self) -> Result<(), FileWatcherError> {
        if *self.is_running.lock().unwrap() {
            return Err(FileWatcherError::AlreadyRunning);
        }

        *self.is_running.lock().unwrap() = true;

        let is_running = self.is_running.clone();
        let file_states = self.file_states.clone();
        let watched_directories = self.watched_directories.clone();
        let config = self.config.clone();
        let event_handler = self.event_handler.take();

        let handle = thread::spawn(move || {
            Self::watch_loop(
                is_running,
                file_states,
                watched_directories,
                config,
                event_handler,
            );
        });

        self.worker_handle = Some(handle);

        if self.config.verbose_logging {
            println!("🔍 文件监控器已启动");
        }

        Ok(())
    }

    /// 停止监控
    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;

        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }

        if self.config.verbose_logging {
            println!("🛑 文件监控器已停止");
        }
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    /// 获取监控的目录列表
    pub fn watched_directories(&self) -> &[PathBuf] {
        &self.watched_directories
    }

    /// 获取监控的文件数量
    pub fn watched_files_count(&self) -> usize {
        self.file_states.lock().unwrap().len()
    }

    /// 扫描目录并初始化文件状态
    fn scan_directory(&self, dir: &Path) -> Result<(), FileWatcherError> {
        let mut states = self.file_states.lock().unwrap();

        self.scan_directory_recursive(dir, &mut states, 0)?;

        Ok(())
    }

    /// 递归扫描目录
    fn scan_directory_recursive(
        &self,
        dir: &Path,
        states: &mut HashMap<PathBuf, FileState>,
        depth: usize,
    ) -> Result<(), FileWatcherError> {
        // 防止过深的递归
        if depth > 10 {
            return Ok(());
        }

        let entries = fs::read_dir(dir).map_err(|e| FileWatcherError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| FileWatcherError::IoError(e.to_string()))?;
            let path = entry.path();

            // 检查是否应该忽略
            if self.should_ignore(&path) {
                continue;
            }

            // 检查文件数量限制
            if let Some(max_files) = self.config.max_files {
                if states.len() >= max_files {
                    if self.config.verbose_logging {
                        println!("⚠️ 达到最大文件数限制: {}", max_files);
                    }
                    break;
                }
            }

            let metadata = entry
                .metadata()
                .map_err(|e| FileWatcherError::IoError(e.to_string()))?;

            let file_state = FileState::from_metadata(&metadata);
            states.insert(path.clone(), file_state);

            // 递归处理子目录
            if metadata.is_dir() && self.config.recursive {
                self.scan_directory_recursive(&path, states, depth + 1)?;
            }
        }

        Ok(())
    }

    /// 监控循环
    fn watch_loop(
        is_running: Arc<Mutex<bool>>,
        file_states: Arc<Mutex<HashMap<PathBuf, FileState>>>,
        watched_directories: Vec<PathBuf>,
        config: FileWatcherConfig,
        event_handler: Option<Box<dyn Fn(&WatchEvent) + Send + Sync>>,
    ) {
        let poll_interval = Duration::from_millis(config.poll_interval_ms);

        while *is_running.lock().unwrap() {
            // 检查所有监控的目录
            for dir in &watched_directories {
                if let Err(e) =
                    Self::check_directory_changes(dir, &file_states, &config, &event_handler)
                {
                    if config.verbose_logging {
                        println!("❌ 检查目录变更时出错: {:?} - {}", dir, e);
                    }
                }
            }

            thread::sleep(poll_interval);
        }
    }

    /// 检查目录变更
    fn check_directory_changes(
        dir: &Path,
        file_states: &Arc<Mutex<HashMap<PathBuf, FileState>>>,
        config: &FileWatcherConfig,
        event_handler: &Option<Box<dyn Fn(&WatchEvent) + Send + Sync>>,
    ) -> Result<(), FileWatcherError> {
        let mut current_files = HashMap::new();
        Self::collect_files_recursive(dir, &mut current_files, config, 0)?;

        let mut states = file_states.lock().unwrap();

        // 检查新文件和修改的文件
        for (path, current_state) in &current_files {
            match states.get(path) {
                Some(old_state) => {
                    // 文件存在，检查是否有变更
                    if current_state.has_changed(old_state) {
                        let event = WatchEvent::new(WatchEventType::Modified, path.clone());
                        Self::emit_event(&event, event_handler, config);
                        states.insert(path.clone(), current_state.clone());
                    }
                }
                None => {
                    // 新文件
                    let event_type = if current_state.is_directory {
                        WatchEventType::DirectoryCreated
                    } else {
                        WatchEventType::Created
                    };
                    let event = WatchEvent::new(event_type, path.clone());
                    Self::emit_event(&event, event_handler, config);
                    states.insert(path.clone(), current_state.clone());
                }
            }
        }

        // 检查删除的文件
        let mut deleted_files = Vec::new();
        for (path, old_state) in states.iter() {
            if path.starts_with(dir) && !current_files.contains_key(path) {
                let event_type = if old_state.is_directory {
                    WatchEventType::DirectoryDeleted
                } else {
                    WatchEventType::Deleted
                };
                let event = WatchEvent::new(event_type, path.clone());
                Self::emit_event(&event, event_handler, config);
                deleted_files.push(path.clone());
            }
        }

        // 移除删除的文件状态
        for path in deleted_files {
            states.remove(&path);
        }

        Ok(())
    }

    /// 递归收集文件
    fn collect_files_recursive(
        dir: &Path,
        files: &mut HashMap<PathBuf, FileState>,
        config: &FileWatcherConfig,
        depth: usize,
    ) -> Result<(), FileWatcherError> {
        if depth > 10 {
            return Ok(());
        }

        let entries = fs::read_dir(dir).map_err(|e| FileWatcherError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| FileWatcherError::IoError(e.to_string()))?;
            let path = entry.path();

            if Self::should_ignore_path(&path, &config.ignore_patterns) {
                continue;
            }

            let metadata = entry
                .metadata()
                .map_err(|e| FileWatcherError::IoError(e.to_string()))?;

            let file_state = FileState::from_metadata(&metadata);
            files.insert(path.clone(), file_state);

            if metadata.is_dir() && config.recursive {
                Self::collect_files_recursive(&path, files, config, depth + 1)?;
            }
        }

        Ok(())
    }

    /// 发送事件
    fn emit_event(
        event: &WatchEvent,
        event_handler: &Option<Box<dyn Fn(&WatchEvent) + Send + Sync>>,
        config: &FileWatcherConfig,
    ) {
        // 检查扩展名过滤器
        if let Some(extensions) = &config.extension_filter {
            if !event.has_extension(extensions) {
                return;
            }
        }

        if let Some(handler) = event_handler {
            handler(event);
        }

        if config.verbose_logging {
            println!("📄 文件事件: {:?} - {:?}", event.event_type, event.path);
        }
    }

    /// 检查是否应该忽略文件
    fn should_ignore(&self, path: &Path) -> bool {
        Self::should_ignore_path(path, &self.config.ignore_patterns)
    }

    /// 检查路径是否应该被忽略
    fn should_ignore_path(path: &Path, ignore_patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in ignore_patterns {
            if Self::matches_glob_pattern(&path_str, pattern) {
                return true;
            }
        }

        false
    }

    /// 简单的glob模式匹配
    fn matches_glob_pattern(path: &str, pattern: &str) -> bool {
        if pattern.contains("**") {
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return path.contains(prefix) && path.ends_with(suffix);
            }
        }

        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return path.starts_with(parts[0]) && path.ends_with(parts[1]);
            }
        }

        path.contains(pattern)
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

/// 文件监控器错误
#[derive(Debug, Clone)]
pub enum FileWatcherError {
    /// 路径不存在
    PathNotFound(PathBuf),
    /// 不是目录
    NotADirectory(PathBuf),
    /// 已经在运行
    AlreadyRunning,
    /// IO错误
    IoError(String),
    /// 权限错误
    PermissionDenied(PathBuf),
    /// 配置错误
    ConfigError(String),
    /// 系统错误
    SystemError(String),
}

impl std::fmt::Display for FileWatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileWatcherError::PathNotFound(path) => {
                write!(f, "路径不存在: {:?}", path)
            }
            FileWatcherError::NotADirectory(path) => {
                write!(f, "不是目录: {:?}", path)
            }
            FileWatcherError::AlreadyRunning => {
                write!(f, "文件监控器已经在运行")
            }
            FileWatcherError::IoError(msg) => {
                write!(f, "IO错误: {}", msg)
            }
            FileWatcherError::PermissionDenied(path) => {
                write!(f, "权限被拒绝: {:?}", path)
            }
            FileWatcherError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
            FileWatcherError::SystemError(msg) => {
                write!(f, "系统错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for FileWatcherError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_file_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(!watcher.is_running());
        assert_eq!(watcher.watched_directories().len(), 0);
    }

    #[test]
    fn test_watch_directory() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new();

        let result = watcher.watch_directory(temp_dir.path().to_path_buf());
        assert!(result.is_ok());
        assert_eq!(watcher.watched_directories().len(), 1);
    }

    #[test]
    fn test_watch_nonexistent_directory() {
        let mut watcher = FileWatcher::new();
        let result = watcher.watch_directory(PathBuf::from("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_glob_pattern_matching() {
        assert!(FileWatcher::matches_glob_pattern("src/main.rs", "**/*.rs"));
        assert!(FileWatcher::matches_glob_pattern(
            "target/debug/app",
            "target/**"
        ));
        assert!(!FileWatcher::matches_glob_pattern("src/main.rs", "**/*.js"));
        assert!(FileWatcher::matches_glob_pattern("file.tmp", "*.tmp"));
    }

    #[test]
    fn test_watch_event_creation() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let event = WatchEvent::new(WatchEventType::Created, file_path.clone());
        assert_eq!(event.path, file_path);
        assert_eq!(event.event_type, WatchEventType::Created);
        assert!(!event.is_directory);
    }

    #[test]
    fn test_file_extension_detection() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        File::create(&file_path).unwrap();

        let event = WatchEvent::new(WatchEventType::Created, file_path);
        assert_eq!(event.file_extension(), Some("rs".to_string()));
        assert!(event.has_extension(&["rs".to_string(), "js".to_string()]));
        assert!(!event.has_extension(&["js".to_string(), "ts".to_string()]));
    }
}
