use std::fs;
use std::path::Path;
use std::collections::HashMap;

use mlua::{Lua, Value, Table};

use crate::config::{MainConfig, MainSection, Bar, CustomWidget};

pub fn load_lua_config<P: AsRef<Path>>(path: P) -> mlua::Result<(MainConfig, String)> {
    let script = fs::read_to_string(path)?;
    let lua = Lua::new();
    lua.globals().set("poll", lua.create_function(|_, _: (String, String)| Ok(()))?)?;
    lua.load(&script).exec()?;

    let globals = lua.globals();

    let main = match globals.get::<Option<Table>>("main")? {
        Some(t) => Some(parse_main_section(t)),
        None => None,
    };

    let bars = match globals.get::<Option<Table>>("bars")? {
        Some(t) => parse_bars(t),
        None => HashMap::new(),
    };

    let custom = match globals.get::<Option<Table>>("custom")? {
        Some(t) => parse_custom(t),
        None => HashMap::new(),
    };

    let boo = match globals.get::<Option<Table>>("boo")? {
        Some(t) => t.pairs::<String, Value>().collect::<mlua::Result<HashMap<_, _>>>()?,
        None => HashMap::new(),
    };

    Ok((MainConfig { main, bars, custom, boo }, script))
}

fn parse_main_section(table: Table) -> MainSection {
    let bar = table.get::<Option<Vec<String>>>("bar").unwrap_or_default();
    MainSection { bar: bar.expect("missing bar array") }
}

fn parse_bars(table: Table) -> HashMap<String, Bar> {
    let mut bars = HashMap::new();
    for pair in table.pairs::<String, Table>() {
        if let Ok((name, entry)) = pair {
            bars.insert(name, Bar {
                width: entry.get("width").ok(),
                offset_x: entry.get("offset_x").ok(),
                position: entry.get("position").ok(),
                left_contents: entry.get("left_contents").ok(),
                center_contents: entry.get("center_contents").ok(),
                right_contents: entry.get("right_contents").ok(),
            });
        }
    }
    bars
}

fn parse_custom(table: Table) -> HashMap<String, CustomWidget> {
    let mut custom_widgets = HashMap::new();
    for pair in table.pairs::<String, Table>() {
        if let Ok((name, entry)) = pair {
            custom_widgets.insert(name, CustomWidget {
                type_: entry.get("type").ok(),
                content: entry.get("content").ok(),
            });
        }
    }
    custom_widgets
}
