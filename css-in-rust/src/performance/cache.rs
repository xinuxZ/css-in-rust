//! 编译时缓存模块
//!
//! 提供CSS编译结果的缓存功能，支持持久化和压缩

use serde::{Deserialize, Serialize};
#[cfg(feature = "optimizer")]
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 最大缓存大小（字节）
    pub max_size: usize,
    /// 缓存过期时间
    pub ttl: Duration,
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 是否启用持久化
    pub enable_persistence: bool,
    /// 缓存目录
    pub cache_dir: PathBuf,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 100 * 1024 * 1024,    // 100MB
            ttl: Duration::from_secs(3600), // 1小时
            enable_compression: true,
            enable_persistence: true,
            cache_dir: PathBuf::from("target/css-cache"),
        }
    }
}

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// 缓存键
    pub key: String,
    /// 缓存值
    pub value: String,
    /// 创建时间戳
    pub created_at: u64,
    /// 最后访问时间戳
    pub last_accessed: u64,
    /// 访问次数
    pub access_count: usize,
    /// 数据大小（字节）
    pub size: usize,
    /// 源文件哈希
    pub source_hash: String,
    /// 编译配置哈希
    pub config_hash: String,
}

impl CacheEntry {
    /// 创建新的缓存条目
    pub fn new(key: String, value: String, source_hash: String, config_hash: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let size = key.len() + value.len() + source_hash.len() + config_hash.len();

        Self {
            key,
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            size,
            source_hash,
            config_hash,
        }
    }

    /// 检查缓存是否过期
    pub fn is_expired(&self, ttl: Duration) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.created_at > ttl.as_secs()
    }

    /// 更新访问信息
    pub fn update_access(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.access_count += 1;
    }

    /// 检查源文件是否发生变化
    pub fn is_source_changed(&self, current_hash: &str) -> bool {
        self.source_hash != current_hash
    }

    /// 检查编译配置是否发生变化
    pub fn is_config_changed(&self, current_hash: &str) -> bool {
        self.config_hash != current_hash
    }
}

/// 缓存统计信息
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// 缓存命中次数
    pub hits: usize,
    /// 缓存未命中次数
    pub misses: usize,
    /// 缓存条目数量
    pub entries: usize,
    /// 总缓存大小（字节）
    pub total_size: usize,
    /// 过期条目数量
    pub expired_entries: usize,
}

impl CacheStats {
    /// 计算缓存命中率
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            return 0.0;
        }
        self.hits as f64 / (self.hits + self.misses) as f64
    }

    /// 记录缓存命中
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// 记录缓存未命中
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }
}

/// 缓存管理器
pub struct CacheManager {
    config: CacheConfig,
    cache: HashMap<String, CacheEntry>,
    stats: CacheStats,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new(config: CacheConfig) -> Self {
        let mut manager = Self {
            config,
            cache: HashMap::new(),
            stats: CacheStats::default(),
        };

        // 尝试加载持久化缓存
        if manager.config.enable_persistence {
            if let Err(e) = manager.load_cache() {
                eprintln!("Failed to load cache: {}", e);
            }
        }

        manager
    }

    /// 生成缓存键
    #[cfg(feature = "optimizer")]
    pub fn generate_cache_key(&self, css_content: &str, config_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(css_content.as_bytes());
        hasher.update(config_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 生成缓存键 (fallback)
    #[cfg(not(feature = "optimizer"))]
    pub fn generate_cache_key(&self, css_content: &str, config_hash: &str) -> String {
        let mut hasher = DefaultHasher::new();
        css_content.hash(&mut hasher);
        config_hash.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 生成内容哈希
    #[cfg(feature = "optimizer")]
    pub fn generate_content_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 生成内容哈希 (fallback)
    #[cfg(not(feature = "optimizer"))]
    pub fn generate_content_hash(&self, content: &str) -> String {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 获取缓存条目
    pub fn get(&mut self, key: &str, source_hash: &str, config_hash: &str) -> Option<String> {
        if let Some(entry) = self.cache.get_mut(key) {
            // 检查是否过期
            if entry.is_expired(self.config.ttl) {
                self.cache.remove(key);
                self.stats.record_miss();
                return None;
            }

            // 检查源文件和配置是否发生变化
            if entry.is_source_changed(source_hash) || entry.is_config_changed(config_hash) {
                self.cache.remove(key);
                self.stats.record_miss();
                return None;
            }

            // 更新访问信息
            entry.update_access();
            self.stats.record_hit();

            Some(entry.value.clone())
        } else {
            self.stats.record_miss();
            None
        }
    }

    /// 设置缓存条目
    pub fn set(&mut self, key: String, value: String, source_hash: String, config_hash: String) {
        let entry = CacheEntry::new(key.clone(), value, source_hash, config_hash);

        // 检查缓存大小限制
        if self.get_total_size() + entry.size > self.config.max_size {
            self.evict_lru();
        }

        self.cache.insert(key, entry);
        self.update_stats();
    }

    /// 清理过期缓存
    pub fn cleanup(&mut self) {
        let expired_keys: Vec<String> = self
            .cache
            .iter()
            .filter(|(_, entry)| entry.is_expired(self.config.ttl))
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.cache.remove(&key);
        }

        self.update_stats();
    }

    /// LRU淘汰策略
    fn evict_lru(&mut self) {
        if self.cache.is_empty() {
            return;
        }

        // 找到最久未访问的条目
        let lru_key = self
            .cache
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone());

        if let Some(key) = lru_key {
            self.cache.remove(&key);
        }
    }

    /// 获取总缓存大小
    fn get_total_size(&self) -> usize {
        self.cache.values().map(|entry| entry.size).sum()
    }

    /// 更新统计信息
    fn update_stats(&mut self) {
        self.stats.entries = self.cache.len();
        self.stats.total_size = self.get_total_size();
        self.stats.expired_entries = self
            .cache
            .values()
            .filter(|entry| entry.is_expired(self.config.ttl))
            .count();
    }

    /// 获取缓存统计信息
    pub fn get_stats(&self) -> &CacheStats {
        &self.stats
    }

    /// 获取缓存条目数量
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// 获取缓存容量
    pub fn capacity(&self) -> usize {
        self.config.max_size
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.cache.clear();
        self.stats = CacheStats::default();
    }

    /// 保存缓存到磁盘
    pub fn save_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.enable_persistence {
            return Ok(());
        }

        // 确保缓存目录存在
        fs::create_dir_all(&self.config.cache_dir)?;

        let cache_file = self.config.cache_dir.join("cache.json");
        let cache_data = serde_json::to_string(&self.cache)?;

        if self.config.enable_compression {
            // 这里可以添加压缩逻辑
            fs::write(cache_file, cache_data)?;
        } else {
            fs::write(cache_file, cache_data)?;
        }

        Ok(())
    }

    /// 从磁盘加载缓存
    pub fn load_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.enable_persistence {
            return Ok(());
        }

        let cache_file = self.config.cache_dir.join("cache.json");

        if !cache_file.exists() {
            return Ok(());
        }

        let cache_data = fs::read_to_string(cache_file)?;

        if self.config.enable_compression {
            // 这里可以添加解压缩逻辑
            self.cache = serde_json::from_str(&cache_data)?;
        } else {
            self.cache = serde_json::from_str(&cache_data)?;
        }

        // 清理过期缓存
        self.cleanup();
        self.update_stats();

        Ok(())
    }
}

impl Drop for CacheManager {
    fn drop(&mut self) {
        if let Err(e) = self.save_cache() {
            eprintln!("Failed to save cache: {}", e);
        }
    }
}
