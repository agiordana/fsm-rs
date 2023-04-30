#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use crate::model::{FiniteStateMachine, State, Transition};

mod model;

struct MyApp {
    fsm: FiniteStateMachine,
    input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut fsm = FiniteStateMachine::default();
        fsm.states.push(State {
            id: 0,
            position: egui::Pos2::new(100.0, 100.0),
            label: "A".to_string(),
            is_initial: true,
            is_final: false,
        });
        fsm.states.push(State {
            id: 1,
            position: egui::Pos2::new(300.0, 100.0),
            label: "B".to_string(),
            is_initial: false,
            is_final: true,
        });
        fsm.transitions.push(Transition {
            from: 0,
            to: 1,
            label: "1".to_string(),
        });

        Self {
            fsm,
            input: "".to_string(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::style::Visuals::light());

        Self::default()
    }
}

impl MyApp {
    fn draw_fsm(&self, ui: &mut egui::Ui) {
        println!("draw_fsm: ui.min_{:?}", ui.min_rect());

        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, ui.available_size());
        let response = ui.interact(rect, ui.id().with("fsm_canvas"), egui::Sense::hover());
        if response.hovered {
            ui.ctx().request_repaint();
        }

        for state in &self.fsm.states {
            state.draw(ui);
        }

        for transition in &self.fsm.transitions {
            transition.draw(&self.fsm, ui);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                if ui.button("New").clicked() {
                    // Handle new automaton creation
                }
                if ui.button("Open").clicked() {
                    // Handle opening an existing automaton
                }
                if ui.button("Save").clicked() {
                    // Handle saving the current automaton
                }
            });
        });

        egui::SidePanel::left("input_panel").show(ctx, |ui| {
            ui.heading("Input");
            ui.add(egui::TextEdit::singleline(&mut self.input));
            if ui.button("Simulate").clicked() {
                // self.simulate();
            }
            if ui.button("Reset").clicked() {
                // self.reset_simulation();
            }
            ui.separator();
            // self.show_simulation_results(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Finite State Machine");
            // ui.label("Finite Automata Canvas will be displayed here");
            self.draw_fsm(ui);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}
