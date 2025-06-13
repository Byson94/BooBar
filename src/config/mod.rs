use mlua::Lua;
use std::collections::HashMap;

pub mod lua_loader;

#[derive(Debug, Clone)]
pub struct MainConfig {
    pub main: Option<MainSection>,
    pub bars: HashMap<String, Bar>,
    pub boo: HashMap<String, mlua::Value>,
    pub custom: HashMap<String, CustomWidget>,
}

#[derive(Debug, Clone)]
pub struct MainSection {
    pub bar: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub width: Option<String>,
    pub offset_x: Option<String>,
    pub position: Option<String>,
    pub left_contents: Option<String>,
    pub center_contents: Option<String>,
    pub right_contents: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CustomWidget {
    pub type_: Option<String>,
    pub content: Option<String>,
}
