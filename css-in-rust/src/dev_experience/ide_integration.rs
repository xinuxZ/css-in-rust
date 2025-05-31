//! IDE集成模块
//!
//! 提供与各种IDE和编辑器的集成功能

use std::collections::HashMap;
use std::fmt;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// IDE类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdeType {
    /// Visual Studio Code
    VsCode,
    /// IntelliJ IDEA
    IntelliJ,
    /// Vim/Neovim
    Vim,
    /// Emacs
    Emacs,
    /// Sublime Text
    SublimeText,
    /// Atom
    Atom,
    /// WebStorm
    WebStorm,
    /// RustRover
    RustRover,
    /// 通用LSP客户端
    Generic,
}

/// LSP消息类型
#[derive(Debug, Clone, PartialEq)]
pub enum LspMessageType {
    /// 请求
    Request,
    /// 响应
    Response,
    /// 通知
    Notification,
}

/// LSP消息
#[derive(Debug, Clone)]
pub struct LspMessage {
    /// 消息类型
    pub message_type: LspMessageType,
    /// 方法名
    pub method: String,
    /// 参数
    pub params: serde_json::Value,
    /// 请求ID（仅用于请求和响应）
    pub id: Option<serde_json::Value>,
}

/// LSP能力
#[derive(Debug, Clone)]
pub struct LspCapabilities {
    /// 文本文档同步
    pub text_document_sync: TextDocumentSyncCapability,
    /// 悬停提示
    pub hover_provider: bool,
    /// 代码补全
    pub completion_provider: Option<CompletionOptions>,
    /// 签名帮助
    pub signature_help_provider: Option<SignatureHelpOptions>,
    /// 跳转到定义
    pub definition_provider: bool,
    /// 跳转到类型定义
    pub type_definition_provider: bool,
    /// 跳转到实现
    pub implementation_provider: bool,
    /// 查找引用
    pub references_provider: bool,
    /// 文档高亮
    pub document_highlight_provider: bool,
    /// 文档符号
    pub document_symbol_provider: bool,
    /// 工作区符号
    pub workspace_symbol_provider: bool,
    /// 代码操作
    pub code_action_provider: Option<CodeActionOptions>,
    /// 代码镜头
    pub code_lens_provider: Option<CodeLensOptions>,
    /// 文档格式化
    pub document_formatting_provider: bool,
    /// 文档范围格式化
    pub document_range_formatting_provider: bool,
    /// 文档类型格式化
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    /// 重命名
    pub rename_provider: Option<RenameOptions>,
    /// 文档链接
    pub document_link_provider: Option<DocumentLinkOptions>,
    /// 颜色提供者
    pub color_provider: bool,
    /// 折叠范围
    pub folding_range_provider: bool,
    /// 执行命令
    pub execute_command_provider: Option<ExecuteCommandOptions>,
    /// 工作区编辑
    pub workspace: Option<WorkspaceServerCapabilities>,
    /// 实验性功能
    pub experimental: Option<serde_json::Value>,
}

/// 文本文档同步能力
#[derive(Debug, Clone)]
pub enum TextDocumentSyncCapability {
    /// 无同步
    None,
    /// 完整同步
    Full,
    /// 增量同步
    Incremental,
}

/// 代码补全选项
#[derive(Debug, Clone)]
pub struct CompletionOptions {
    /// 解析提供者
    pub resolve_provider: bool,
    /// 触发字符
    pub trigger_characters: Vec<String>,
    /// 所有提交字符
    pub all_commit_characters: Vec<String>,
}

/// 签名帮助选项
#[derive(Debug, Clone)]
pub struct SignatureHelpOptions {
    /// 触发字符
    pub trigger_characters: Vec<String>,
    /// 重新触发字符
    pub retrigger_characters: Vec<String>,
}

/// 代码操作选项
#[derive(Debug, Clone)]
pub struct CodeActionOptions {
    /// 代码操作类型
    pub code_action_kinds: Vec<String>,
    /// 解析提供者
    pub resolve_provider: bool,
}

/// 代码镜头选项
#[derive(Debug, Clone)]
pub struct CodeLensOptions {
    /// 解析提供者
    pub resolve_provider: bool,
}

/// 文档类型格式化选项
#[derive(Debug, Clone)]
pub struct DocumentOnTypeFormattingOptions {
    /// 第一个触发字符
    pub first_trigger_character: String,
    /// 更多触发字符
    pub more_trigger_character: Vec<String>,
}

/// 重命名选项
#[derive(Debug, Clone)]
pub struct RenameOptions {
    /// 准备提供者
    pub prepare_provider: bool,
}

/// 文档链接选项
#[derive(Debug, Clone)]
pub struct DocumentLinkOptions {
    /// 解析提供者
    pub resolve_provider: bool,
}

/// 执行命令选项
#[derive(Debug, Clone)]
pub struct ExecuteCommandOptions {
    /// 命令
    pub commands: Vec<String>,
}

/// 工作区服务器能力
#[derive(Debug, Clone)]
pub struct WorkspaceServerCapabilities {
    /// 工作区文件夹
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,
    /// 文件操作
    pub file_operations: Option<FileOperationOptions>,
}

/// 工作区文件夹服务器能力
#[derive(Debug, Clone)]
pub struct WorkspaceFoldersServerCapabilities {
    /// 支持
    pub supported: bool,
    /// 变更通知
    pub change_notifications: Option<String>,
}

/// 文件操作选项
#[derive(Debug, Clone)]
pub struct FileOperationOptions {
    /// 将创建
    pub will_create: Option<FileOperationRegistrationOptions>,
    /// 已创建
    pub did_create: Option<FileOperationRegistrationOptions>,
    /// 将重命名
    pub will_rename: Option<FileOperationRegistrationOptions>,
    /// 已重命名
    pub did_rename: Option<FileOperationRegistrationOptions>,
    /// 将删除
    pub will_delete: Option<FileOperationRegistrationOptions>,
    /// 已删除
    pub did_delete: Option<FileOperationRegistrationOptions>,
}

/// 文件操作注册选项
#[derive(Debug, Clone)]
pub struct FileOperationRegistrationOptions {
    /// 过滤器
    pub filters: Vec<FileOperationFilter>,
}

/// 文件操作过滤器
#[derive(Debug, Clone)]
pub struct FileOperationFilter {
    /// 方案
    pub scheme: Option<String>,
    /// 模式
    pub pattern: FileOperationPattern,
}

/// 文件操作模式
#[derive(Debug, Clone)]
pub struct FileOperationPattern {
    /// 全局模式
    pub glob: String,
    /// 匹配
    pub matches: Option<FileOperationPatternKind>,
    /// 选项
    pub options: Option<FileOperationPatternOptions>,
}

/// 文件操作模式类型
#[derive(Debug, Clone, PartialEq)]
pub enum FileOperationPatternKind {
    /// 文件
    File,
    /// 文件夹
    Folder,
}

/// 文件操作模式选项
#[derive(Debug, Clone)]
pub struct FileOperationPatternOptions {
    /// 忽略大小写
    pub ignore_case: bool,
}

/// IDE集成管理器
pub struct IdeIntegration {
    /// IDE类型
    ide_type: IdeType,
    /// LSP服务器状态
    server_state: Arc<Mutex<LspServerState>>,
    /// 客户端连接
    clients: Arc<Mutex<HashMap<String, ClientConnection>>>,
    /// 配置
    config: IdeConfig,
    /// 能力
    capabilities: LspCapabilities,
    /// 消息处理器
    message_handlers: HashMap<String, Box<dyn MessageHandler + Send + Sync>>,
}

/// LSP服务器状态
#[derive(Debug, Clone, PartialEq)]
pub enum LspServerState {
    /// 未启动
    NotStarted,
    /// 正在启动
    Starting,
    /// 运行中
    Running,
    /// 正在停止
    Stopping,
    /// 已停止
    Stopped,
    /// 错误
    Error(String),
}

/// 语言服务器协议
#[derive(Debug, Clone)]
pub struct LanguageServerProtocol {
    /// 协议版本
    pub version: String,
    /// 支持的方法
    pub supported_methods: Vec<String>,
    /// 服务器能力
    pub capabilities: LspCapabilities,
}

/// 客户端连接
#[derive(Debug, Clone)]
pub struct ClientConnection {
    /// 客户端ID
    pub id: String,
    /// 连接地址
    pub address: SocketAddr,
    /// 连接时间
    pub connected_at: std::time::SystemTime,
    /// 最后活动时间
    pub last_activity: std::time::SystemTime,
    /// 客户端信息
    pub client_info: Option<ClientInfo>,
}

/// 客户端信息
#[derive(Debug, Clone)]
pub struct ClientInfo {
    /// 名称
    pub name: String,
    /// 版本
    pub version: Option<String>,
}

/// IDE配置
#[derive(Debug, Clone)]
pub struct IdeConfig {
    /// 是否启用语言服务器
    pub enable_language_server: bool,
    /// 服务器端口
    pub server_port: u16,
    /// 是否启用悬停提示
    pub enable_hover: bool,
    /// 是否启用跳转到定义
    pub enable_goto_definition: bool,
    /// 是否启用重构支持
    pub enable_refactoring: bool,
    /// 工作区根目录
    pub workspace_root: Option<PathBuf>,
    /// 自定义命令
    pub custom_commands: Vec<String>,
    /// 文件监控模式
    pub file_watch_patterns: Vec<String>,
    /// 诊断延迟（毫秒）
    pub diagnostic_delay: u64,
    /// 最大诊断数量
    pub max_diagnostics: usize,
}

/// 消息处理器特征
pub trait MessageHandler {
    /// 处理消息
    fn handle_message(&self, message: &LspMessage) -> Result<Option<LspMessage>, LspError>;

    /// 获取支持的方法
    fn supported_methods(&self) -> Vec<String>;
}

/// LSP错误
#[derive(Debug, Clone)]
pub enum LspError {
    /// 解析错误
    ParseError(String),
    /// 无效请求
    InvalidRequest(String),
    /// 方法未找到
    MethodNotFound(String),
    /// 无效参数
    InvalidParams(String),
    /// 内部错误
    InternalError(String),
    /// 服务器错误
    ServerError(i32, String),
}

impl IdeIntegration {
    /// 创建新的IDE集成
    pub fn new(ide_type: IdeType, config: IdeConfig) -> Self {
        let capabilities = Self::create_default_capabilities();

        Self {
            ide_type,
            server_state: Arc::new(Mutex::new(LspServerState::NotStarted)),
            clients: Arc::new(Mutex::new(HashMap::new())),
            config,
            capabilities,
            message_handlers: HashMap::new(),
        }
    }

    /// 创建默认能力
    fn create_default_capabilities() -> LspCapabilities {
        LspCapabilities {
            text_document_sync: TextDocumentSyncCapability::Incremental,
            hover_provider: true,
            completion_provider: Some(CompletionOptions {
                resolve_provider: true,
                trigger_characters: vec![
                    ":".to_string(),
                    ";".to_string(),
                    "{".to_string(),
                    "}".to_string(),
                    " ".to_string(),
                ],
                all_commit_characters: vec![";".to_string(), "}".to_string()],
            }),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: vec!["(".to_string(), ",".to_string()],
                retrigger_characters: vec![")".to_string()],
            }),
            definition_provider: true,
            type_definition_provider: true,
            implementation_provider: true,
            references_provider: true,
            document_highlight_provider: true,
            document_symbol_provider: true,
            workspace_symbol_provider: true,
            code_action_provider: Some(CodeActionOptions {
                code_action_kinds: vec![
                    "quickfix".to_string(),
                    "refactor".to_string(),
                    "source".to_string(),
                ],
                resolve_provider: true,
            }),
            code_lens_provider: Some(CodeLensOptions {
                resolve_provider: true,
            }),
            document_formatting_provider: true,
            document_range_formatting_provider: true,
            document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                first_trigger_character: ";".to_string(),
                more_trigger_character: vec!["}".to_string()],
            }),
            rename_provider: Some(RenameOptions {
                prepare_provider: true,
            }),
            document_link_provider: Some(DocumentLinkOptions {
                resolve_provider: true,
            }),
            color_provider: true,
            folding_range_provider: true,
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: vec![
                    "css-in-rust.format".to_string(),
                    "css-in-rust.optimize".to_string(),
                    "css-in-rust.extract-component".to_string(),
                ],
            }),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: true,
                    change_notifications: Some("workspace/didChangeWorkspaceFolders".to_string()),
                }),
                file_operations: Some(FileOperationOptions {
                    will_create: Some(FileOperationRegistrationOptions {
                        filters: vec![FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.css".to_string(),
                                matches: Some(FileOperationPatternKind::File),
                                options: Some(FileOperationPatternOptions { ignore_case: true }),
                            },
                        }],
                    }),
                    did_create: None,
                    will_rename: None,
                    did_rename: None,
                    will_delete: None,
                    did_delete: None,
                }),
            }),
            experimental: None,
        }
    }

    /// 启动LSP服务器
    pub fn start_language_server(&mut self) -> Result<(), LspError> {
        if !self.config.enable_language_server {
            return Err(LspError::ServerError(-1, "语言服务器未启用".to_string()));
        }

        // 更新状态
        {
            let mut state = self.server_state.lock().unwrap();
            *state = LspServerState::Starting;
        }

        // 启动TCP服务器
        let addr = format!("127.0.0.1:{}", self.config.server_port);
        let listener = TcpListener::bind(&addr)
            .map_err(|e| LspError::ServerError(-2, format!("无法绑定地址 {}: {}", addr, e)))?;

        println!("CSS-in-Rust LSP服务器启动在 {}", addr);

        // 更新状态为运行中
        {
            let mut state = self.server_state.lock().unwrap();
            *state = LspServerState::Running;
        }

        // 在新线程中处理连接
        let clients = Arc::clone(&self.clients);
        let server_state = Arc::clone(&self.server_state);

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let client_addr = stream.peer_addr().unwrap();
                        let client_id = format!("client_{}", client_addr);

                        // 添加客户端连接
                        {
                            let mut clients_guard = clients.lock().unwrap();
                            clients_guard.insert(
                                client_id.clone(),
                                ClientConnection {
                                    id: client_id.clone(),
                                    address: client_addr,
                                    connected_at: std::time::SystemTime::now(),
                                    last_activity: std::time::SystemTime::now(),
                                    client_info: None,
                                },
                            );
                        }

                        println!("客户端连接: {}", client_addr);

                        // 在新线程中处理客户端
                        let clients_clone = Arc::clone(&clients);
                        thread::spawn(move || {
                            Self::handle_client(stream, client_id, clients_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("连接错误: {}", e);
                    }
                }

                // 检查服务器状态
                {
                    let state = server_state.lock().unwrap();
                    if *state == LspServerState::Stopping {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// 处理客户端连接
    fn handle_client(
        _stream: TcpStream,
        client_id: String,
        clients: Arc<Mutex<HashMap<String, ClientConnection>>>,
    ) {
        // 简化的客户端处理实现
        // 实际实现应该处理LSP协议消息

        // 模拟处理一段时间
        thread::sleep(Duration::from_secs(1));

        // 移除客户端连接
        {
            let mut clients_guard = clients.lock().unwrap();
            clients_guard.remove(&client_id);
        }

        println!("客户端断开连接: {}", client_id);
    }

    /// 停止LSP服务器
    pub fn stop_language_server(&mut self) -> Result<(), LspError> {
        // 更新状态
        {
            let mut state = self.server_state.lock().unwrap();
            *state = LspServerState::Stopping;
        }

        // 断开所有客户端连接
        {
            let mut clients = self.clients.lock().unwrap();
            clients.clear();
        }

        // 更新状态为已停止
        {
            let mut state = self.server_state.lock().unwrap();
            *state = LspServerState::Stopped;
        }

        println!("LSP服务器已停止");
        Ok(())
    }

    /// 获取服务器状态
    pub fn get_server_state(&self) -> LspServerState {
        self.server_state.lock().unwrap().clone()
    }

    /// 获取连接的客户端数量
    pub fn get_client_count(&self) -> usize {
        self.clients.lock().unwrap().len()
    }

    /// 获取客户端列表
    pub fn get_clients(&self) -> Vec<ClientConnection> {
        self.clients.lock().unwrap().values().cloned().collect()
    }

    /// 注册消息处理器
    pub fn register_message_handler(
        &mut self,
        method: String,
        handler: Box<dyn MessageHandler + Send + Sync>,
    ) {
        self.message_handlers.insert(method, handler);
    }

    /// 处理LSP消息
    pub fn handle_message(&self, message: &LspMessage) -> Result<Option<LspMessage>, LspError> {
        if let Some(handler) = self.message_handlers.get(&message.method) {
            handler.handle_message(message)
        } else {
            Err(LspError::MethodNotFound(format!(
                "未找到方法: {}",
                message.method
            )))
        }
    }

    /// 发送通知给所有客户端
    pub fn send_notification(&self, method: String, params: serde_json::Value) {
        let notification = LspMessage {
            message_type: LspMessageType::Notification,
            method,
            params,
            id: None,
        };

        // 简化实现：打印通知
        println!("发送通知: {:?}", notification);
    }

    /// 获取能力
    pub fn get_capabilities(&self) -> &LspCapabilities {
        &self.capabilities
    }

    /// 更新配置
    pub fn update_config(&mut self, config: IdeConfig) {
        self.config = config;
    }

    /// 获取配置
    pub fn get_config(&self) -> &IdeConfig {
        &self.config
    }

    /// 获取IDE类型
    pub fn get_ide_type(&self) -> &IdeType {
        &self.ide_type
    }

    /// 生成IDE配置文件
    pub fn generate_ide_config(&self) -> Result<String, LspError> {
        match self.ide_type {
            IdeType::VsCode => self.generate_vscode_config(),
            IdeType::IntelliJ => self.generate_intellij_config(),
            IdeType::Vim => self.generate_vim_config(),
            IdeType::Emacs => self.generate_emacs_config(),
            _ => Err(LspError::ServerError(
                -3,
                format!("不支持的IDE类型: {:?}", self.ide_type),
            )),
        }
    }

    /// 生成VS Code配置
    fn generate_vscode_config(&self) -> Result<String, LspError> {
        let config = format!(
            r#"{{
  "css-in-rust.languageServer.enable": {},
  "css-in-rust.languageServer.port": {},
  "css-in-rust.hover.enable": {},
  "css-in-rust.gotoDefinition.enable": {},
  "css-in-rust.refactoring.enable": {},
  "css-in-rust.diagnostics.delay": {},
  "css-in-rust.diagnostics.maxCount": {}
}}"#,
            self.config.enable_language_server,
            self.config.server_port,
            self.config.enable_hover,
            self.config.enable_goto_definition,
            self.config.enable_refactoring,
            self.config.diagnostic_delay,
            self.config.max_diagnostics
        );
        Ok(config)
    }

    /// 生成IntelliJ配置
    fn generate_intellij_config(&self) -> Result<String, LspError> {
        let config = format!(
            r#"<component name="CssInRustSettings">
  <option name="enableLanguageServer" value="{}" />
  <option name="serverPort" value="{}" />
  <option name="enableHover" value="{}" />
  <option name="enableGotoDefinition" value="{}" />
  <option name="enableRefactoring" value="{}" />
  <option name="diagnosticDelay" value="{}" />
  <option name="maxDiagnostics" value="{}" />
</component>"#,
            self.config.enable_language_server,
            self.config.server_port,
            self.config.enable_hover,
            self.config.enable_goto_definition,
            self.config.enable_refactoring,
            self.config.diagnostic_delay,
            self.config.max_diagnostics
        );
        Ok(config)
    }

    /// 生成Vim配置
    fn generate_vim_config(&self) -> Result<String, LspError> {
        let config = format!(
            r#"" CSS-in-Rust LSP配置
let g:css_in_rust_enable_lsp = {}
let g:css_in_rust_server_port = {}
let g:css_in_rust_enable_hover = {}
let g:css_in_rust_enable_goto_definition = {}
let g:css_in_rust_enable_refactoring = {}
let g:css_in_rust_diagnostic_delay = {}
let g:css_in_rust_max_diagnostics = {}"#,
            if self.config.enable_language_server {
                1
            } else {
                0
            },
            self.config.server_port,
            if self.config.enable_hover { 1 } else { 0 },
            if self.config.enable_goto_definition {
                1
            } else {
                0
            },
            if self.config.enable_refactoring { 1 } else { 0 },
            self.config.diagnostic_delay,
            self.config.max_diagnostics
        );
        Ok(config)
    }

    /// 生成Emacs配置
    fn generate_emacs_config(&self) -> Result<String, LspError> {
        let config = format!(
            r#";; CSS-in-Rust LSP配置
(setq css-in-rust-enable-lsp {})
(setq css-in-rust-server-port {})
(setq css-in-rust-enable-hover {})
(setq css-in-rust-enable-goto-definition {})
(setq css-in-rust-enable-refactoring {})
(setq css-in-rust-diagnostic-delay {})
(setq css-in-rust-max-diagnostics {})"#,
            if self.config.enable_language_server {
                "t"
            } else {
                "nil"
            },
            self.config.server_port,
            if self.config.enable_hover { "t" } else { "nil" },
            if self.config.enable_goto_definition {
                "t"
            } else {
                "nil"
            },
            if self.config.enable_refactoring {
                "t"
            } else {
                "nil"
            },
            self.config.diagnostic_delay,
            self.config.max_diagnostics
        );
        Ok(config)
    }

    /// 获取服务器统计信息
    pub fn get_server_stats(&self) -> ServerStats {
        let clients = self.clients.lock().unwrap();
        let total_clients = clients.len();
        let active_clients = clients
            .values()
            .filter(|client| {
                client
                    .last_activity
                    .elapsed()
                    .unwrap_or(Duration::from_secs(0))
                    < Duration::from_secs(300) // 5分钟内活跃
            })
            .count();

        ServerStats {
            state: self.get_server_state(),
            total_clients,
            active_clients,
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0)),
        }
    }
}

/// 服务器统计信息
#[derive(Debug, Clone)]
pub struct ServerStats {
    /// 服务器状态
    pub state: LspServerState,
    /// 总客户端数
    pub total_clients: usize,
    /// 活跃客户端数
    pub active_clients: usize,
    /// 运行时间
    pub uptime: Duration,
}

/// 默认消息处理器
pub struct DefaultMessageHandler;

impl MessageHandler for DefaultMessageHandler {
    fn handle_message(&self, message: &LspMessage) -> Result<Option<LspMessage>, LspError> {
        match message.method.as_str() {
            "initialize" => {
                // 处理初始化请求
                let response = LspMessage {
                    message_type: LspMessageType::Response,
                    method: "initialize".to_string(),
                    params: serde_json::json!({
                        "capabilities": {
                            "textDocumentSync": 2,
                            "hoverProvider": true,
                            "completionProvider": {
                                "resolveProvider": true,
                                "triggerCharacters": [":", ";", "{", "}", " "]
                            }
                        }
                    }),
                    id: message.id.clone(),
                };
                Ok(Some(response))
            }
            "textDocument/hover" => {
                // 处理悬停请求
                let response = LspMessage {
                    message_type: LspMessageType::Response,
                    method: "textDocument/hover".to_string(),
                    params: serde_json::json!({
                        "contents": {
                            "kind": "markdown",
                            "value": "CSS-in-Rust 悬停信息"
                        }
                    }),
                    id: message.id.clone(),
                };
                Ok(Some(response))
            }
            _ => Err(LspError::MethodNotFound(format!(
                "未实现的方法: {}",
                message.method
            ))),
        }
    }

    fn supported_methods(&self) -> Vec<String> {
        vec![
            "initialize".to_string(),
            "textDocument/hover".to_string(),
            "textDocument/completion".to_string(),
        ]
    }
}

impl Default for IdeConfig {
    fn default() -> Self {
        Self {
            enable_language_server: true,
            server_port: 9257, // CSS-in-Rust LSP默认端口
            enable_hover: true,
            enable_goto_definition: true,
            enable_refactoring: true,
            workspace_root: None,
            custom_commands: Vec::new(),
            file_watch_patterns: vec![
                "**/*.css".to_string(),
                "**/*.scss".to_string(),
                "**/*.less".to_string(),
                "**/*.rs".to_string(),
            ],
            diagnostic_delay: 500,
            max_diagnostics: 100,
        }
    }
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LspError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            LspError::InvalidRequest(msg) => write!(f, "无效请求: {}", msg),
            LspError::MethodNotFound(msg) => write!(f, "方法未找到: {}", msg),
            LspError::InvalidParams(msg) => write!(f, "无效参数: {}", msg),
            LspError::InternalError(msg) => write!(f, "内部错误: {}", msg),
            LspError::ServerError(code, msg) => write!(f, "服务器错误 {}: {}", code, msg),
        }
    }
}

impl std::error::Error for LspError {}
