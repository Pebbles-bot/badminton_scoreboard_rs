// external crate imports
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui::{
    Context, Image, Sense, Vec2, self, Color32, Stroke, Style, Theme, global_theme_preference_buttons, style::Selection,
};


// local imports
mod engine;
use crate::engine::state;

mod render;
use crate::render::render_cli;
use crate::render::render_egui;


fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // start a gamestate
    let mut current_gamestate : state::Gamestate = state::initialize_singles();
    
    // create a gui appstruct that can access the gamestate
    let app = App {
        gamestate : current_gamestate,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Scoreboard",
        options,
        Box::new(|cc| {

            // Use the dark theme
            cc.egui_ctx.set_theme(egui::Theme::Dark);

            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::new(app))
        }),
    )
}

struct App {
    gamestate: state::Gamestate,
}

impl App {

}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        //main window
        egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::DARK_GREEN)
            )
        .show(ctx, |ui| {

            // shut down if the gamestate is resolved TODO: turn this into a prompt
            if !!self.gamestate.is_resolved() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            };

            // just fetch some things from gamestate for convenience later
            let score_team1 : String = self.gamestate.get_scores().0.to_string();
            let score_team2 : String = self.gamestate.get_scores().1.to_string();
            let name_team1 : String = self.gamestate.get_names().0.to_string();
            let name_team2 : String = self.gamestate.get_names().1.to_string();

            // drawing stuff
            //let screen_rect = ui.ctx().viewport_rect().shrink2(Vec2{x: 0.0, y: 10.0}).translate(Vec2{x: 0.0, y: -5.0});

            let width = ui.available_size_before_wrap().x;
            let height = ui.available_size_before_wrap().y;

            let size_court = egui::vec2(0.5 * width, 0.95 * height );

            let settpanel_button = egui::vec2(0.04 * width, 0.04 * height);


            let screen_rect = egui::Rect {
                min: egui::Pos2{x:0.0, y:0.0},
                max: egui::Pos2{x: width, y: 0.95 * height}
            };

            let bgimage = Image::new(egui::include_image!("assets/imgs/bcourt.png"));
            bgimage.paint_at(ui, screen_rect);

            // new comment

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {

                // HORIZONTAL BLOCK ONE
                ui.horizontal(|ui| {

                    // some style overrides for this section
                    let mut style = (*ctx.style()).clone();
                    style.text_styles = [(egui::TextStyle::Button, egui::FontId::new(40.0, egui::FontFamily::Monospace))].into();
                    ui.style_mut().text_styles = style.text_styles;
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

                    //ui.style_mut().spacing.button_padding = egui::vec2(0.0, 0.0);
                    
                    if ui.add_sized(size_court, egui::Button::new( format!("{1} \n {0}", score_team1, name_team1) )
                            .frame(false)
                            .fill(Color32::TRANSPARENT) )
                        .clicked() {

                        // process game logic
                        state::score_team1(&mut self.gamestate);
                        state::check_winning(&mut self.gamestate);

                        render_cli::render_gamestate(&self.gamestate);

                    };

                    if ui.add_sized(size_court, egui::Button::new( format!("{1} \n {0}", score_team2, name_team2) )
                            .frame(false)
                            .fill(Color32::TRANSPARENT) )
                        .clicked() {

                        // process game logic
                        state::score_team2(&mut self.gamestate);
                        state::check_winning(&mut self.gamestate);

                        render_cli::render_gamestate(&self.gamestate);

                    };

                });
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {

                // HORIZONTAL BLOCK OPTIONS MENU
                ui.horizontal(|ui| {

                    if ui.add_sized(settpanel_button, egui::Button::new("Sett")).clicked() {
                        dbg!(self.gamestate.is_resolved());
                    };

                    if ui.add_sized(settpanel_button, egui::Button::new("Undo")).clicked() {
                        dbg!(self.gamestate.is_resolved());
                    };

                    if ui.add_sized(settpanel_button, egui::Button::new("End")).clicked() {
                        dbg!(self.gamestate.is_resolved());
                    };

                    if ui.add_sized(settpanel_button, egui::Button::new("Log")).clicked() {
                        dbg!(self.gamestate.is_resolved());
                    };



                });
            });

            

            //ui.send_viewport_cmd(egui::ViewportCommand::Close);


        });



        
    }
}