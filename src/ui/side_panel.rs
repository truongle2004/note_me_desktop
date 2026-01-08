use log::info;

use crate::{
    command::command::Command, components, shared, state::side_panel_state::SidePanelState,
};

// FIXME
pub fn show(ctx: &egui::Context, _: &mut Vec<Command>, state: SidePanelState) {
    egui::SidePanel::left("my_left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.take_available_space();
            ui.label("Hello World!");

            ui.vertical(|ui| {
                for (_, item) in state.item3.clone().iter().enumerate() {
                    if components::button::create(
                        ui,
                        format!(
                            "{:#} {:#}",
                            shared::icon::get_icon(item.icon.clone()),
                            item.text
                        )
                        .as_str(),
                    )
                    .clicked()
                    {
                        info!("clicked")
                    }
                }

                ui.separator();

                for (_, item) in state.item4.iter().enumerate() {
                    if components::button::create(
                        ui,
                        format!(
                            "{:#} {:#}",
                            shared::icon::get_icon(item.icon.clone()),
                            item.text
                        )
                        .as_str(),
                    )
                    .clicked()
                    {
                        info!("clicked");
                    }
                }
            })
        });
}
