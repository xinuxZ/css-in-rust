//! Dioxus框架适配
//!
//! 本模块提供与Dioxus框架的集成功能。

#[cfg(feature = "dioxus")]
pub mod dioxus;

// Re-export adapter functionality
#[cfg(feature = "dioxus")]
pub use dioxus::*;

/// Dioxus框架适配器接口
pub trait DioxusAdapterTrait {
    /// 组件类型
    type Component;

    /// 应用CSS类到组件
    fn apply_class(component: &mut Self::Component, class_name: &str);

    /// 获取组件当前的类名
    fn get_classes(component: &Self::Component) -> Vec<String>;
}

/// Generic CSS application helper
pub fn apply_css_class<T: DioxusAdapterTrait>(component: &mut T::Component, class_name: &str) {
    T::apply_class(component, class_name);
}
