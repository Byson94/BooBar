use gtk4::{gio, Application, ApplicationWindow, Label, Box as GtkBox, Orientation, Align};
use gtk4::prelude::{ApplicationExtManual, WidgetExt, BoxExt, ApplicationExt};

use crate::config::{MainConfig, Bar, CustomWidget};

pub fn start_ui(_rt: impl std::any::Any, cfg: &MainConfig) {
    let app = Application::builder()
        .application_id("com.sysbar.Boobar")
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    let main_config = cfg.main.clone();
    let bars_config = cfg.bars.clone();
    let custom_config = cfg.custom.clone();

    app.connect_command_line(|app, _| {
        app.activate();
        0
    });

    app.connect_activate(move |app| {
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        if let Some(main_section) = &main_config {
            for bar_name in &main_section.bar {
                if let Some(bar_table) = bars_config.get(bar_name) {
                    let hbox = GtkBox::new(Orientation::Horizontal, 5);

                    // Left container
                    let left_box = GtkBox::new(Orientation::Horizontal, 5);
                    left_box.set_halign(Align::Start);
                    if let Some(left_ref) = &bar_table.left_contents {
                        if let Some(widget) = build_widget_from_ref(left_ref, &custom_config) {
                            left_box.append(&widget);
                        }
                    }

                    // Center container
                    let center_box = GtkBox::new(Orientation::Horizontal, 5);
                    center_box.set_hexpand(true);
                    center_box.set_halign(Align::Center);
                    if let Some(center_ref) = &bar_table.center_contents {
                        if let Some(widget) = build_widget_from_ref(center_ref, &custom_config) {
                            center_box.append(&widget);
                        }
                    }

                    // Right container
                    let right_box = GtkBox::new(Orientation::Horizontal, 5);
                    right_box.set_halign(Align::End);
                    if let Some(right_ref) = &bar_table.right_contents {
                        if let Some(widget) = build_widget_from_ref(right_ref, &custom_config) {
                            right_box.append(&widget);
                        }
                    }

                    hbox.append(&left_box);
                    hbox.append(&center_box);
                    hbox.append(&right_box);
                    vbox.append(&hbox);
                }
            }
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Boobar")
            .default_width(600)
            .default_height(60)
            .child(&vbox)
            .build();

        window.show();
    });

    app.run();
}

fn build_widget_from_ref(ref_str: &str, custom_config: &std::collections::HashMap<String, CustomWidget>) -> Option<Label> {
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
