use std::sync::{Arc, RwLock};

/// 主题历史记录
///
/// 管理主题切换的历史记录，支持撤销和重做操作。
/// 该结构体记录主题切换的顺序，允许用户在不同主题之间前进和后退。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::manager::ThemeHistory;
///
/// // 创建主题历史记录
/// let history = ThemeHistory::new();
///
/// // 添加主题切换记录
/// history.add_theme("light").unwrap();
/// history.add_theme("dark").unwrap();
///
/// // 撤销到上一个主题
/// let previous = history.get_previous_theme().unwrap();
/// assert_eq!(previous, Some("light".to_string()));
///
/// // 重做到下一个主题
/// let next = history.get_next_theme().unwrap();
/// assert_eq!(next, Some("dark".to_string()));
/// ```
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
    ///
    /// 创建一个空的主题历史记录，默认最大记录数为10。
    ///
    /// # Returns
    ///
    /// 新创建的主题历史记录
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// ```
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            max_history: 10,
            current_position: Arc::new(RwLock::new(0)),
        }
    }

    /// 设置最大历史记录数
    ///
    /// 设置历史记录可以保存的最大主题数量。
    ///
    /// # Arguments
    ///
    /// * `max` - 最大历史记录数
    ///
    /// # Returns
    ///
    /// 更新后的主题历史记录
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new().with_max_history(20);
    /// ```
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// 添加主题到历史记录
    ///
    /// 将指定的主题名称添加到历史记录中。如果当前位置不在历史记录末尾，
    /// 会截断后面的记录。如果历史记录超出最大数量，会移除最早的记录。
    ///
    /// # Arguments
    ///
    /// * `theme_name` - 要添加的主题名称
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    /// ```
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
    ///
    /// 返回完整的主题历史记录列表。
    ///
    /// # Returns
    ///
    /// 成功时返回主题名称的向量，失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    ///
    /// let themes = history.get_history().unwrap();
    /// assert_eq!(themes, vec!["light", "dark"]);
    /// ```
    pub fn get_history(&self) -> Result<Vec<String>, String> {
        match self.history.read() {
            Ok(history) => Ok(history.clone()),
            Err(_) => Err("无法获取历史记录读锁".to_string()),
        }
    }

    /// 获取上一个主题
    ///
    /// 将当前位置向前移动一步，并返回对应的主题名称。
    /// 如果已经在历史记录的开头，则返回`None`。
    ///
    /// # Returns
    ///
    /// 成功时返回上一个主题的名称（如果有），失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    ///
    /// let previous = history.get_previous_theme().unwrap();
    /// assert_eq!(previous, Some("light".to_string()));
    /// ```
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
    ///
    /// 将当前位置向后移动一步，并返回对应的主题名称。
    /// 如果已经在历史记录的末尾，则返回`None`。
    ///
    /// # Returns
    ///
    /// 成功时返回下一个主题的名称（如果有），失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    ///
    /// // 先后退一步
    /// history.get_previous_theme().unwrap();
    ///
    /// // 再前进一步
    /// let next = history.get_next_theme().unwrap();
    /// assert_eq!(next, Some("dark".to_string()));
    /// ```
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
    ///
    /// 清空所有主题历史记录，并将当前位置重置为0。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    ///
    /// history.clear_history().unwrap();
    /// assert_eq!(history.get_history().unwrap().len(), 0);
    /// ```
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
    ///
    /// 更新历史记录可以保存的最大主题数量。如果当前历史记录超出新的最大值，
    /// 会移除最早的记录。
    ///
    /// # Arguments
    ///
    /// * `max` - 最大历史记录数
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let mut history = ThemeHistory::new();
    /// history.add_theme("theme1").unwrap();
    /// history.add_theme("theme2").unwrap();
    /// history.add_theme("theme3").unwrap();
    ///
    /// // 设置最大记录数为2
    /// history.set_max_history(2);
    ///
    /// // 只保留最新的两个主题
    /// let themes = history.get_history().unwrap();
    /// assert_eq!(themes.len(), 2);
    /// ```
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
    ///
    /// 返回历史记录中的当前位置索引。
    ///
    /// # Returns
    ///
    /// 成功时返回当前位置，失败时返回包含错误信息的`Err`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeHistory;
    ///
    /// let history = ThemeHistory::new();
    /// history.add_theme("light").unwrap();
    /// history.add_theme("dark").unwrap();
    ///
    /// assert_eq!(history.get_current_position().unwrap(), 2);
    ///
    /// // 后退一步
    /// history.get_previous_theme().unwrap();
    /// assert_eq!(history.get_current_position().unwrap(), 1);
    /// ```
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
