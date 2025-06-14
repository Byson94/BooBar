use rhai::{Engine, Scope, Dynamic};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod rhai_loader;

#[derive(Debug, Clone)]
pub struct MainConfig {
    pub windows: HashMap<String, Windows>,
    pub boo: HashMap<String, Dynamic>,
    pub custom: HashMap<String, CustomWidget>,
}

#[derive(Debug, Clone)]
pub struct MainSection {
    pub windows: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Windows {
    pub width: Option<String>,
    pub height: Option<String>,
    pub win_type: Option<String>,
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
