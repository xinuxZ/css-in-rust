use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// CSS 属性值类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CssValue {
    String(String),
    Number(f64),
    Bool(bool),
    Object(CssObject),
    Array(Vec<CssValue>),
    Null,
}

impl From<&str> for CssValue {
    fn from(s: &str) -> Self {
        CssValue::String(s.to_string())
    }
}

impl From<String> for CssValue {
    fn from(s: String) -> Self {
        CssValue::String(s)
    }
}

impl From<f64> for CssValue {
    fn from(n: f64) -> Self {
        CssValue::Number(n)
    }
}

impl From<i32> for CssValue {
    fn from(n: i32) -> Self {
        CssValue::Number(n as f64)
    }
}

impl From<bool> for CssValue {
    fn from(b: bool) -> Self {
        CssValue::Bool(b)
    }
}

impl From<CssObject> for CssValue {
    fn from(o: CssObject) -> Self {
        CssValue::Object(o)
    }
}

impl<T> From<Vec<T>> for CssValue
where
    T: Into<CssValue>,
{
    fn from(v: Vec<T>) -> Self {
        CssValue::Array(v.into_iter().map(|item| item.into()).collect())
    }
}

impl CssValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            CssValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CssValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CssValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&CssObject> {
        match self {
            CssValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<CssValue>> {
        match self {
            CssValue::Array(a) => Some(a),
            _ => None,
        }
    }
}

/// CSS 对象，表示一组 CSS 属性或嵌套规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CssObject {
    pub properties: HashMap<String, CssValue>,
}

impl CssObject {
    /// 创建新的空 CSS 对象
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// 从 JSON 值创建 CSS 对象
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
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<CssValue>,
    {
        self.properties.insert(key.into(), value.into());
    }

    /// 获取 CSS 属性
    pub fn get(&self, key: &str) -> Option<&CssValue> {
        self.properties.get(key)
    }

    /// 移除 CSS 属性
    pub fn remove(&mut self, key: &str) -> Option<CssValue> {
        self.properties.remove(key)
    }

    /// 合并另一个 CSS 对象到当前对象
    pub fn merge(&mut self, other: &CssObject) {
        for (key, value) in &other.properties {
            self.properties.insert(key.clone(), value.clone());
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// 获取属性数量
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// 清空所有属性
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
        obj.insert("color", "red");
        obj.insert("font-size", 16);

        assert_eq!(obj.len(), 2);

        if let CssValue::String(color) = obj.get("color").unwrap() {
            assert_eq!(color, "red");
        } else {
            panic!("Expected String value for color");
        }

        if let CssValue::Number(size) = obj.get("font-size").unwrap() {
            assert_eq!(size, 16.0);
        } else {
            panic!("Expected Number value for font-size");
        }
    }

    #[test]
    fn test_css_object_merge() {
        let mut obj1 = CssObject::new();
        obj1.insert("color", "red");

        let mut obj2 = CssObject::new();
        obj2.insert("font-size", 16);

        obj1.merge(&obj2);

        assert_eq!(obj1.len(), 2);
        assert!(obj1.get("color").is_some());
        assert!(obj1.get("font-size").is_some());
    }

    #[test]
    fn test_css_object_nested() {
        let mut inner = CssObject::new();
        inner.insert("color", "blue");

        let mut obj = CssObject::new();
        obj.insert("&:hover", inner);

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
