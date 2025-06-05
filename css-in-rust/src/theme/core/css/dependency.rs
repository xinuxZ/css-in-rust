use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// 依赖类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependencyType {
    /// 变量依赖
    Variable,
    /// 主题依赖
    Theme,
    /// 组件依赖
    Component,
    /// 媒体查询依赖
    MediaQuery,
}

/// 依赖项
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    /// 依赖类型
    pub dep_type: DependencyType,
    /// 依赖名称
    pub name: String,
    /// 依赖路径
    pub path: Option<String>,
}

impl Dependency {
    /// 创建变量依赖
    pub fn variable(name: impl Into<String>) -> Self {
        Self {
            dep_type: DependencyType::Variable,
            name: name.into(),
            path: None,
        }
    }

    /// 创建主题依赖
    pub fn theme(name: impl Into<String>) -> Self {
        Self {
            dep_type: DependencyType::Theme,
            name: name.into(),
            path: None,
        }
    }

    /// 创建组件依赖
    pub fn component(name: impl Into<String>) -> Self {
        Self {
            dep_type: DependencyType::Component,
            name: name.into(),
            path: None,
        }
    }

    /// 创建媒体查询依赖
    pub fn media_query(name: impl Into<String>) -> Self {
        Self {
            dep_type: DependencyType::MediaQuery,
            name: name.into(),
            path: None,
        }
    }

    /// 设置依赖路径
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

/// 依赖图
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// 依赖关系映射
    dependencies: HashMap<String, HashSet<Dependency>>,
    /// 反向依赖关系映射
    reverse_dependencies: HashMap<Dependency, HashSet<String>>,
}

impl DependencyGraph {
    /// 创建新的依赖图
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    /// 添加依赖
    pub fn add_dependency(&mut self, source: impl Into<String>, dependency: Dependency) {
        let source = source.into();

        // 添加正向依赖
        self.dependencies
            .entry(source.clone())
            .or_insert_with(HashSet::new)
            .insert(dependency.clone());

        // 添加反向依赖
        self.reverse_dependencies
            .entry(dependency)
            .or_insert_with(HashSet::new)
            .insert(source);
    }

    /// 获取依赖项
    pub fn get_dependencies(&self, source: &str) -> Option<&HashSet<Dependency>> {
        self.dependencies.get(source)
    }

    /// 获取反向依赖项
    pub fn get_reverse_dependencies(&self, dependency: &Dependency) -> Option<&HashSet<String>> {
        self.reverse_dependencies.get(dependency)
    }

    /// 移除依赖
    pub fn remove_dependency(&mut self, source: &str, dependency: &Dependency) {
        // 移除正向依赖
        if let Some(deps) = self.dependencies.get_mut(source) {
            deps.remove(dependency);
            if deps.is_empty() {
                self.dependencies.remove(source);
            }
        }

        // 移除反向依赖
        if let Some(sources) = self.reverse_dependencies.get_mut(dependency) {
            sources.remove(source);
            if sources.is_empty() {
                self.reverse_dependencies.remove(dependency);
            }
        }
    }

    /// 移除所有依赖
    pub fn remove_all_dependencies(&mut self, source: &str) {
        if let Some(deps) = self.dependencies.remove(source) {
            for dep in deps {
                if let Some(sources) = self.reverse_dependencies.get_mut(&dep) {
                    sources.remove(source);
                    if sources.is_empty() {
                        self.reverse_dependencies.remove(&dep);
                    }
                }
            }
        }
    }

    /// 获取受影响的源
    pub fn get_affected_sources(&self, dependency: &Dependency) -> HashSet<String> {
        self.get_reverse_dependencies(dependency)
            .cloned()
            .unwrap_or_default()
    }

    /// 检查是否存在依赖环
    pub fn has_dependency_cycle(&self, source: &str) -> bool {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        self.dfs_check_cycle(source, &mut visited, &mut path)
    }

    /// 深度优先搜索检查环
    fn dfs_check_cycle(
        &self,
        current: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if path.contains(&current.to_string()) {
            return true;
        }

        if visited.contains(current) {
            return false;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        if let Some(deps) = self.dependencies.get(current) {
            for dep in deps {
                if let DependencyType::Component = dep.dep_type {
                    if self.dfs_check_cycle(&dep.name, visited, path) {
                        return true;
                    }
                }
            }
        }

        path.pop();
        false
    }
}

/// 线程安全的依赖追踪器
pub struct DependencyTracker {
    /// 依赖图
    graph: Arc<RwLock<DependencyGraph>>,
}

impl DependencyTracker {
    /// 创建新的依赖追踪器
    pub fn new() -> Self {
        Self {
            graph: Arc::new(RwLock::new(DependencyGraph::new())),
        }
    }

    /// 添加依赖
    pub fn track_dependency(
        &self,
        source: impl Into<String>,
        dependency: Dependency,
    ) -> Result<(), String> {
        match self.graph.write() {
            Ok(mut graph) => {
                graph.add_dependency(source, dependency);
                Ok(())
            }
            Err(_) => Err("无法获取依赖图写锁".to_string()),
        }
    }

    /// 获取依赖项
    pub fn get_dependencies(&self, source: &str) -> Result<HashSet<Dependency>, String> {
        match self.graph.read() {
            Ok(graph) => Ok(graph.get_dependencies(source).cloned().unwrap_or_default()),
            Err(_) => Err("无法获取依赖图读锁".to_string()),
        }
    }

    /// 获取受影响的源
    pub fn get_affected_sources(&self, dependency: &Dependency) -> Result<HashSet<String>, String> {
        match self.graph.read() {
            Ok(graph) => Ok(graph.get_affected_sources(dependency)),
            Err(_) => Err("无法获取依赖图读锁".to_string()),
        }
    }

    /// 移除所有依赖
    pub fn remove_all_dependencies(&self, source: &str) -> Result<(), String> {
        match self.graph.write() {
            Ok(mut graph) => {
                graph.remove_all_dependencies(source);
                Ok(())
            }
            Err(_) => Err("无法获取依赖图写锁".to_string()),
        }
    }

    /// 检查是否存在依赖环
    pub fn has_dependency_cycle(&self, source: &str) -> Result<bool, String> {
        match self.graph.read() {
            Ok(graph) => Ok(graph.has_dependency_cycle(source)),
            Err(_) => Err("无法获取依赖图读锁".to_string()),
        }
    }
}

impl Default for DependencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_graph_basic() {
        let mut graph = DependencyGraph::new();

        // 添加依赖
        graph.add_dependency("button", Dependency::variable("color"));
        graph.add_dependency("button", Dependency::variable("background"));
        graph.add_dependency("input", Dependency::variable("border"));

        // 检查依赖项
        let button_deps = graph.get_dependencies("button").unwrap();
        assert_eq!(button_deps.len(), 2);
        assert!(button_deps.contains(&Dependency::variable("color")));
        assert!(button_deps.contains(&Dependency::variable("background")));

        // 检查反向依赖
        let color_deps = graph
            .get_reverse_dependencies(&Dependency::variable("color"))
            .unwrap();
        assert_eq!(color_deps.len(), 1);
        assert!(color_deps.contains("button"));

        // 移除依赖
        graph.remove_dependency("button", &Dependency::variable("color"));
        let button_deps = graph.get_dependencies("button").unwrap();
        assert_eq!(button_deps.len(), 1);
        assert!(button_deps.contains(&Dependency::variable("background")));

        // 移除所有依赖
        graph.remove_all_dependencies("button");
        assert!(graph.get_dependencies("button").is_none());
    }

    #[test]
    fn test_dependency_cycle_detection() {
        let mut graph = DependencyGraph::new();

        // 创建依赖环
        graph.add_dependency("a", Dependency::component("b"));
        graph.add_dependency("b", Dependency::component("c"));
        graph.add_dependency("c", Dependency::component("a"));

        // 检测环
        assert!(graph.has_dependency_cycle("a"));

        // 创建无环依赖
        let mut graph2 = DependencyGraph::new();
        graph2.add_dependency("a", Dependency::component("b"));
        graph2.add_dependency("b", Dependency::component("c"));
        graph2.add_dependency("c", Dependency::component("d"));

        // 检测环
        assert!(!graph2.has_dependency_cycle("a"));
    }
}
