use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Align, Label};
use gtk4::prelude::*;
use gdk4::{Display};
use gdk4_x11::{X11Display, X11Surface};
use glib::signal::Inhibit;
use x11::xlib;

use crate::config::{MainConfig, CustomWidget};

#[derive(Debug)]
enum Backend {
    X11,
    Unsupported(String),
}

impl Backend {
    fn detect() -> Self {
        match Display::default() {
            Some(display) => match display.backend() {
                gdk4::Backend::X11 => Backend::X11,
                other => Backend::Unsupported(format!("{:?}", other)),
            },
            None => Backend::Unsupported("no display".into()),
        }
    }
}

pub fn start_ui(_rt: impl std::any::Any, cfg: &MainConfig, window_name: String) {
    let app = Application::builder()
        .application_id("com.sysbar.Boobar")
        .flags(gtk4::gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    let windows = cfg.windows.clone();
    let custom_config = cfg.custom.clone();
    let window_config = match windows.get(&window_name) {
        Some(win) => win.clone(),
        None => {
            eprintln!("No such window: {}", window_name);
            return;
        }
    };

    app.connect_command_line(|app, _| {
        app.activate();
        0
    });

    app.connect_activate(move |app| {
        let hbox = GtkBox::new(Orientation::Horizontal, 5);

        let left_box = GtkBox::new(Orientation::Horizontal, 5);
        left_box.set_halign(Align::Start);
        if let Some(left_ref) = &window_config.left_contents {
            if let Some(widget) = build_widget_from_ref(left_ref, &custom_config) {
                left_box.append(&widget);
            }
        }

        let center_box = GtkBox::new(Orientation::Horizontal, 5);
        center_box.set_hexpand(true);
        center_box.set_halign(Align::Center);
        if let Some(center_ref) = &window_config.center_contents {
            if let Some(widget) = build_widget_from_ref(center_ref, &custom_config) {
                center_box.append(&widget);
            }
        }

        let right_box = GtkBox::new(Orientation::Horizontal, 5);
        right_box.set_halign(Align::End);
        if let Some(right_ref) = &window_config.right_contents {
            if let Some(widget) = build_widget_from_ref(right_ref, &custom_config) {
                right_box.append(&widget);
            }
        }

        hbox.append(&left_box);
        hbox.append(&center_box);
        hbox.append(&right_box);

        let width = window_config.width.as_deref().unwrap_or("600").parse().unwrap_or(600);
        let height = window_config.height.as_deref().unwrap_or("60").parse().unwrap_or(60);
        let win_type = window_config.win_type.as_deref().unwrap_or("float").trim().to_lowercase();

        let window = ApplicationWindow::builder()
            .application(app)
            .title(&format!("Boobar - {}", window_name))
            .default_width(width)
            .default_height(height)
            .resizable(false)
            .child(&hbox)
            .build();

        window.set_decorated(win_type == "window");

        match Backend::detect() {
            Backend::X11 => {
                if win_type == "dock" || win_type == "float" {
                    // SAFELY defer setup until window is mapped
                    let window_clone = window.clone();
                    window.connect_map(move |_| {
                        setup_x11_dock(&window_clone, width, height);
                        // GTK4 signal expects `()`, not `Inhibit`
                    });
                }
            }
            Backend::Unsupported(name) => {
                eprintln!("This bar only supports X11 right now (detected: {})", name);
            }
        }


        window.show();
    });

    app.run();
}

fn build_widget_from_ref(
    ref_str: &str,
    custom_config: &std::collections::HashMap<String, CustomWidget>,
) -> Option<Label> {
    if let Some(custom_name) = ref_str.strip_prefix("custom.") {
        if let Some(custom_table) = custom_config.get(custom_name) {
            if let Some(typ) = &custom_table.type_ {
                if typ == "label" {
                    let content = custom_table.content.as_deref().unwrap_or("missing");
                    return Some(Label::new(Some(content)));
                }
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn setup_x11_dock(window: &ApplicationWindow, width: i32, height: i32) {
    let display = Display::default().unwrap();
    let x11_display = display
        .downcast_ref::<X11Display>()
        .expect("Expected X11 backend");

    let surface = window.surface();
    let x11_surface = surface
        .downcast_ref::<X11Surface>()
        .expect("Expected X11 surface");

    let xdisp = x11_display.as_ptr() as *mut xlib::Display;
    let xid = x11_surface.xid();

    unsafe {
        let net_wm_type = xlib::XInternAtom(xdisp, b"_NET_WM_WINDOW_TYPE\0".as_ptr() as *const i8, 0);
        let net_wm_type_dock = xlib::XInternAtom(xdisp, b"_NET_WM_WINDOW_TYPE_DOCK\0".as_ptr() as *const i8, 0);
        xlib::XChangeProperty(
            xdisp,
            xid,
            net_wm_type,
            xlib::XA_ATOM,
            32,
            xlib::PropModeReplace,
            &net_wm_type_dock as *const _ as *const u8,
            1,
        );

        let strut = [
            0, 0, height as u32, 0,
            0, 0, 0, 0,
            0, width as u32, 0, 0,
        ];
        let strut_partial = xlib::XInternAtom(xdisp, b"_NET_WM_STRUT_PARTIAL\0".as_ptr() as *const i8, 0);
        xlib::XChangeProperty(
            xdisp,
            xid,
            strut_partial,
            xlib::XA_CARDINAL,
            32,
            xlib::PropModeReplace,
            strut.as_ptr() as *const u8,
            strut.len() as i32,
        );

        xlib::XFlush(xdisp);
    }
}
