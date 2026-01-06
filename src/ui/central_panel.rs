use crate::{
    command::command::Command,
    state::app_state::{self, AppState},
};

pub fn show(ctx: &egui::Context, command: &mut Vec<Command>, state: AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(state.value.to_string());
        // The central panel the region left after adding TopPanel's and SidePanel's
        if ui.button("Increment").clicked() {
            command.push(Command::Increment);
        }
        if ui.button("Decrease").clicked() {
            command.push(Command::Decrement);
        }
        ui.separator();

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            // powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}
