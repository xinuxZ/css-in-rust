//! WebSocket 服务器模块
//!
//! 用于实现浏览器与开发服务器的实时通信

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

/// WebSocket 配置
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// 服务器地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 最大连接数
    pub max_connections: usize,
    /// 心跳间隔（秒）
    pub heartbeat_interval: u64,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 缓冲区大小
    pub buffer_size: usize,
    /// 是否启用详细日志
    pub verbose_logging: bool,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3001,
            max_connections: 100,
            heartbeat_interval: 30,
            connection_timeout: 300,
            enable_compression: true,
            buffer_size: 8192,
            verbose_logging: false,
        }
    }
}

/// WebSocket 消息类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    /// 连接建立
    Connected { client_id: String, timestamp: u64 },
    /// 心跳
    Ping { timestamp: u64 },
    /// 心跳响应
    Pong { timestamp: u64 },
    /// CSS 热重载
    CssHotReload {
        files: Vec<String>,
        css_content: String,
        timestamp: SystemTime,
    },
    /// JavaScript 重新加载
    JsReload { files: Vec<String>, timestamp: u64 },
    /// 页面完全重新加载
    FullReload {
        reason: String,
        timestamp: SystemTime,
    },
    /// 构建状态更新
    BuildStatus {
        status: BuildStatus,
        message: String,
        timestamp: u64,
    },
    /// 错误消息
    Error {
        message: String,
        error_type: String,
        timestamp: u64,
    },
    /// 日志消息
    Log {
        level: LogLevel,
        message: String,
        timestamp: u64,
    },
    /// 客户端信息
    ClientInfo {
        user_agent: String,
        url: String,
        timestamp: u64,
    },
    /// 断开连接
    Disconnect {
        client_id: String,
        reason: String,
        timestamp: u64,
    },
}

/// 构建状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BuildStatus {
    /// 开始构建
    Building,
    /// 构建成功
    Success,
    /// 构建失败
    Failed,
    /// 构建警告
    Warning,
}

/// 日志级别
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    /// 调试
    Debug,
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
}

/// 客户端连接信息
#[derive(Debug, Clone)]
struct ClientConnection {
    /// 客户端ID
    id: String,
    /// TCP流
    stream: Arc<Mutex<TcpStream>>,
    /// 连接时间
    connected_at: SystemTime,
    /// 最后活动时间
    last_activity: SystemTime,
    /// 用户代理
    user_agent: Option<String>,
    /// 当前URL
    current_url: Option<String>,
    /// 是否已认证
    authenticated: bool,
}

impl ClientConnection {
    fn new(id: String, stream: TcpStream) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            stream: Arc::new(Mutex::new(stream)),
            connected_at: now,
            last_activity: now,
            user_agent: None,
            current_url: None,
            authenticated: false,
        }
    }

    fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
    }

    fn is_expired(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed().unwrap_or(Duration::ZERO) > timeout
    }

    fn send_message(&self, message: &WebSocketMessage) -> Result<(), WebSocketError> {
        let json = serde_json::to_string(message)
            .map_err(|e| WebSocketError::SerializationError(e.to_string()))?;

        let frame = Self::create_websocket_frame(&json);

        let mut stream = self.stream.lock().unwrap();
        stream
            .write_all(&frame)
            .map_err(|e| WebSocketError::SendError(e.to_string()))?;
        stream
            .flush()
            .map_err(|e| WebSocketError::SendError(e.to_string()))?;

        Ok(())
    }

    fn create_websocket_frame(payload: &str) -> Vec<u8> {
        let payload_bytes = payload.as_bytes();
        let payload_len = payload_bytes.len();

        let mut frame = Vec::new();

        // FIN=1, RSV=000, Opcode=0001 (text frame)
        frame.push(0x81);

        // Payload length
        if payload_len < 126 {
            frame.push(payload_len as u8);
        } else if payload_len < 65536 {
            frame.push(126);
            frame.extend_from_slice(&(payload_len as u16).to_be_bytes());
        } else {
            frame.push(127);
            frame.extend_from_slice(&(payload_len as u64).to_be_bytes());
        }

        // Payload
        frame.extend_from_slice(payload_bytes);

        frame
    }
}

/// WebSocket 服务器统计
#[derive(Debug, Clone, Default)]
pub struct WebSocketStats {
    /// 总连接数
    pub total_connections: usize,
    /// 当前连接数
    pub active_connections: usize,
    /// 发送的消息数
    pub messages_sent: usize,
    /// 接收的消息数
    pub messages_received: usize,
    /// 错误数
    pub errors: usize,
    /// 服务器启动时间
    pub server_start_time: Option<SystemTime>,
    /// 平均连接时长
    pub average_connection_duration: Duration,
}

impl WebSocketStats {
    fn record_connection(&mut self) {
        self.total_connections += 1;
        self.active_connections += 1;
    }

    fn record_disconnection(&mut self, duration: Duration) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }

        // 更新平均连接时长
        let total_duration =
            self.average_connection_duration * self.total_connections as u32 + duration;
        self.average_connection_duration = total_duration / self.total_connections as u32;
    }

    fn record_message_sent(&mut self) {
        self.messages_sent += 1;
    }

    fn record_message_received(&mut self) {
        self.messages_received += 1;
    }

    fn record_error(&mut self) {
        self.errors += 1;
    }
}

/// WebSocket 服务器
pub struct WebSocketServer {
    config: WebSocketConfig,
    clients: Arc<Mutex<HashMap<String, ClientConnection>>>,
    stats: Arc<Mutex<WebSocketStats>>,
    is_running: Arc<Mutex<bool>>,
    listener: Option<TcpListener>,
    worker_handles: Vec<thread::JoinHandle<()>>,
}

impl WebSocketServer {
    /// 创建新的 WebSocket 服务器
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            config,
            clients: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(WebSocketStats::default())),
            is_running: Arc::new(Mutex::new(false)),
            listener: None,
            worker_handles: Vec::new(),
        }
    }

    /// 启动服务器
    pub fn start(&mut self) -> Result<(), WebSocketError> {
        if *self.is_running.lock().unwrap() {
            return Err(WebSocketError::ServerAlreadyRunning);
        }

        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener =
            TcpListener::bind(&addr).map_err(|e| WebSocketError::BindError(e.to_string()))?;

        listener
            .set_nonblocking(true)
            .map_err(|e| WebSocketError::ConfigError(e.to_string()))?;

        self.listener = Some(listener);
        *self.is_running.lock().unwrap() = true;

        // 记录服务器启动时间
        {
            let mut stats = self.stats.lock().unwrap();
            stats.server_start_time = Some(SystemTime::now());
        }

        // 启动连接处理线程
        self.start_connection_handler();

        // 启动心跳线程
        self.start_heartbeat_worker();

        // 启动清理线程
        self.start_cleanup_worker();

        println!("🌐 WebSocket 服务器已启动: {}", addr);
        Ok(())
    }

    /// 停止服务器
    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;

        // 断开所有客户端连接
        {
            let mut clients = self.clients.lock().unwrap();
            for (client_id, client) in clients.drain() {
                let _ = client.send_message(&WebSocketMessage::Disconnect {
                    client_id: client_id.clone(),
                    reason: "服务器关闭".to_string(),
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                });
            }
        }

        // 等待所有工作线程完成
        for handle in self.worker_handles.drain(..) {
            let _ = handle.join();
        }

        self.listener = None;
        println!("🛑 WebSocket 服务器已停止");
    }

    /// 广播消息给所有客户端
    pub fn broadcast(&self, message: WebSocketMessage) -> Result<usize, WebSocketError> {
        let clients = self.clients.lock().unwrap();
        let mut sent_count = 0;
        let mut errors = 0;

        for client in clients.values() {
            match client.send_message(&message) {
                Ok(_) => {
                    sent_count += 1;
                    self.stats.lock().unwrap().record_message_sent();
                }
                Err(e) => {
                    errors += 1;
                    self.stats.lock().unwrap().record_error();
                    if self.config.verbose_logging {
                        println!("❌ 发送消息失败: {}", e);
                    }
                }
            }
        }

        if errors > 0 && self.config.verbose_logging {
            println!("⚠️ 广播消息时发生 {} 个错误", errors);
        }

        Ok(sent_count)
    }

    /// 发送消息给特定客户端
    pub fn send_to_client(
        &self,
        client_id: &str,
        message: WebSocketMessage,
    ) -> Result<(), WebSocketError> {
        let clients = self.clients.lock().unwrap();

        if let Some(client) = clients.get(client_id) {
            client.send_message(&message)?;
            self.stats.lock().unwrap().record_message_sent();
            Ok(())
        } else {
            Err(WebSocketError::ClientNotFound(client_id.to_string()))
        }
    }

    /// 获取连接的客户端列表
    pub fn get_connected_clients(&self) -> Vec<String> {
        self.clients.lock().unwrap().keys().cloned().collect()
    }

    /// 获取服务器统计信息
    pub fn get_stats(&self) -> WebSocketStats {
        self.stats.lock().unwrap().clone()
    }

    /// 检查服务器是否正在运行
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    /// 获取服务器地址
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    /// 启动连接处理线程
    fn start_connection_handler(&mut self) {
        let listener = self.listener.as_ref().unwrap().try_clone().unwrap();
        let clients = self.clients.clone();
        let stats = self.stats.clone();
        let is_running = self.is_running.clone();
        let config = self.config.clone();

        let handle = thread::spawn(move || {
            Self::connection_handler_loop(listener, clients, stats, is_running, config);
        });

        self.worker_handles.push(handle);
    }

    /// 连接处理循环
    fn connection_handler_loop(
        listener: TcpListener,
        clients: Arc<Mutex<HashMap<String, ClientConnection>>>,
        stats: Arc<Mutex<WebSocketStats>>,
        is_running: Arc<Mutex<bool>>,
        config: WebSocketConfig,
    ) {
        while *is_running.lock().unwrap() {
            match listener.accept() {
                Ok((stream, addr)) => {
                    if config.verbose_logging {
                        println!("🔗 新连接: {}", addr);
                    }

                    // 检查连接数限制
                    {
                        let clients_guard = clients.lock().unwrap();
                        if clients_guard.len() >= config.max_connections {
                            if config.verbose_logging {
                                println!("⚠️ 连接数已达上限，拒绝连接: {}", addr);
                            }
                            continue;
                        }
                    }

                    // 处理 WebSocket 握手
                    match Self::handle_websocket_handshake(stream, addr) {
                        Ok((client_id, client_stream)) => {
                            let client = ClientConnection::new(client_id.clone(), client_stream);

                            // 添加到客户端列表
                            {
                                let mut clients_guard = clients.lock().unwrap();
                                clients_guard.insert(client_id.clone(), client);
                                stats.lock().unwrap().record_connection();
                            }

                            // 发送连接确认消息
                            let welcome_message = WebSocketMessage::Connected {
                                client_id: client_id.clone(),
                                timestamp: SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(),
                            };

                            if let Some(client) = clients.lock().unwrap().get(&client_id) {
                                let _ = client.send_message(&welcome_message);
                            }

                            if config.verbose_logging {
                                println!("✅ 客户端已连接: {}", client_id);
                            }
                        }
                        Err(e) => {
                            if config.verbose_logging {
                                println!("❌ WebSocket 握手失败: {}", e);
                            }
                            stats.lock().unwrap().record_error();
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // 非阻塞模式下没有新连接
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    if config.verbose_logging {
                        println!("❌ 接受连接失败: {}", e);
                    }
                    stats.lock().unwrap().record_error();
                }
            }
        }
    }

    /// 处理 WebSocket 握手
    fn handle_websocket_handshake(
        mut stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(String, TcpStream), WebSocketError> {
        // 简化的 WebSocket 握手实现
        let mut buffer = [0; 1024];
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|e| WebSocketError::HandshakeError(e.to_string()))?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        // 检查是否是 WebSocket 升级请求
        if !request.contains("Upgrade: websocket") {
            return Err(WebSocketError::HandshakeError(
                "不是 WebSocket 升级请求".to_string(),
            ));
        }

        // 提取 Sec-WebSocket-Key
        let key = request
            .lines()
            .find(|line| line.starts_with("Sec-WebSocket-Key:"))
            .and_then(|line| line.split(':').nth(1))
            .map(|key| key.trim())
            .ok_or_else(|| WebSocketError::HandshakeError("缺少 Sec-WebSocket-Key".to_string()))?;

        // 生成响应密钥
        let accept_key = Self::generate_accept_key(key);

        // 发送握手响应
        let response = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Accept: {}\r\n\
             \r\n",
            accept_key
        );

        stream
            .write_all(response.as_bytes())
            .map_err(|e| WebSocketError::HandshakeError(e.to_string()))?;

        // 生成客户端ID
        let client_id = format!("client_{}_{}", addr.ip(), addr.port());

        Ok((client_id, stream))
    }

    /// 生成 WebSocket Accept 密钥
    fn generate_accept_key(key: &str) -> String {
        use sha1::{Digest, Sha1};

        let magic_string = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
        let combined = format!("{}{}", key, magic_string);

        let mut hasher = Sha1::new();
        hasher.update(combined.as_bytes());
        let hash = hasher.finalize();

        base64::encode(hash)
    }

    /// 启动心跳工作线程
    fn start_heartbeat_worker(&mut self) {
        let clients = self.clients.clone();
        let is_running = self.is_running.clone();
        let heartbeat_interval = Duration::from_secs(self.config.heartbeat_interval);
        let verbose_logging = self.config.verbose_logging;

        let handle = thread::spawn(move || {
            while *is_running.lock().unwrap() {
                thread::sleep(heartbeat_interval);

                let ping_message = WebSocketMessage::Ping {
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                };

                let clients_guard = clients.lock().unwrap();
                for client in clients_guard.values() {
                    if let Err(e) = client.send_message(&ping_message) {
                        if verbose_logging {
                            println!("❌ 发送心跳失败: {}", e);
                        }
                    }
                }
            }
        });

        self.worker_handles.push(handle);
    }

    /// 启动清理工作线程
    fn start_cleanup_worker(&mut self) {
        let clients = self.clients.clone();
        let stats = self.stats.clone();
        let is_running = self.is_running.clone();
        let timeout = Duration::from_secs(self.config.connection_timeout);
        let verbose_logging = self.config.verbose_logging;

        let handle = thread::spawn(move || {
            while *is_running.lock().unwrap() {
                thread::sleep(Duration::from_secs(60)); // 每分钟清理一次

                let mut expired_clients = Vec::new();

                // 查找过期连接
                {
                    let clients_guard = clients.lock().unwrap();
                    for (client_id, client) in clients_guard.iter() {
                        if client.is_expired(timeout) {
                            expired_clients.push(client_id.clone());
                        }
                    }
                }

                // 移除过期连接
                if !expired_clients.is_empty() {
                    let mut clients_guard = clients.lock().unwrap();
                    let mut stats_guard = stats.lock().unwrap();

                    for client_id in expired_clients {
                        if let Some(client) = clients_guard.remove(&client_id) {
                            let duration = client.connected_at.elapsed().unwrap_or_default();
                            stats_guard.record_disconnection(duration);

                            if verbose_logging {
                                println!("🗑️ 清理过期连接: {}", client_id);
                            }
                        }
                    }
                }
            }
        });

        self.worker_handles.push(handle);
    }
}

impl Drop for WebSocketServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// WebSocket 错误
#[derive(Debug, Clone)]
pub enum WebSocketError {
    /// 服务器已在运行
    ServerAlreadyRunning,
    /// 绑定地址失败
    BindError(String),
    /// 配置错误
    ConfigError(String),
    /// 握手错误
    HandshakeError(String),
    /// 客户端未找到
    ClientNotFound(String),
    /// 发送错误
    SendError(String),
    /// 序列化错误
    SerializationError(String),
    /// 网络错误
    NetworkError(String),
}

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketError::ServerAlreadyRunning => {
                write!(f, "WebSocket 服务器已在运行")
            }
            WebSocketError::BindError(msg) => {
                write!(f, "绑定地址失败: {}", msg)
            }
            WebSocketError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
            WebSocketError::HandshakeError(msg) => {
                write!(f, "握手错误: {}", msg)
            }
            WebSocketError::ClientNotFound(id) => {
                write!(f, "客户端未找到: {}", id)
            }
            WebSocketError::SendError(msg) => {
                write!(f, "发送错误: {}", msg)
            }
            WebSocketError::SerializationError(msg) => {
                write!(f, "序列化错误: {}", msg)
            }
            WebSocketError::NetworkError(msg) => {
                write!(f, "网络错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for WebSocketError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_config_default() {
        let config = WebSocketConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3001);
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_websocket_message_serialization() {
        let message = WebSocketMessage::CssHotReload {
            files: vec!["style.css".to_string()],
            css_content: ".test { color: red; }".to_string(),
            timestamp: SystemTime::now(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: WebSocketMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            WebSocketMessage::CssHotReload {
                files, css_content, ..
            } => {
                assert_eq!(files, vec!["style.css"]);
                assert_eq!(css_content, ".test { color: red; }");
            }
            _ => panic!("消息类型不匹配"),
        }
    }

    #[test]
    fn test_client_connection() {
        use std::net::{TcpListener, TcpStream};

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let stream = TcpStream::connect(addr).unwrap();
        let client = ClientConnection::new("test_client".to_string(), stream);

        assert_eq!(client.id, "test_client");
        assert!(!client.authenticated);
        assert!(!client.is_expired(Duration::from_secs(1)));
    }

    #[test]
    fn test_websocket_stats() {
        let mut stats = WebSocketStats::default();

        stats.record_connection();
        stats.record_message_sent();
        stats.record_error();

        assert_eq!(stats.total_connections, 1);
        assert_eq!(stats.active_connections, 1);
        assert_eq!(stats.messages_sent, 1);
        assert_eq!(stats.errors, 1);

        stats.record_disconnection(Duration::from_secs(10));
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.average_connection_duration, Duration::from_secs(10));
    }
}
