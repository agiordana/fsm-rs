use eframe::egui;

#[derive(Debug, Default)]
pub(crate) struct FiniteStateMachine {
    pub(crate) states: Vec<State>,
    pub(crate) transitions: Vec<Transition>,
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) id: usize,
    pub(crate) position: egui::Pos2,
    pub(crate) label: String,
    pub(crate) is_initial: bool,
    pub(crate) is_final: bool,
}

impl State {
    pub(crate) fn draw(&self, ui: &mut egui::Ui) {
        let corner = ui.min_rect().min.to_vec2();

        let radius = 20.0;
        let color_final = egui::Color32::from_rgb(200, 100, 100);
        let color_ordinary = egui::Color32::from_rgb(100, 200, 100);
        let circle_color = if self.is_final {
            color_final
        } else {
            color_ordinary
        };
        // let text_color = egui::Color32::from_rgb(255, 255, 255);

        ui.painter()
            .circle_filled(self.position + corner, radius, circle_color);
        // ui.label_styled(self.label.clone(), egui::TextStyle::Body, text_color)
        //     .align_on_pos(self.position, egui::Align2::CENTER_CENTER)
        //     .draw_at(self.position, ui.style().spacing.text_paint_options);

        if self.is_initial {
            let arrow_vec = egui::Vec2::new(20.0, 0.0);
            let arrow_start = self.position - egui::Vec2::new(radius, 0.0) - arrow_vec;
            let stroke = egui::Stroke::new(3.0, egui::Color32::BLACK);
            ui.painter().arrow(arrow_start + corner, arrow_vec, stroke);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Transition {
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) label: String,
}

impl Transition {
    pub(crate) fn draw(&self, fsm: &FiniteStateMachine, ui: &egui::Ui) {
        let corner = ui.min_rect().min.to_vec2();

        let from_state = fsm.states.get(self.from).unwrap();
        let to_state = fsm.states.get(self.to).unwrap();

        let arrow_color = egui::Color32::from_rgb(50, 50, 50);
        let text_color = egui::Color32::from_rgb(0, 0, 0);

        let stroke = egui::Stroke::new(2.0, arrow_color);
        ui.painter().arrow(
            from_state.position + corner,
            to_state.position - from_state.position,
            stroke,
        );

        // let label_position = (from_state.position.to_vec2() + to_state.position.to_vec2()) / 2.0;
        // ui.label_styled(self.label.clone(), egui::TextStyle::Body, text_color)
        //     .align_on_pos(label_position, egui::Align2::CENTER_CENTER)
        //     .draw_at(label_position, ui.style().spacing.text_paint_options);
    }
}
