use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use mlua::{Lua, Table, Value, Result as LuaResult};

use crate::config::{MainConfig, MainSection, CustomWidget, Bar};

#[derive(Clone)]
pub struct Runtime {
    lua: Lua,
    custom_widgets: Arc<Mutex<HashMap<String, CustomWidget>>>,
    bars: Arc<Mutex<HashMap<String, Bar>>>,
    main_section: Arc<Mutex<Option<MainSection>>>,
    boo: Arc<Mutex<HashMap<String, Value>>>,
}

impl Runtime {
    pub fn from_script<P: AsRef<Path>>(path: P) -> Result<(Arc<Self>, MainConfig), Box<dyn std::error::Error>> {
        let script = fs::read_to_string(&path)?;
        let lua = Lua::new();

        let rt = Arc::new(Self {
            lua,
            custom_widgets: Arc::new(Mutex::new(HashMap::new())),
            bars: Arc::new(Mutex::new(HashMap::new())),
            main_section: Arc::new(Mutex::new(None)),
            boo: Arc::new(Mutex::new(HashMap::new())),
        });

        rt.expose_poll_function();

        let globals = rt.lua.globals();

        rt.lua.load(&script).exec()?;

        let main_section = match globals.get::<Option<Table>>("main")? {
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
            Some(t) => t.pairs::<String, Value>().collect::<LuaResult<HashMap<_, _>>>()?,
            None => HashMap::new(),
        };

        *rt.main_section.lock().unwrap() = main_section.clone();
        *rt.boo.lock().unwrap() = boo.clone();
        *rt.custom_widgets.lock().unwrap() = custom.clone();

        let config = MainConfig {
            main: main_section,
            bars: bars.clone(),
            custom,
            boo,
        };

        rt.bars.lock().unwrap().extend(bars.clone());


        Ok((rt, config))
    }

    fn expose_poll_function(self: &Arc<Self>) {
        let poll_func = self.lua.create_function(move |_, (src, interval): (String, String)| {
            let duration = parse_duration(&interval)?;
            thread::spawn(move || {
                let lua = Lua::new();
                let func: mlua::Function = lua.load(&src).eval().expect("reloading function failed");
                loop {
                    if let Err(e) = func.call::<()>(()) {
                        eprintln!("poll() error: {}", e);
                    }
                    thread::sleep(duration);
                }
            });
            Ok(())
        }).expect("Failed to create poll function");

        self.lua.globals().set("poll", poll_func).expect("Failed to set poll function");
    }

    pub fn get_custom_widget(&self, name: &str) -> Option<CustomWidget> {
        self.custom_widgets.lock().unwrap().get(name).cloned()
    }

    pub fn get_bar(&self, name: &str) -> Option<Bar> {
        self.bars.lock().unwrap().get(name).cloned()
    }

    pub fn get_all_bars(&self) -> HashMap<String, Bar> {
        self.bars.lock().unwrap().clone()
    }
}

fn parse_duration(s: &str) -> LuaResult<Duration> {
    if let Some(ms) = s.strip_suffix("ms") {
        ms.trim().parse::<u64>().map(Duration::from_millis).map_err(mlua::Error::external)
    } else if let Some(s) = s.strip_suffix('s') {
        s.trim().parse::<u64>().map(Duration::from_secs).map_err(mlua::Error::external)
    } else {
        Err(mlua::Error::external("Invalid duration format (use '1s' or '500ms')"))
    }
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
