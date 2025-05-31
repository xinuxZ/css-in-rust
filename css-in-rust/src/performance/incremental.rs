//! 增量编译模块
//!
//! 提供智能的增量编译功能，只重新编译发生变化的CSS

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// 文件依赖信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDependency {
    /// 文件路径
    pub path: PathBuf,
    /// 文件哈希
    pub hash: String,
    /// 最后修改时间
    pub last_modified: u64,
    /// 文件大小
    pub size: u64,
    /// 依赖的文件列表
    pub dependencies: HashSet<PathBuf>,
}

impl FileDependency {
    /// 创建新的文件依赖信息
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let metadata = fs::metadata(&path)?;
        let content = fs::read_to_string(&path)?;
        let hash = Self::calculate_hash(&content);

        let last_modified = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Self {
            path,
            hash,
            last_modified,
            size: metadata.len(),
            dependencies: HashSet::new(),
        })
    }

    /// 计算文件内容哈希
    pub fn calculate_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 检查文件是否发生变化
    pub fn is_changed(&self) -> bool {
        if let Ok(metadata) = fs::metadata(&self.path) {
            let current_modified = metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if current_modified != self.last_modified || metadata.len() != self.size {
                return true;
            }

            // 如果时间和大小都没变，检查内容哈希
            if let Ok(content) = fs::read_to_string(&self.path) {
                let current_hash = Self::calculate_hash(&content);
                return current_hash != self.hash;
            }
        }

        true // 如果无法读取文件，认为已变化
    }

    /// 更新文件信息
    pub fn update(&mut self) -> Result<(), std::io::Error> {
        let metadata = fs::metadata(&self.path)?;
        let content = fs::read_to_string(&self.path)?;

        self.hash = Self::calculate_hash(&content);
        self.last_modified = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.size = metadata.len();

        Ok(())
    }

    /// 添加依赖文件
    pub fn add_dependency(&mut self, dep_path: PathBuf) {
        self.dependencies.insert(dep_path);
    }

    /// 检查依赖文件是否发生变化
    pub fn has_dependency_changed(
        &self,
        dependency_map: &HashMap<PathBuf, FileDependency>,
    ) -> bool {
        for dep_path in &self.dependencies {
            if let Some(dep) = dependency_map.get(dep_path) {
                if dep.is_changed() {
                    return true;
                }
            } else {
                // 依赖文件不存在，认为已变化
                return true;
            }
        }
        false
    }
}

/// 编译状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationState {
    /// 文件依赖映射
    pub dependencies: HashMap<PathBuf, FileDependency>,
    /// 编译输出映射
    pub outputs: HashMap<PathBuf, String>,
    /// 最后编译时间
    pub last_compile_time: u64,
    /// 编译配置哈希
    pub config_hash: String,
}

impl CompilationState {
    /// 创建新的编译状态
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            outputs: HashMap::new(),
            last_compile_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            config_hash: String::new(),
        }
    }

    /// 添加文件依赖
    pub fn add_file(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        let dependency = FileDependency::new(path.clone())?;
        self.dependencies.insert(path, dependency);
        Ok(())
    }

    /// 更新编译输出
    pub fn update_output(&mut self, path: PathBuf, output: String) {
        self.outputs.insert(path, output);
    }

    /// 检查文件是否需要重新编译
    pub fn needs_recompile(&self, path: &Path) -> bool {
        if let Some(dependency) = self.dependencies.get(path) {
            // 检查文件本身是否变化
            if dependency.is_changed() {
                return true;
            }

            // 检查依赖文件是否变化
            if dependency.has_dependency_changed(&self.dependencies) {
                return true;
            }

            false
        } else {
            // 新文件，需要编译
            true
        }
    }

    /// 获取需要重新编译的文件列表
    pub fn get_changed_files(&self) -> Vec<PathBuf> {
        self.dependencies
            .iter()
            .filter(|(path, _)| self.needs_recompile(path))
            .map(|(path, _)| path.clone())
            .collect()
    }

    /// 更新编译时间
    pub fn update_compile_time(&mut self) {
        self.last_compile_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// 设置配置哈希
    pub fn set_config_hash(&mut self, hash: String) {
        self.config_hash = hash;
    }

    /// 检查配置是否发生变化
    pub fn is_config_changed(&self, current_hash: &str) -> bool {
        self.config_hash != current_hash
    }
}

/// 增量编译器
pub struct IncrementalCompiler {
    state: CompilationState,
    state_file: PathBuf,
}

impl IncrementalCompiler {
    /// 创建新的增量编译器
    pub fn new() -> Self {
        Self {
            state: CompilationState::new(),
            state_file: PathBuf::from("target/css-incremental-state.json"),
        }
    }

    /// 使用指定状态文件创建增量编译器
    pub fn with_state_file(state_file: PathBuf) -> Self {
        let mut compiler = Self {
            state: CompilationState::new(),
            state_file,
        };

        // 尝试加载现有状态
        if let Err(e) = compiler.load_state() {
            eprintln!("Failed to load incremental state: {}", e);
        }

        compiler
    }

    /// 添加源文件
    pub fn add_source_file(&mut self, path: PathBuf) -> Result<(), std::io::Error> {
        self.state.add_file(path)
    }

    /// 添加文件依赖关系
    pub fn add_dependency(&mut self, source: &Path, dependency: PathBuf) {
        if let Some(file_dep) = self.state.dependencies.get_mut(source) {
            file_dep.add_dependency(dependency);
        }
    }

    /// 检查是否需要增量编译
    pub fn needs_incremental_compile(&self, config_hash: &str) -> bool {
        // 检查配置是否变化
        if self.state.is_config_changed(config_hash) {
            return true;
        }

        // 检查是否有文件变化
        !self.state.get_changed_files().is_empty()
    }

    /// 获取需要重新编译的文件
    pub fn get_files_to_compile(&self) -> Vec<PathBuf> {
        self.state.get_changed_files()
    }

    /// 标记文件编译完成
    pub fn mark_compiled(&mut self, path: PathBuf, output: String) -> Result<(), std::io::Error> {
        // 更新文件依赖信息
        if let Some(dependency) = self.state.dependencies.get_mut(&path) {
            dependency.update()?;
        }

        // 更新编译输出
        self.state.update_output(path, output);

        Ok(())
    }

    /// 完成编译
    pub fn finish_compilation(&mut self, config_hash: String) {
        self.state.set_config_hash(config_hash);
        self.state.update_compile_time();

        // 保存状态
        if let Err(e) = self.save_state() {
            eprintln!("Failed to save incremental state: {}", e);
        }
    }

    /// 获取编译输出
    pub fn get_output(&self, path: &Path) -> Option<&String> {
        self.state.outputs.get(path)
    }

    /// 清理状态
    pub fn clean(&mut self) {
        self.state = CompilationState::new();
        if let Err(e) = fs::remove_file(&self.state_file) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("Failed to remove state file: {}", e);
            }
        }
    }

    /// 保存编译状态
    pub fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 确保目录存在
        if let Some(parent) = self.state_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let state_data = serde_json::to_string_pretty(&self.state)?;
        fs::write(&self.state_file, state_data)?;

        Ok(())
    }

    /// 加载编译状态
    pub fn load_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.state_file.exists() {
            return Ok(());
        }

        let state_data = fs::read_to_string(&self.state_file)?;
        self.state = serde_json::from_str(&state_data)?;

        Ok(())
    }

    /// 获取编译统计信息
    pub fn get_stats(&self) -> IncrementalStats {
        let total_files = self.state.dependencies.len();
        let changed_files = self.state.get_changed_files().len();
        let unchanged_files = total_files - changed_files;

        IncrementalStats {
            total_files,
            changed_files,
            unchanged_files,
            last_compile_time: self.state.last_compile_time,
            cache_hit_rate: if total_files > 0 {
                unchanged_files as f64 / total_files as f64
            } else {
                0.0
            },
        }
    }
}

/// 增量编译统计信息
#[derive(Debug, Clone)]
pub struct IncrementalStats {
    /// 总文件数
    pub total_files: usize,
    /// 变化的文件数
    pub changed_files: usize,
    /// 未变化的文件数
    pub unchanged_files: usize,
    /// 最后编译时间
    pub last_compile_time: u64,
    /// 缓存命中率
    pub cache_hit_rate: f64,
}

impl IncrementalStats {
    /// 计算节省的编译时间百分比
    pub fn time_saved_percentage(&self) -> f64 {
        self.cache_hit_rate * 100.0
    }
}
