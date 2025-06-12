pub mod toml_loader;

#[derive(Debug, serde::Deserialize)]
pub struct MainConfig {
    pub main: MainSection,
    pub bars: std::collections::HashMap<String, Bar>,
    pub boo: std::collections::HashMap<String, toml::Value>,
    pub custom: std::collections::HashMap<String, CustomWidget>,
    pub variable: Option<String>,
    pub value_Listened: Option<String>,
    pub value_Polled: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MainSection {
    pub bar: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Bar {
    pub width: Option<String>,
    pub offset_x: Option<String>,
    pub position: Option<String>,
    pub left_contents: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CustomWidget {
    pub time: Option<String>,
    pub type_: Option<String>,
    pub content: Option<String>,
}