//! WASM types and bindings for CSS injection
//! This module centralizes all wasm_bindgen type declarations to avoid conflicts

#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Document;
        pub type Element;
        pub type Node;

        #[wasm_bindgen(method, js_name = createElement)]
        pub fn create_element(this: &Document, tag_name: &str) -> Element;

        #[wasm_bindgen(method, js_name = appendChild)]
        pub fn append_child(this: &Node, child: &Node);

        #[wasm_bindgen(method, setter = innerHTML)]
        pub fn set_inner_html(this: &Element, html: &str);

        #[wasm_bindgen(method, js_name = setAttribute)]
        pub fn set_attribute(this: &Element, name: &str, value: &str);

        #[wasm_bindgen(method, getter = head)]
        pub fn head(this: &Document) -> Element;

        #[wasm_bindgen(method, js_name = getElementById)]
        pub fn get_element_by_id(this: &Document, id: &str) -> Option<Element>;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = document, js_namespace = window)]
        pub static DOCUMENT: Document;
    }

    // Element 到 Node 的转换 - 只定义一次
    impl From<Element> for Node {
        fn from(element: Element) -> Node {
            element.unchecked_into()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod wasm_bindings {
    // For non-WASM targets, provide empty stubs
    pub struct Document;
    pub struct Element;
    pub struct Node;

    impl From<Element> for Node {
        fn from(_element: Element) -> Node {
            Node
        }
    }
}
