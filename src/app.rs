use egui::{Color32, FontData, FontDefinitions, FontFamily};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,
    selected: Option<usize>,
    #[serde(skip)]
    item3: Vec<ButtonSetting>,

    #[serde(skip)]
    item4: Vec<ButtonSetting>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
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
            selected: None,
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
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

impl eframe::App for TemplateApp {
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

        egui::SidePanel::left("my_left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.take_available_space();
                ui.label("Hello World!");

                ui.vertical(|ui| {
                    // ui.take_available_space();
                    // for (i, item) in self.item.iter().enumerate() {
                    //     // let is_selected = self.selected == Some(i);
                    //
                    //     if fancy_button(ui, format!("{} {}", icon(AppIcon::Home), item).as_str())
                    //         .clicked()
                    //     {
                    //         info!("truong dep trai");
                    //     }
                    //
                    //     // if ue
                    //     //     .button(RichText::new(item).color(Color32::RED))
                    //     //     .clicked()
                    //     // {
                    //     //     self.selected = Some(i);
                    //     //     info!("{} is clickek", item);
                    //     // }
                    //     //
                    // }
                    //

                    // for (_, item) in self.item3.iter().enumerate() {
                    //     if fancy_button(
                    //         ui,
                    //         format!("{:#} {:#}", icon(item.icon.clone()), item.text).as_str(),
                    //     )
                    //     .clicked()
                    //     {
                    //         info!("clicked")
                    //     }
                    // }

                    for (_, item) in self.item3.iter().enumerate() {
                        if fancy_button(
                            ui,
                            format!("{:#} {:#}", icon(item.icon.clone()), item.text).as_str(),
                        )
                        .clicked()
                        {
                            info!("clicked")
                        }
                    }

                    ui.separator();

                    for (_, item) in self.item4.iter().enumerate() {
                        if fancy_button(
                            ui,
                            format!("{:#} {:#}", icon(item.icon.clone()), item.text).as_str(),
                        )
                        .clicked()
                        {
                            info!("clicked");
                        }
                    }
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            if ui.button("Decrease").clicked() {
                self.value -= 1.0;
            }
            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn fancy_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let desired_size = egui::vec2(120.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        // let bg_color = if response.hovered() {
        //     egui::Color32::from_rgb(80, 160, 240)
        // } else {
        //     egui::Color32::from_rgb(60, 120, 200)
        // };

        ui.painter().rect_filled(rect, 10.0, Color32::TRANSPARENT);

        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            text,
            egui::TextStyle::Button.resolve(ui.style()),
            visuals.text_color(),
        );
    }

    response
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum AppIcon {
    Search,
    Home,
    Inbox,
    Settings,
    Marketplace,
    Trash,
}

pub fn icon(icon: AppIcon) -> &'static str {
    match icon {
        AppIcon::Search => "󰍉",      // nf-md-magnify
        AppIcon::Home => "󰋜",        // nf-md-home
        AppIcon::Inbox => "󰮍",       // nf-md-inbox
        AppIcon::Marketplace => "󰏓", // nf-md-store
        AppIcon::Settings => "󰒓",    // nf-md-cog
        AppIcon::Trash => "󰩺",       // nf-md-trash_can
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ButtonSetting {
    text: String,
    icon: AppIcon,
}

