use egui::Color32;

pub fn create(ui: &mut egui::Ui, text: &str) -> egui::Response {
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
