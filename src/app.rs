use crate::{
    command::command::Command,
    shared::icon::AppIcon,
    state::{
        app_state::AppState, button_setting::ButtonSetting, reducer::reduce,
        side_panel_state::SidePanelState,
    },
    ui::{central_panel, side_panel},
};
use eframe::glow::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER;
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NoteMeApp {
    command: Vec<Command>,
    app_state: AppState,
    side_panel_state: SidePanelState,
}

impl Default for NoteMeApp {
    fn default() -> Self {
        Self {
            command: Vec::new(),
            app_state: AppState { value: 2.7 },
            side_panel_state: SidePanelState {
                item3: vec![
                    {
                        ButtonSetting {
                            text: "Search".to_string(),
                            icon: AppIcon::Search,
                        }
                    },
                    {
                        ButtonSetting {
                            text: "Home".to_string(),
                            icon: AppIcon::Home,
                        }
                    },
                    {
                        ButtonSetting {
                            text: "Inbox".to_string(),
                            icon: AppIcon::Inbox,
                        }
                    },
                ],
                item4: vec![
                    {
                        ButtonSetting {
                            text: "Settings".to_string(),
                            icon: AppIcon::Settings,
                        }
                    },
                    {
                        ButtonSetting {
                            text: "Marketplace".to_string(),
                            icon: AppIcon::Marketplace,
                        }
                    },
                    {
                        ButtonSetting {
                            text: "Trash".to_string(),
                            icon: AppIcon::Trash,
                        }
                    },
                ],
            },
        }
    }
}
impl NoteMeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = FontDefinitions::default();

        // Load Nerd Font
        fonts.font_data.insert(
            "icons".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/SymbolsNerdFontMono-Regular.ttf"
            ))),
        );

        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts
                .families
                .get_mut(&family)
                .unwrap()
                .insert(0, "icons".to_owned());
        }
        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for NoteMeApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        central_panel::show(ctx, &mut self.command, self.app_state.to_owned());

        side_panel::show(ctx, &mut self.command, self.side_panel_state.to_owned());

        for cmd in self.command.drain(..) {
            reduce(&mut self.app_state, cmd);
        }
    }
}
