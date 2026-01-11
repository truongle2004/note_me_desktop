use log::info;

use crate::{command::command::Command, components::button, state::app_state::AppState};

pub fn show(ctx: &egui::Context, command: &mut Vec<Command>, state: AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(state.value.to_string());
        if button::create(ui, "Increase").clicked() {
            command.push(Command::Increment);
        }

        if button::create(ui, "Decrease").clicked() {
            command.push(Command::Decrement);
        }
        ui.separator();

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            // powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}
