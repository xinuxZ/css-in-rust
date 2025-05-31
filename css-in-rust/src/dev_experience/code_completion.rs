//! 代码补全模块
//!
//! 提供CSS代码的智能补全功能

use std::collections::HashMap;
use std::fmt;

/// 补全项类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompletionItemKind {
    /// CSS属性
    Property,
    /// CSS值
    Value,
    /// CSS选择器
    Selector,
    /// CSS函数
    Function,
    /// CSS单位
    Unit,
    /// CSS颜色
    Color,
    /// CSS关键字
    Keyword,
    /// CSS变量
    Variable,
    /// CSS类名
    ClassName,
    /// CSS ID
    Id,
    /// CSS伪类
    PseudoClass,
    /// CSS伪元素
    PseudoElement,
    /// CSS媒体查询
    MediaQuery,
    /// CSS动画
    Animation,
    /// 代码片段
    Snippet,
}

/// 补全项
#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// 标签（显示文本）
    pub label: String,
    /// 种类
    pub kind: CompletionItemKind,
    /// 详细信息
    pub detail: Option<String>,
    /// 文档
    pub documentation: Option<String>,
    /// 插入文本
    pub insert_text: String,
    /// 过滤文本
    pub filter_text: Option<String>,
    /// 排序文本
    pub sort_text: Option<String>,
    /// 是否预选
    pub preselect: bool,
    /// 额外的编辑
    pub additional_text_edits: Vec<TextEdit>,
    /// 命令
    pub command: Option<Command>,
}

/// 文本编辑
#[derive(Debug, Clone)]
pub struct TextEdit {
    /// 范围
    pub range: Range,
    /// 新文本
    pub new_text: String,
}

/// 范围
#[derive(Debug, Clone)]
pub struct Range {
    /// 开始位置
    pub start: Position,
    /// 结束位置
    pub end: Position,
}

/// 位置
#[derive(Debug, Clone)]
pub struct Position {
    /// 行号（从0开始）
    pub line: usize,
    /// 列号（从0开始）
    pub character: usize,
}

/// 命令
#[derive(Debug, Clone)]
pub struct Command {
    /// 命令标题
    pub title: String,
    /// 命令ID
    pub command: String,
    /// 参数
    pub arguments: Vec<String>,
}

/// 补全上下文
#[derive(Debug, Clone)]
pub struct CompletionContext {
    /// 触发类型
    pub trigger_kind: CompletionTriggerKind,
    /// 触发字符
    pub trigger_character: Option<char>,
}

/// 补全触发类型
#[derive(Debug, Clone, PartialEq)]
pub enum CompletionTriggerKind {
    /// 调用补全
    Invoked,
    /// 触发字符
    TriggerCharacter,
    /// 不完整补全
    TriggerForIncompleteCompletions,
}

/// 代码补全提供者
pub struct CompletionProvider {
    /// CSS属性补全项
    properties: HashMap<String, CompletionItem>,
    /// CSS值补全项
    values: HashMap<String, Vec<CompletionItem>>,
    /// CSS选择器补全项
    selectors: Vec<CompletionItem>,
    /// CSS函数补全项
    functions: Vec<CompletionItem>,
    /// CSS单位补全项
    units: Vec<CompletionItem>,
    /// CSS颜色补全项
    colors: Vec<CompletionItem>,
    /// CSS关键字补全项
    keywords: Vec<CompletionItem>,
    /// 代码片段
    snippets: Vec<CompletionItem>,
}

impl CompletionProvider {
    /// 创建新的补全提供者
    pub fn new() -> Self {
        let mut provider = Self {
            properties: HashMap::new(),
            values: HashMap::new(),
            selectors: Vec::new(),
            functions: Vec::new(),
            units: Vec::new(),
            colors: Vec::new(),
            keywords: Vec::new(),
            snippets: Vec::new(),
        };

        provider.initialize_completions();
        provider
    }

    /// 初始化补全项
    fn initialize_completions(&mut self) {
        self.initialize_properties();
        self.initialize_values();
        self.initialize_selectors();
        self.initialize_functions();
        self.initialize_units();
        self.initialize_colors();
        self.initialize_keywords();
        self.initialize_snippets();
    }

    /// 初始化CSS属性补全
    fn initialize_properties(&mut self) {
        let properties = vec![
            ("color", "设置文本颜色", "color: "),
            ("background", "设置背景", "background: "),
            ("background-color", "设置背景颜色", "background-color: "),
            ("background-image", "设置背景图片", "background-image: "),
            ("background-size", "设置背景尺寸", "background-size: "),
            (
                "background-position",
                "设置背景位置",
                "background-position: ",
            ),
            ("background-repeat", "设置背景重复", "background-repeat: "),
            ("border", "设置边框", "border: "),
            ("border-color", "设置边框颜色", "border-color: "),
            ("border-width", "设置边框宽度", "border-width: "),
            ("border-style", "设置边框样式", "border-style: "),
            ("border-radius", "设置边框圆角", "border-radius: "),
            ("margin", "设置外边距", "margin: "),
            ("margin-top", "设置上外边距", "margin-top: "),
            ("margin-right", "设置右外边距", "margin-right: "),
            ("margin-bottom", "设置下外边距", "margin-bottom: "),
            ("margin-left", "设置左外边距", "margin-left: "),
            ("padding", "设置内边距", "padding: "),
            ("padding-top", "设置上内边距", "padding-top: "),
            ("padding-right", "设置右内边距", "padding-right: "),
            ("padding-bottom", "设置下内边距", "padding-bottom: "),
            ("padding-left", "设置左内边距", "padding-left: "),
            ("width", "设置宽度", "width: "),
            ("height", "设置高度", "height: "),
            ("max-width", "设置最大宽度", "max-width: "),
            ("max-height", "设置最大高度", "max-height: "),
            ("min-width", "设置最小宽度", "min-width: "),
            ("min-height", "设置最小高度", "min-height: "),
            ("display", "设置显示类型", "display: "),
            ("position", "设置定位类型", "position: "),
            ("top", "设置顶部位置", "top: "),
            ("right", "设置右侧位置", "right: "),
            ("bottom", "设置底部位置", "bottom: "),
            ("left", "设置左侧位置", "left: "),
            ("z-index", "设置层级", "z-index: "),
            ("float", "设置浮动", "float: "),
            ("clear", "设置清除浮动", "clear: "),
            ("overflow", "设置溢出处理", "overflow: "),
            ("overflow-x", "设置水平溢出处理", "overflow-x: "),
            ("overflow-y", "设置垂直溢出处理", "overflow-y: "),
            ("visibility", "设置可见性", "visibility: "),
            ("opacity", "设置透明度", "opacity: "),
            ("font-family", "设置字体族", "font-family: "),
            ("font-size", "设置字体大小", "font-size: "),
            ("font-weight", "设置字体粗细", "font-weight: "),
            ("font-style", "设置字体样式", "font-style: "),
            ("line-height", "设置行高", "line-height: "),
            ("text-align", "设置文本对齐", "text-align: "),
            ("text-decoration", "设置文本装饰", "text-decoration: "),
            ("text-transform", "设置文本转换", "text-transform: "),
            ("letter-spacing", "设置字符间距", "letter-spacing: "),
            ("word-spacing", "设置单词间距", "word-spacing: "),
            ("white-space", "设置空白处理", "white-space: "),
            ("vertical-align", "设置垂直对齐", "vertical-align: "),
            ("cursor", "设置鼠标样式", "cursor: "),
            ("transition", "设置过渡效果", "transition: "),
            ("transform", "设置变换", "transform: "),
            ("animation", "设置动画", "animation: "),
            ("box-shadow", "设置盒子阴影", "box-shadow: "),
            ("text-shadow", "设置文本阴影", "text-shadow: "),
            ("flex", "设置弹性项目", "flex: "),
            ("flex-direction", "设置弹性方向", "flex-direction: "),
            ("flex-wrap", "设置弹性换行", "flex-wrap: "),
            ("justify-content", "设置主轴对齐", "justify-content: "),
            ("align-items", "设置交叉轴对齐", "align-items: "),
            ("align-content", "设置多行对齐", "align-content: "),
            ("grid", "设置网格", "grid: "),
            (
                "grid-template-columns",
                "设置网格列模板",
                "grid-template-columns: ",
            ),
            (
                "grid-template-rows",
                "设置网格行模板",
                "grid-template-rows: ",
            ),
            ("grid-gap", "设置网格间距", "grid-gap: "),
        ];

        for (name, description, insert_text) in properties {
            let item = CompletionItem {
                label: name.to_string(),
                kind: CompletionItemKind::Property,
                detail: Some(format!("CSS属性: {}", name)),
                documentation: Some(description.to_string()),
                insert_text: insert_text.to_string(),
                filter_text: Some(name.to_string()),
                sort_text: Some(format!("0{}", name)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.properties.insert(name.to_string(), item);
        }
    }

    /// 初始化CSS值补全
    fn initialize_values(&mut self) {
        // display 属性值
        let display_values = vec![
            "block",
            "inline",
            "inline-block",
            "flex",
            "inline-flex",
            "grid",
            "inline-grid",
            "table",
            "table-cell",
            "table-row",
            "none",
            "contents",
            "list-item",
            "run-in",
        ];
        self.add_property_values("display", &display_values);

        // position 属性值
        let position_values = vec!["static", "relative", "absolute", "fixed", "sticky"];
        self.add_property_values("position", &position_values);

        // text-align 属性值
        let text_align_values = vec!["left", "right", "center", "justify", "start", "end"];
        self.add_property_values("text-align", &text_align_values);

        // font-weight 属性值
        let font_weight_values = vec![
            "normal", "bold", "bolder", "lighter", "100", "200", "300", "400", "500", "600", "700",
            "800", "900",
        ];
        self.add_property_values("font-weight", &font_weight_values);

        // overflow 属性值
        let overflow_values = vec!["visible", "hidden", "scroll", "auto", "clip"];
        self.add_property_values("overflow", &overflow_values);
        self.add_property_values("overflow-x", &overflow_values);
        self.add_property_values("overflow-y", &overflow_values);

        // flex-direction 属性值
        let flex_direction_values = vec!["row", "row-reverse", "column", "column-reverse"];
        self.add_property_values("flex-direction", &flex_direction_values);

        // justify-content 属性值
        let justify_content_values = vec![
            "flex-start",
            "flex-end",
            "center",
            "space-between",
            "space-around",
            "space-evenly",
            "start",
            "end",
        ];
        self.add_property_values("justify-content", &justify_content_values);

        // align-items 属性值
        let align_items_values = vec![
            "stretch",
            "flex-start",
            "flex-end",
            "center",
            "baseline",
            "start",
            "end",
            "self-start",
            "self-end",
        ];
        self.add_property_values("align-items", &align_items_values);
    }

    /// 为属性添加值补全
    fn add_property_values(&mut self, property: &str, values: &[&str]) {
        let completion_items: Vec<CompletionItem> = values
            .iter()
            .map(|value| CompletionItem {
                label: value.to_string(),
                kind: CompletionItemKind::Value,
                detail: Some(format!("{} 的值", property)),
                documentation: Some(format!("CSS属性 {} 的可选值: {}", property, value)),
                insert_text: value.to_string(),
                filter_text: Some(value.to_string()),
                sort_text: Some(format!("1{}", value)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            })
            .collect();

        self.values.insert(property.to_string(), completion_items);
    }

    /// 初始化选择器补全
    fn initialize_selectors(&mut self) {
        let selectors = vec![
            (":hover", "鼠标悬停伪类"),
            (":focus", "获得焦点伪类"),
            (":active", "激活状态伪类"),
            (":visited", "已访问链接伪类"),
            (":first-child", "第一个子元素伪类"),
            (":last-child", "最后一个子元素伪类"),
            (":nth-child()", "第n个子元素伪类"),
            (":nth-of-type()", "第n个同类型元素伪类"),
            (":not()", "否定伪类"),
            ("::before", "前置伪元素"),
            ("::after", "后置伪元素"),
            ("::first-line", "首行伪元素"),
            ("::first-letter", "首字母伪元素"),
        ];

        for (selector, description) in selectors {
            let item = CompletionItem {
                label: selector.to_string(),
                kind: if selector.starts_with("::") {
                    CompletionItemKind::PseudoElement
                } else {
                    CompletionItemKind::PseudoClass
                },
                detail: Some("CSS选择器".to_string()),
                documentation: Some(description.to_string()),
                insert_text: selector.to_string(),
                filter_text: Some(selector.to_string()),
                sort_text: Some(format!("2{}", selector)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.selectors.push(item);
        }
    }

    /// 初始化函数补全
    fn initialize_functions(&mut self) {
        let functions = vec![
            ("rgb()", "RGB颜色函数", "rgb(${1:255}, ${2:255}, ${3:255})"),
            (
                "rgba()",
                "RGBA颜色函数",
                "rgba(${1:255}, ${2:255}, ${3:255}, ${4:1})",
            ),
            ("hsl()", "HSL颜色函数", "hsl(${1:0}, ${2:100%}, ${3:50%})"),
            (
                "hsla()",
                "HSLA颜色函数",
                "hsla(${1:0}, ${2:100%}, ${3:50%}, ${4:1})",
            ),
            ("url()", "URL函数", "url(${1:path})"),
            ("calc()", "计算函数", "calc(${1:expression})"),
            ("var()", "CSS变量函数", "var(${1:--variable-name})"),
            (
                "linear-gradient()",
                "线性渐变函数",
                "linear-gradient(${1:to right}, ${2:#000}, ${3:#fff})",
            ),
            (
                "radial-gradient()",
                "径向渐变函数",
                "radial-gradient(${1:circle}, ${2:#000}, ${3:#fff})",
            ),
            ("translate()", "平移变换函数", "translate(${1:0}, ${2:0})"),
            ("translateX()", "X轴平移函数", "translateX(${1:0})"),
            ("translateY()", "Y轴平移函数", "translateY(${1:0})"),
            ("scale()", "缩放变换函数", "scale(${1:1})"),
            ("scaleX()", "X轴缩放函数", "scaleX(${1:1})"),
            ("scaleY()", "Y轴缩放函数", "scaleY(${1:1})"),
            ("rotate()", "旋转变换函数", "rotate(${1:0deg})"),
            ("skew()", "倾斜变换函数", "skew(${1:0deg}, ${2:0deg})"),
            (
                "matrix()",
                "矩阵变换函数",
                "matrix(${1:1}, ${2:0}, ${3:0}, ${4:1}, ${5:0}, ${6:0})",
            ),
        ];

        for (name, description, insert_text) in functions {
            let item = CompletionItem {
                label: name.to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("CSS函数".to_string()),
                documentation: Some(description.to_string()),
                insert_text: insert_text.to_string(),
                filter_text: Some(name.to_string()),
                sort_text: Some(format!("3{}", name)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.functions.push(item);
        }
    }

    /// 初始化单位补全
    fn initialize_units(&mut self) {
        let units = vec![
            ("px", "像素单位"),
            ("em", "相对于父元素字体大小的单位"),
            ("rem", "相对于根元素字体大小的单位"),
            ("%", "百分比单位"),
            ("vh", "视口高度单位"),
            ("vw", "视口宽度单位"),
            ("vmin", "视口最小尺寸单位"),
            ("vmax", "视口最大尺寸单位"),
            ("pt", "点单位"),
            ("pc", "派卡单位"),
            ("in", "英寸单位"),
            ("cm", "厘米单位"),
            ("mm", "毫米单位"),
            ("deg", "角度单位（度）"),
            ("rad", "角度单位（弧度）"),
            ("grad", "角度单位（梯度）"),
            ("turn", "角度单位（圈）"),
            ("s", "时间单位（秒）"),
            ("ms", "时间单位（毫秒）"),
            ("Hz", "频率单位（赫兹）"),
            ("kHz", "频率单位（千赫兹）"),
            ("dpi", "分辨率单位（每英寸点数）"),
            ("dpcm", "分辨率单位（每厘米点数）"),
            ("dppx", "分辨率单位（每像素点数）"),
        ];

        for (unit, description) in units {
            let item = CompletionItem {
                label: unit.to_string(),
                kind: CompletionItemKind::Unit,
                detail: Some("CSS单位".to_string()),
                documentation: Some(description.to_string()),
                insert_text: unit.to_string(),
                filter_text: Some(unit.to_string()),
                sort_text: Some(format!("4{}", unit)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.units.push(item);
        }
    }

    /// 初始化颜色补全
    fn initialize_colors(&mut self) {
        let colors = vec![
            ("red", "#ff0000"),
            ("green", "#008000"),
            ("blue", "#0000ff"),
            ("white", "#ffffff"),
            ("black", "#000000"),
            ("gray", "#808080"),
            ("yellow", "#ffff00"),
            ("orange", "#ffa500"),
            ("purple", "#800080"),
            ("pink", "#ffc0cb"),
            ("brown", "#a52a2a"),
            ("cyan", "#00ffff"),
            ("magenta", "#ff00ff"),
            ("lime", "#00ff00"),
            ("navy", "#000080"),
            ("maroon", "#800000"),
            ("olive", "#808000"),
            ("teal", "#008080"),
            ("silver", "#c0c0c0"),
            ("transparent", "透明色"),
            ("currentColor", "当前颜色"),
            ("inherit", "继承颜色"),
            ("initial", "初始颜色"),
            ("unset", "未设置颜色"),
        ];

        for (name, value) in colors {
            let item = CompletionItem {
                label: name.to_string(),
                kind: CompletionItemKind::Color,
                detail: Some(format!("颜色: {}", value)),
                documentation: Some(format!("CSS颜色 {} ({})", name, value)),
                insert_text: name.to_string(),
                filter_text: Some(name.to_string()),
                sort_text: Some(format!("5{}", name)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.colors.push(item);
        }
    }

    /// 初始化关键字补全
    fn initialize_keywords(&mut self) {
        let keywords = vec![
            ("important", "!important 声明"),
            ("auto", "自动值"),
            ("none", "无值"),
            ("normal", "正常值"),
            ("inherit", "继承值"),
            ("initial", "初始值"),
            ("unset", "未设置值"),
            ("revert", "恢复值"),
        ];

        for (keyword, description) in keywords {
            let item = CompletionItem {
                label: keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("CSS关键字".to_string()),
                documentation: Some(description.to_string()),
                insert_text: keyword.to_string(),
                filter_text: Some(keyword.to_string()),
                sort_text: Some(format!("6{}", keyword)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.keywords.push(item);
        }
    }

    /// 初始化代码片段
    fn initialize_snippets(&mut self) {
        let snippets = vec![
            (
                "flexbox",
                "Flexbox布局",
                "display: flex;\njustify-content: ${1:center};\nalign-items: ${2:center};",
            ),
            (
                "grid",
                "Grid布局",
                "display: grid;\ngrid-template-columns: ${1:repeat(3, 1fr)};\ngrid-gap: ${2:1rem};",
            ),
            (
                "center",
                "居中布局",
                "display: flex;\njustify-content: center;\nalign-items: center;",
            ),
            (
                "transition",
                "过渡动画",
                "transition: ${1:all} ${2:0.3s} ${3:ease};",
            ),
            (
                "animation",
                "关键帧动画",
                "animation: ${1:name} ${2:1s} ${3:ease} ${4:infinite};",
            ),
            (
                "media",
                "媒体查询",
                "@media (${1:max-width}: ${2:768px}) {\n\t$0\n}",
            ),
            (
                "keyframes",
                "关键帧定义",
                "@keyframes ${1:name} {\n\t0% {\n\t\t$2\n\t}\n\t100% {\n\t\t$3\n\t}\n}",
            ),
            (
                "gradient",
                "线性渐变",
                "background: linear-gradient(${1:to right}, ${2:#000}, ${3:#fff});",
            ),
            (
                "shadow",
                "盒子阴影",
                "box-shadow: ${1:0} ${2:2px} ${3:4px} ${4:rgba(0, 0, 0, 0.1)};",
            ),
            ("border-radius", "圆角边框", "border-radius: ${1:4px};"),
        ];

        for (name, description, insert_text) in snippets {
            let item = CompletionItem {
                label: name.to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("代码片段".to_string()),
                documentation: Some(description.to_string()),
                insert_text: insert_text.to_string(),
                filter_text: Some(name.to_string()),
                sort_text: Some(format!("7{}", name)),
                preselect: false,
                additional_text_edits: Vec::new(),
                command: None,
            };
            self.snippets.push(item);
        }
    }

    /// 获取补全项
    pub fn get_completions(&self, text: &str, position: usize) -> Vec<CompletionItem> {
        let context = self.analyze_context(text, position);
        let mut completions = Vec::new();

        match context.completion_type {
            CompletionType::Property => {
                // 属性补全
                completions.extend(self.get_property_completions(&context.prefix));
            }
            CompletionType::Value(ref property) => {
                // 值补全
                completions.extend(self.get_value_completions(property, &context.prefix));
            }
            CompletionType::Selector => {
                // 选择器补全
                completions.extend(self.get_selector_completions(&context.prefix));
            }
            CompletionType::Function => {
                // 函数补全
                completions.extend(self.get_function_completions(&context.prefix));
            }
            CompletionType::Unit => {
                // 单位补全
                completions.extend(self.get_unit_completions(&context.prefix));
            }
            CompletionType::Color => {
                // 颜色补全
                completions.extend(self.get_color_completions(&context.prefix));
            }
            CompletionType::Snippet => {
                // 代码片段补全
                completions.extend(self.get_snippet_completions(&context.prefix));
            }
            CompletionType::General => {
                // 通用补全
                completions.extend(self.get_general_completions(&context.prefix));
            }
        }

        // 过滤和排序
        self.filter_and_sort_completions(completions, &context.prefix)
    }

    /// 分析补全上下文
    fn analyze_context(&self, text: &str, position: usize) -> CompletionAnalysisContext {
        let before_cursor = &text[..position.min(text.len())];
        let lines: Vec<&str> = before_cursor.split('\n').collect();
        let current_line = lines.last().unwrap_or(&"");

        // 获取当前单词前缀
        let prefix = self.get_word_prefix(current_line);

        // 分析补全类型
        let completion_type = if self.is_in_property_context(current_line) {
            CompletionType::Property
        } else if let Some(property) = self.get_current_property(current_line) {
            CompletionType::Value(property)
        } else if self.is_in_selector_context(before_cursor) {
            CompletionType::Selector
        } else if self.is_in_function_context(current_line) {
            CompletionType::Function
        } else if self.is_in_unit_context(current_line) {
            CompletionType::Unit
        } else if self.is_in_color_context(current_line) {
            CompletionType::Color
        } else {
            CompletionType::General
        };

        CompletionAnalysisContext {
            prefix,
            completion_type,
            line: current_line.to_string(),
            position_in_line: current_line.len(),
        }
    }

    /// 获取单词前缀
    fn get_word_prefix(&self, line: &str) -> String {
        let chars: Vec<char> = line.chars().collect();
        let mut start = chars.len();

        for (i, &ch) in chars.iter().enumerate().rev() {
            if ch.is_whitespace() || ch == ':' || ch == ';' || ch == '{' || ch == '}' {
                break;
            }
            start = i;
        }

        chars[start..].iter().collect()
    }

    /// 检查是否在属性上下文中
    fn is_in_property_context(&self, line: &str) -> bool {
        let trimmed = line.trim();
        !trimmed.contains(':') || trimmed.ends_with('{')
    }

    /// 获取当前属性名
    fn get_current_property(&self, line: &str) -> Option<String> {
        if let Some(colon_pos) = line.find(':') {
            let property = line[..colon_pos].trim();
            if !property.is_empty() {
                return Some(property.to_string());
            }
        }
        None
    }

    /// 检查是否在选择器上下文中
    fn is_in_selector_context(&self, text: &str) -> bool {
        // 简单检查：如果不在大括号内，则认为是选择器上下文
        let open_braces = text.matches('{').count();
        let close_braces = text.matches('}').count();
        open_braces <= close_braces
    }

    /// 检查是否在函数上下文中
    fn is_in_function_context(&self, line: &str) -> bool {
        line.contains('(') && !line.contains(')')
    }

    /// 检查是否在单位上下文中
    fn is_in_unit_context(&self, line: &str) -> bool {
        // 检查是否有数字后跟字母
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len().saturating_sub(1) {
            if chars[i].is_ascii_digit() && chars[i + 1].is_ascii_alphabetic() {
                return true;
            }
        }
        false
    }

    /// 检查是否在颜色上下文中
    fn is_in_color_context(&self, line: &str) -> bool {
        line.contains("color") || line.contains("background") || line.contains("border")
    }

    /// 获取属性补全
    fn get_property_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.properties
            .values()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取值补全
    fn get_value_completions(&self, property: &str, prefix: &str) -> Vec<CompletionItem> {
        if let Some(values) = self.values.get(property) {
            values
                .iter()
                .filter(|item| item.label.starts_with(prefix))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 获取选择器补全
    fn get_selector_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.selectors
            .iter()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取函数补全
    fn get_function_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.functions
            .iter()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取单位补全
    fn get_unit_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.units
            .iter()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取颜色补全
    fn get_color_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.colors
            .iter()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取代码片段补全
    fn get_snippet_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.snippets
            .iter()
            .filter(|item| item.label.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// 获取通用补全
    fn get_general_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // 添加所有类型的补全项
        completions.extend(self.get_property_completions(prefix));
        completions.extend(self.get_selector_completions(prefix));
        completions.extend(self.get_function_completions(prefix));
        completions.extend(self.get_unit_completions(prefix));
        completions.extend(self.get_color_completions(prefix));
        completions.extend(self.get_snippet_completions(prefix));

        completions
    }

    /// 过滤和排序补全项
    fn filter_and_sort_completions(
        &self,
        mut completions: Vec<CompletionItem>,
        prefix: &str,
    ) -> Vec<CompletionItem> {
        // 过滤匹配的项
        completions.retain(|item| {
            item.label.to_lowercase().contains(&prefix.to_lowercase())
                || item
                    .filter_text
                    .as_ref()
                    .map(|text| text.to_lowercase().contains(&prefix.to_lowercase()))
                    .unwrap_or(false)
        });

        // 排序
        completions.sort_by(|a, b| {
            // 首先按照是否精确匹配排序
            let a_exact = a.label.starts_with(prefix);
            let b_exact = b.label.starts_with(prefix);

            match (a_exact, b_exact) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => {
                    // 然后按照排序文本排序
                    a.sort_text
                        .as_ref()
                        .unwrap_or(&a.label)
                        .cmp(b.sort_text.as_ref().unwrap_or(&b.label))
                }
            }
        });

        // 限制返回数量
        completions.truncate(50);

        completions
    }

    /// 添加自定义补全项
    pub fn add_custom_completion(&mut self, item: CompletionItem) {
        match item.kind {
            CompletionItemKind::Property => {
                self.properties.insert(item.label.clone(), item);
            }
            CompletionItemKind::Selector
            | CompletionItemKind::PseudoClass
            | CompletionItemKind::PseudoElement => {
                self.selectors.push(item);
            }
            CompletionItemKind::Function => {
                self.functions.push(item);
            }
            CompletionItemKind::Unit => {
                self.units.push(item);
            }
            CompletionItemKind::Color => {
                self.colors.push(item);
            }
            CompletionItemKind::Keyword => {
                self.keywords.push(item);
            }
            CompletionItemKind::Snippet => {
                self.snippets.push(item);
            }
            _ => {
                // 其他类型暂不支持
            }
        }
    }

    /// 移除自定义补全项
    pub fn remove_custom_completion(&mut self, label: &str, kind: CompletionItemKind) {
        match kind {
            CompletionItemKind::Property => {
                self.properties.remove(label);
            }
            CompletionItemKind::Selector
            | CompletionItemKind::PseudoClass
            | CompletionItemKind::PseudoElement => {
                self.selectors.retain(|item| item.label != label);
            }
            CompletionItemKind::Function => {
                self.functions.retain(|item| item.label != label);
            }
            CompletionItemKind::Unit => {
                self.units.retain(|item| item.label != label);
            }
            CompletionItemKind::Color => {
                self.colors.retain(|item| item.label != label);
            }
            CompletionItemKind::Keyword => {
                self.keywords.retain(|item| item.label != label);
            }
            CompletionItemKind::Snippet => {
                self.snippets.retain(|item| item.label != label);
            }
            _ => {
                // 其他类型暂不支持
            }
        }
    }

    /// 获取补全统计信息
    pub fn get_completion_stats(&self) -> CompletionStats {
        CompletionStats {
            total_properties: self.properties.len(),
            total_values: self.values.values().map(|v| v.len()).sum(),
            total_selectors: self.selectors.len(),
            total_functions: self.functions.len(),
            total_units: self.units.len(),
            total_colors: self.colors.len(),
            total_keywords: self.keywords.len(),
            total_snippets: self.snippets.len(),
        }
    }
}

/// 补全类型
#[derive(Debug, Clone)]
enum CompletionType {
    /// 属性补全
    Property,
    /// 值补全
    Value(String),
    /// 选择器补全
    Selector,
    /// 函数补全
    Function,
    /// 单位补全
    Unit,
    /// 颜色补全
    Color,
    /// 代码片段补全
    Snippet,
    /// 通用补全
    General,
}

/// 补全分析上下文
#[derive(Debug, Clone)]
struct CompletionAnalysisContext {
    /// 前缀
    prefix: String,
    /// 补全类型
    completion_type: CompletionType,
    /// 当前行
    line: String,
    /// 行内位置
    position_in_line: usize,
}

/// 补全统计信息
#[derive(Debug, Clone)]
pub struct CompletionStats {
    /// 属性总数
    pub total_properties: usize,
    /// 值总数
    pub total_values: usize,
    /// 选择器总数
    pub total_selectors: usize,
    /// 函数总数
    pub total_functions: usize,
    /// 单位总数
    pub total_units: usize,
    /// 颜色总数
    pub total_colors: usize,
    /// 关键字总数
    pub total_keywords: usize,
    /// 代码片段总数
    pub total_snippets: usize,
}

/// 补全错误
#[derive(Debug, Clone)]
pub enum CompletionError {
    /// 无效的位置
    InvalidPosition(usize),
    /// 解析错误
    ParseError(String),
    /// 上下文分析失败
    ContextAnalysisFailed(String),
}

impl fmt::Display for CompletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionError::InvalidPosition(pos) => {
                write!(f, "无效的位置: {}", pos)
            }
            CompletionError::ParseError(msg) => {
                write!(f, "解析错误: {}", msg)
            }
            CompletionError::ContextAnalysisFailed(msg) => {
                write!(f, "上下文分析失败: {}", msg)
            }
        }
    }
}

impl std::error::Error for CompletionError {}

impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}
