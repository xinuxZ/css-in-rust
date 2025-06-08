use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// CSS 属性值类型
///
/// 表示CSS属性的各种可能值类型，包括字符串、数字、布尔值、对象、数组和空值。
/// 这个枚举允许构建复杂的CSS数据结构，支持嵌套对象和数组。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{CssValue, CssObject};
///
/// // 创建不同类型的CSS值
/// let string_value = CssValue::String("red".to_string());
/// let number_value = CssValue::Number(16.0);
/// let bool_value = CssValue::Bool(true);
///
/// // 创建嵌套对象
/// let mut nested_object = CssObject::new();
/// nested_object.set("color", "blue");
/// let object_value = CssValue::Object(nested_object);
///
/// // 创建数组值
/// let array_value = CssValue::Array(vec![
///     CssValue::String("solid".to_string()),
///     CssValue::Number(1.0),
///     CssValue::String("black".to_string())
/// ]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CssValue {
    /// 字符串值，如 "red", "16px", "bold"
    String(String),
    /// 数字值，如 16.0, 1.5, 100
    Number(f64),
    /// 布尔值，如 true, false
    Bool(bool),
    /// 嵌套的CSS对象，用于复杂属性或嵌套选择器
    Object(CssObject),
    /// 数组值，用于多值属性如 border, transform 等
    Array(Vec<CssValue>),
    /// 空值，表示属性不存在或已被移除
    Null,
}

impl From<&str> for CssValue {
    /// 从字符串字面量创建CSS值
    ///
    /// # 参数
    ///
    /// * `s` - 字符串字面量
    ///
    /// # 返回值
    ///
    /// 返回包含字符串的 `CssValue::String` 变体
    fn from(s: &str) -> Self {
        CssValue::String(s.to_string())
    }
}

impl From<String> for CssValue {
    /// 从String类型创建CSS值
    ///
    /// # 参数
    ///
    /// * `s` - String对象
    ///
    /// # 返回值
    ///
    /// 返回包含字符串的 `CssValue::String` 变体
    fn from(s: String) -> Self {
        CssValue::String(s)
    }
}

impl From<f64> for CssValue {
    /// 从f64类型创建CSS值
    ///
    /// # 参数
    ///
    /// * `n` - 浮点数值
    ///
    /// # 返回值
    ///
    /// 返回包含数值的 `CssValue::Number` 变体
    fn from(n: f64) -> Self {
        CssValue::Number(n)
    }
}

impl From<i32> for CssValue {
    /// 从i32类型创建CSS值
    ///
    /// # 参数
    ///
    /// * `n` - 整数值
    ///
    /// # 返回值
    ///
    /// 返回包含转换为f64的数值的 `CssValue::Number` 变体
    fn from(n: i32) -> Self {
        CssValue::Number(n as f64)
    }
}

impl From<bool> for CssValue {
    /// 从bool类型创建CSS值
    ///
    /// # 参数
    ///
    /// * `b` - 布尔值
    ///
    /// # 返回值
    ///
    /// 返回包含布尔值的 `CssValue::Bool` 变体
    fn from(b: bool) -> Self {
        CssValue::Bool(b)
    }
}

impl From<CssObject> for CssValue {
    /// 从CssObject类型创建CSS值
    ///
    /// # 参数
    ///
    /// * `o` - CSS对象
    ///
    /// # 返回值
    ///
    /// 返回包含CSS对象的 `CssValue::Object` 变体
    fn from(o: CssObject) -> Self {
        CssValue::Object(o)
    }
}

impl<T> From<Vec<T>> for CssValue
where
    T: Into<CssValue>,
{
    /// 从向量创建CSS数组值
    ///
    /// # 参数
    ///
    /// * `v` - 可转换为CssValue的元素向量
    ///
    /// # 返回值
    ///
    /// 返回包含转换后元素的 `CssValue::Array` 变体
    fn from(v: Vec<T>) -> Self {
        CssValue::Array(v.into_iter().map(|item| item.into()).collect())
    }
}

impl CssValue {
    /// 尝试获取字符串值
    ///
    /// 如果当前值是字符串类型，返回字符串的引用；否则返回None。
    ///
    /// # 返回值
    ///
    /// 如果是字符串值，返回 `Some(&str)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssValue;
    ///
    /// let value = CssValue::String("red".to_string());
    /// assert_eq!(value.as_str(), Some("red"));
    ///
    /// let number_value = CssValue::Number(42.0);
    /// assert_eq!(number_value.as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            CssValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// 尝试获取数值
    ///
    /// 如果当前值是数字类型，返回数值；否则返回None。
    ///
    /// # 返回值
    ///
    /// 如果是数字值，返回 `Some(f64)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssValue;
    ///
    /// let value = CssValue::Number(42.0);
    /// assert_eq!(value.as_f64(), Some(42.0));
    ///
    /// let string_value = CssValue::String("red".to_string());
    /// assert_eq!(string_value.as_f64(), None);
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CssValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// 尝试获取布尔值
    ///
    /// 如果当前值是布尔类型，返回布尔值；否则返回None。
    ///
    /// # 返回值
    ///
    /// 如果是布尔值，返回 `Some(bool)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssValue;
    ///
    /// let value = CssValue::Bool(true);
    /// assert_eq!(value.as_bool(), Some(true));
    ///
    /// let string_value = CssValue::String("true".to_string());
    /// assert_eq!(string_value.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CssValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// 尝试获取对象值
    ///
    /// 如果当前值是对象类型，返回对象的引用；否则返回None。
    ///
    /// # 返回值
    ///
    /// 如果是对象值，返回 `Some(&CssObject)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{CssValue, CssObject};
    ///
    /// let mut obj = CssObject::new();
    /// obj.set("color", "red");
    /// let value = CssValue::Object(obj);
    ///
    /// assert!(value.as_object().is_some());
    /// assert_eq!(value.as_object().unwrap().get("color").unwrap().as_str(), Some("red"));
    /// ```
    pub fn as_object(&self) -> Option<&CssObject> {
        match self {
            CssValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// 尝试获取数组值
    ///
    /// 如果当前值是数组类型，返回数组的引用；否则返回None。
    ///
    /// # 返回值
    ///
    /// 如果是数组值，返回 `Some(&Vec<CssValue>)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssValue;
    ///
    /// let array = vec![CssValue::String("solid".to_string()), CssValue::Number(1.0)];
    /// let value = CssValue::Array(array);
    ///
    /// assert!(value.as_array().is_some());
    /// assert_eq!(value.as_array().unwrap().len(), 2);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<CssValue>> {
        match self {
            CssValue::Array(a) => Some(a),
            _ => None,
        }
    }
}

/// CSS 对象，表示一组 CSS 属性或嵌套规则
///
/// 这个结构体用于表示CSS规则集，可以包含属性和值，也可以嵌套其他CSS对象。
/// 它提供了一种结构化的方式来构建和操作CSS，支持属性的添加、获取、删除和合并等操作。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssObject;
///
/// // 创建基本的CSS对象
/// let mut button = CssObject::new();
/// button.set("background-color", "#1890ff");
/// button.set("color", "white");
/// button.set("padding", "8px 16px");
/// button.set("border-radius", 4);
///
/// // 创建嵌套的CSS对象（用于伪类）
/// let mut hover = CssObject::new();
/// hover.set("background-color", "#40a9ff");
///
/// // 添加嵌套规则
/// button.set("&:hover", hover);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssObject {
    /// CSS属性映射，键为属性名，值为属性值
    pub properties: HashMap<String, CssValue>,
}

impl CssObject {
    /// 创建新的空 CSS 对象
    ///
    /// 初始化一个没有任何属性的CSS对象。
    ///
    /// # 返回值
    ///
    /// 返回一个新的空 `CssObject` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let css_obj = CssObject::new();
    /// assert!(css_obj.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// 从 JSON 值创建 CSS 对象
    ///
    /// 将JSON对象转换为CSS对象，支持嵌套结构。
    ///
    /// # 参数
    ///
    /// * `json` - JSON值
    ///
    /// # 返回值
    ///
    /// 如果转换成功，返回 `Ok(CssObject)`；如果失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    /// use serde_json::json;
    ///
    /// let json = json!({
    ///     "color": "red",
    ///     "font-size": 16,
    ///     "&:hover": {
    ///         "color": "blue"
    ///     }
    /// });
    ///
    /// let css_obj = CssObject::from_json(json).unwrap();
    /// assert_eq!(css_obj.get("color").unwrap().as_str(), Some("red"));
    /// ```
    pub fn from_json(json: Value) -> Result<Self, String> {
        match json {
            Value::Object(map) => {
                let mut properties = HashMap::new();
                for (key, value) in map {
                    properties.insert(key, convert_json_to_css_value(value)?);
                }
                Ok(Self { properties })
            }
            _ => Err("Expected JSON object".to_string()),
        }
    }

    /// 设置 CSS 属性
    ///
    /// 添加或更新CSS对象中的属性。
    ///
    /// # 参数
    ///
    /// * `key` - 属性名
    /// * `value` - 属性值，可以是任何可转换为 `CssValue` 的类型
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    ///
    /// // 设置字符串属性
    /// css_obj.set("color", "red");
    ///
    /// // 设置数字属性
    /// css_obj.set("font-size", 16);
    ///
    /// // 设置嵌套对象
    /// let mut hover = CssObject::new();
    /// hover.set("color", "blue");
    /// css_obj.set("&:hover", hover);
    /// ```
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<CssValue>,
    {
        self.properties.insert(key.into(), value.into());
    }

    /// 获取 CSS 属性
    ///
    /// 获取指定属性名的CSS值。
    ///
    /// # 参数
    ///
    /// * `key` - 属性名
    ///
    /// # 返回值
    ///
    /// 如果属性存在，返回 `Some(&CssValue)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("color", "red");
    ///
    /// let color = css_obj.get("color");
    /// assert!(color.is_some());
    /// assert_eq!(color.unwrap().as_str(), Some("red"));
    ///
    /// let font_size = css_obj.get("font-size");
    /// assert!(font_size.is_none());
    /// ```
    pub fn get(&self, key: &str) -> Option<&CssValue> {
        self.properties.get(key)
    }

    /// 移除 CSS 属性
    ///
    /// 从CSS对象中移除指定的属性。
    ///
    /// # 参数
    ///
    /// * `key` - 要移除的属性名
    ///
    /// # 返回值
    ///
    /// 如果属性存在并被移除，返回 `Some(CssValue)`；如果属性不存在，返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("color", "red");
    /// css_obj.set("font-size", 16);
    ///
    /// // 移除属性
    /// let removed = css_obj.remove("color");
    /// assert!(removed.is_some());
    /// assert_eq!(removed.unwrap().as_str(), Some("red"));
    ///
    /// // 属性已被移除
    /// assert!(css_obj.get("color").is_none());
    /// assert_eq!(css_obj.len(), 1);
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<CssValue> {
        self.properties.remove(key)
    }

    /// 合并另一个 CSS 对象到当前对象
    ///
    /// 将另一个CSS对象的所有属性合并到当前对象中。
    /// 如果有相同的属性名，将覆盖当前对象中的值。
    ///
    /// # 参数
    ///
    /// * `other` - 要合并的CSS对象
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut base = CssObject::new();
    /// base.set("color", "black");
    /// base.set("font-size", 14);
    ///
    /// let mut override_styles = CssObject::new();
    /// override_styles.set("color", "red");
    /// override_styles.set("font-weight", "bold");
    ///
    /// // 合并样式
    /// base.merge(&override_styles);
    ///
    /// // 检查合并结果
    /// assert_eq!(base.get("color").unwrap().as_str(), Some("red")); // 被覆盖
    /// assert_eq!(base.get("font-size").unwrap().as_f64(), Some(14.0)); // 保持不变
    /// assert_eq!(base.get("font-weight").unwrap().as_str(), Some("bold")); // 新增
    /// ```
    pub fn merge(&mut self, other: &CssObject) {
        for (key, value) in &other.properties {
            self.properties.insert(key.clone(), value.clone());
        }
    }

    /// 检查是否为空
    ///
    /// 判断CSS对象是否不包含任何属性。
    ///
    /// # 返回值
    ///
    /// 如果对象不包含任何属性，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    /// assert!(css_obj.is_empty());
    ///
    /// css_obj.set("color", "red");
    /// assert!(!css_obj.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// 获取属性数量
    ///
    /// 返回CSS对象中包含的属性数量。
    ///
    /// # 返回值
    ///
    /// 返回属性的数量。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    /// assert_eq!(css_obj.len(), 0);
    ///
    /// css_obj.set("color", "red");
    /// css_obj.set("font-size", 16);
    /// assert_eq!(css_obj.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// 清空所有属性
    ///
    /// 移除CSS对象中的所有属性。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("color", "red");
    /// css_obj.set("font-size", 16);
    ///
    /// assert_eq!(css_obj.len(), 2);
    ///
    /// css_obj.clear();
    /// assert_eq!(css_obj.len(), 0);
    /// assert!(css_obj.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.properties.clear();
    }
}

impl Default for CssObject {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<String, CssValue>> for CssObject {
    fn from(properties: HashMap<String, CssValue>) -> Self {
        Self { properties }
    }
}

/// 将 JSON 值转换为 CSS 值
///
/// 递归地将JSON值转换为对应的CSS值类型。
///
/// # 参数
///
/// * `value` - 要转换的JSON值
///
/// # 返回值
///
/// 如果转换成功，返回 `Ok(CssValue)`；如果失败，返回错误信息。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{convert_json_to_css_value, CssValue};
/// use serde_json::json;
///
/// let json_value = json!("red");
/// let css_value = convert_json_to_css_value(json_value).unwrap();
/// assert_eq!(css_value.as_str(), Some("red"));
///
/// let json_object = json!({
///     "color": "blue",
///     "size": 16
/// });
/// let css_object = convert_json_to_css_value(json_object).unwrap();
/// ```
fn convert_json_to_css_value(value: Value) -> Result<CssValue, String> {
    match value {
        Value::Null => Ok(CssValue::Null),
        Value::Bool(b) => Ok(CssValue::Bool(b)),
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(CssValue::Number(f))
            } else {
                Err("Invalid number".to_string())
            }
        }
        Value::String(s) => Ok(CssValue::String(s)),
        Value::Array(arr) => {
            let mut css_arr = Vec::new();
            for item in arr {
                css_arr.push(convert_json_to_css_value(item)?);
            }
            Ok(CssValue::Array(css_arr))
        }
        Value::Object(map) => {
            let mut properties = HashMap::new();
            for (key, val) in map {
                properties.insert(key, convert_json_to_css_value(val)?);
            }
            Ok(CssValue::Object(CssObject { properties }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_object_basic() {
        let mut obj = CssObject::new();
        obj.set("color", "red");
        obj.set("font-size", 16);

        assert_eq!(obj.len(), 2);

        if let CssValue::String(color) = obj.get("color").unwrap() {
            assert_eq!(color, "red");
        } else {
            panic!("Expected String value for color");
        }

        if let CssValue::Number(size) = obj.get("font-size").unwrap() {
            assert_eq!(*size, 16.0);
        } else {
            panic!("Expected Number value for font-size");
        }
    }

    #[test]
    fn test_css_object_merge() {
        let mut obj1 = CssObject::new();
        obj1.set("color", "red");

        let mut obj2 = CssObject::new();
        obj2.set("font-size", 16);

        obj1.merge(&obj2);

        assert_eq!(obj1.len(), 2);
        assert!(obj1.get("color").is_some());
        assert!(obj1.get("font-size").is_some());
    }

    #[test]
    fn test_css_object_nested() {
        let mut inner = CssObject::new();
        inner.set("color", "blue");

        let mut obj = CssObject::new();
        obj.set("&:hover", inner);

        if let CssValue::Object(hover) = obj.get("&:hover").unwrap() {
            if let CssValue::String(color) = hover.get("color").unwrap() {
                assert_eq!(color, "blue");
            } else {
                panic!("Expected String value for color");
            }
        } else {
            panic!("Expected Object value for &:hover");
        }
    }
}
