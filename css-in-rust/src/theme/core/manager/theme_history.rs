use std::sync::{Arc, RwLock};

/// 主题历史记录
///
/// 管理主题切换的历史记录，支持撤销和重做操作
#[derive(Debug, Clone)]
pub struct ThemeHistory {
    /// 历史记录
    history: Arc<RwLock<Vec<String>>>,
    /// 最大历史记录数
    max_history: usize,
    /// 当前位置
    current_position: Arc<RwLock<usize>>,
}

impl ThemeHistory {
    /// 创建新的主题历史记录
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            max_history: 10,
            current_position: Arc::new(RwLock::new(0)),
        }
    }

    /// 设置最大历史记录数
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// 添加主题到历史记录
    pub fn add_theme(&self, theme_name: &str) -> Result<(), String> {
        let mut history = match self.history.write() {
            Ok(h) => h,
            Err(_) => return Err("无法获取历史记录写锁".to_string()),
        };

        let mut position = match self.current_position.write() {
            Ok(p) => p,
            Err(_) => return Err("无法获取位置写锁".to_string()),
        };

        // 如果不是在历史记录的末尾，需要截断历史记录
        if *position < history.len() {
            history.truncate(*position);
        }

        // 添加新主题
        history.push(theme_name.to_string());

        // 如果超出最大历史记录数，移除最早的记录
        if history.len() > self.max_history {
            history.remove(0);
        } else {
            *position += 1;
        }

        Ok(())
    }

    /// 获取历史记录
    pub fn get_history(&self) -> Result<Vec<String>, String> {
        match self.history.read() {
            Ok(history) => Ok(history.clone()),
            Err(_) => Err("无法获取历史记录读锁".to_string()),
        }
    }

    /// 获取上一个主题
    pub fn get_previous_theme(&self) -> Result<Option<String>, String> {
        let history = match self.history.read() {
            Ok(h) => h,
            Err(_) => return Err("无法获取历史记录读锁".to_string()),
        };

        let mut position = match self.current_position.write() {
            Ok(p) => p,
            Err(_) => return Err("无法获取位置写锁".to_string()),
        };

        // 如果当前位置大于1，可以返回上一个主题
        if *position > 1 {
            *position -= 1;
            return Ok(Some(history[*position - 1].clone()));
        }

        Ok(None)
    }

    /// 获取下一个主题
    pub fn get_next_theme(&self) -> Result<Option<String>, String> {
        let history = match self.history.read() {
            Ok(h) => h,
            Err(_) => return Err("无法获取历史记录读锁".to_string()),
        };

        let mut position = match self.current_position.write() {
            Ok(p) => p,
            Err(_) => return Err("无法获取位置写锁".to_string()),
        };

        // 如果当前位置小于历史记录长度，可以返回下一个主题
        if *position < history.len() {
            *position += 1;
            return Ok(Some(history[*position - 1].clone()));
        }

        Ok(None)
    }

    /// 清除历史记录
    pub fn clear_history(&self) -> Result<(), String> {
        let mut history = match self.history.write() {
            Ok(h) => h,
            Err(_) => return Err("无法获取历史记录写锁".to_string()),
        };

        let mut position = match self.current_position.write() {
            Ok(p) => p,
            Err(_) => return Err("无法获取位置写锁".to_string()),
        };

        history.clear();
        *position = 0;

        Ok(())
    }

    /// 设置最大历史记录数
    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;

        // 如果当前历史记录超出新的最大值，需要截断
        if let Ok(mut history) = self.history.write() {
            let history_len = history.len();
            if history_len > max {
                history.drain(0..history_len - max);
            }
        }
    }

    /// 获取当前位置
    pub fn get_current_position(&self) -> Result<usize, String> {
        match self.current_position.read() {
            Ok(position) => Ok(*position),
            Err(_) => Err("无法获取位置读锁".to_string()),
        }
    }
}

impl Default for ThemeHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_history_basic() {
        let history = ThemeHistory::new();

        // 添加主题
        history.add_theme("light").unwrap();
        history.add_theme("dark").unwrap();
        history.add_theme("custom").unwrap();

        // 检查历史记录
        let themes = history.get_history().unwrap();
        assert_eq!(themes, vec!["light", "dark", "custom"]);

        // 检查当前位置
        assert_eq!(history.get_current_position().unwrap(), 3);

        // 获取上一个主题
        let prev = history.get_previous_theme().unwrap();
        assert_eq!(prev, Some("dark".to_string()));

        // 检查当前位置
        assert_eq!(history.get_current_position().unwrap(), 2);

        // 获取下一个主题
        let next = history.get_next_theme().unwrap();
        assert_eq!(next, Some("custom".to_string()));

        // 清除历史记录
        history.clear_history().unwrap();
        assert_eq!(history.get_history().unwrap().len(), 0);
        assert_eq!(history.get_current_position().unwrap(), 0);
    }

    #[test]
    fn test_theme_history_max_size() {
        let history = ThemeHistory::new().with_max_history(2);

        // 添加超过最大数量的主题
        history.add_theme("theme1").unwrap();
        history.add_theme("theme2").unwrap();
        history.add_theme("theme3").unwrap();

        // 检查历史记录，应该只保留最新的两个
        let themes = history.get_history().unwrap();
        assert_eq!(themes, vec!["theme2", "theme3"]);
    }
}
