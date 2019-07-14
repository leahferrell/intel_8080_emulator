extern crate piston_window;
extern crate find_folder;

use opengl_graphics::OpenGL;

use piston_window::*;

use std::path::PathBuf;

use crate::ui::controllers::arcade_controller::ArcadeController;
use crate::ui::models::screen::Screen;
use crate::ui::views::input_view::{InputView,InputViewSettings};
use crate::ui::views::screen_view::{ScreenView, ScreenViewSettings};
use crate::ui::views::state_view::{StateView, StateViewSettings};
use crate::ui::views::stack_trace_view::{StackTraceView, StackTraceViewSettings};
use crate::ui::settings;
use crate::actor::ui_actor::UiActor;

pub struct Window {
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub controller: ArcadeController,
    pub stack_trace_view: StackTraceView,
    pub screen_view: ScreenView,
    pub input_view: InputView,
    pub state_view: StateView,
    pub font_path: PathBuf,
    pub actor: UiActor,
}

impl Window {
    pub fn new(actor: UiActor) -> Window {
        let screen = Screen::new();
        let arcade_controller = ArcadeController::new(screen);

        let stack_trace_view_settings = StackTraceViewSettings::new();
        let stack_trace_view = StackTraceView::new(stack_trace_view_settings);

        let screen_view_settings = ScreenViewSettings::new();
        let screen_view = ScreenView::new(screen_view_settings);

        let input_view_settings = InputViewSettings::new();
        let input_view = InputView::new(input_view_settings);

        let state_view_settings = StateViewSettings::new();
        let state_view = StateView::new(state_view_settings);

        let font_path = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources").unwrap().join("FiraSans-Regular.ttf");

        Window{
            title: "Intel 8080 Emulator".to_owned(),
            width: 768.0,
            height: 512.0,
            controller: arcade_controller,
            stack_trace_view,
            screen_view,
            input_view,
            state_view,
            font_path,
            actor,
        }
    }

    pub fn render(&mut self) {
        let opengl = OpenGL::V4_5;
        let mut window: PistonWindow = WindowSettings::new(&self.title, [self.width, self.height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .expect("Could not create window");

        let mut glyphs = window.load_font(&self.font_path).unwrap();

        while let Some(e) = window.next() {
            self.controller.event(self.input_view.settings.position,
                                    self.input_view.settings.size,
                                    &e);
            window.draw_2d(&e, |c, g, device| {
                clear(settings::MOUNTAIN_SHADOW_BLUE, g);

                self.stack_trace_view.draw(&self.controller, &mut glyphs, &c, g);
                self.screen_view.draw(&c, g);
                self.input_view.draw(&c, g);
                self.state_view.draw(&c, g);

                glyphs.factory.encoder.flush(device);
            });
        }

        self.actor.send_exit_code();
    }
}