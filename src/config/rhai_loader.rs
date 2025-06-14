use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rhai::{Engine, Scope, Dynamic, Map};

use crate::config::{MainConfig, MainSection, Windows, CustomWidget};

pub fn load_rhai_config<P: AsRef<Path>>(path: P) -> Result<(MainConfig, String), Box<dyn std::error::Error>> {
    let script = fs::read_to_string(&path)?;
    let engine = Engine::new();
    let mut scope = Scope::new();

    let _ = engine.eval_with_scope::<Dynamic>(&mut scope, &script)?;

    let mut config_map = Map::new();
    for (name, _, value) in scope.iter_raw() {
        match name {
            "windows" | "custom" | "boo" => {
                config_map.insert(name.into(), value.clone());
            }
            _ => {}
        }
    }

    let windows = config_map.get("windows")
        .and_then(|v| v.clone().try_cast::<Map>())
        .map(parse_windows)
        .unwrap_or_default();

    let custom = config_map.get("custom")
        .and_then(|v| v.clone().try_cast::<Map>())
        .map(parse_custom)
        .unwrap_or_default();

    let boo = config_map.get("boo")
        .and_then(|v| v.clone().try_cast::<Map>())
        .map(|map| map.into_iter().map(|(k, v)| (k.to_string(), v.into())).collect())
        .unwrap_or_default();

    Ok((MainConfig { windows, custom, boo }, script))
}

fn parse_main_section(map: Map) -> MainSection {
    let windows = map.get("windows")
        .and_then(|v| v.clone().try_cast::<rhai::Array>())
        .map(|arr| {
            arr.into_iter()
                .filter_map(|v| v.try_cast::<String>())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    MainSection { windows }
}

fn parse_windows(map: Map) -> HashMap<String, Windows> {
    let mut windows = HashMap::new();
    for (name, value) in map {
        if let Some(entry) = value.clone().try_cast::<Map>() {
            windows.insert(name.to_string(), Windows {
                width: entry.get("width").and_then(|v| v.clone().try_cast()),
                height: entry.get("height").and_then(|v| v.clone().try_cast()),
                win_type: entry.get("win_type").and_then(|v| v.clone().try_cast()),
                offset_x: entry.get("offset_x").and_then(|v| v.clone().try_cast()),
                position: entry.get("position").and_then(|v| v.clone().try_cast()),
                left_contents: entry.get("left_contents").and_then(|v| v.clone().try_cast()),
                center_contents: entry.get("center_contents").and_then(|v| v.clone().try_cast()),
                right_contents: entry.get("right_contents").and_then(|v| v.clone().try_cast()),
            });
        }
    }
    windows
}

fn parse_custom(map: Map) -> HashMap<String, CustomWidget> {
    let mut custom_widgets = HashMap::new();
    for (name, value) in map {
        if let Some(entry) = value.clone().try_cast::<Map>() {
            custom_widgets.insert(name.to_string(), CustomWidget {
                type_: entry.get("type").and_then(|v| v.clone().try_cast()),
                content: entry.get("content").and_then(|v| v.clone().try_cast()),
            });
        }
    }
    custom_widgets
}
