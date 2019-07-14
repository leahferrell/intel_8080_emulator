extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

use crate::ui::controllers::arcade_controller::ArcadeController;

use crate::ui::models::screen::Screen;

use crate::ui::views::input_view::{InputView,InputViewSettings};
use crate::ui::views::screen_view::{ScreenView, ScreenViewSettings};
use crate::ui::views::state_view::{StateView, StateViewSettings};
use crate::ui::views::stack_trace_view::{StackTraceView, StackTraceViewSettings};
use crate::ui::settings;

pub fn load_window() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Intel 8080 Emulator", [768, 512])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let screen = Screen::new();
    let mut arcade_controller = ArcadeController::new(screen);

    let stack_trace_view_settings = StackTraceViewSettings::new();
    let stack_trace_view = StackTraceView::new(stack_trace_view_settings);

    let screen_view_settings = ScreenViewSettings::new();
    let screen_view = ScreenView::new(screen_view_settings);

    let input_view_settings = InputViewSettings::new();
    let input_view = InputView::new(input_view_settings);

    let state_view_settings = StateViewSettings::new();
    let state_view = StateView::new(state_view_settings);

    while let Some(e) = events.next(&mut window) {
        arcade_controller.event(input_view.settings.position,
                                   input_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear(settings::MOUNTAIN_SHADOW_BLUE, g);

                stack_trace_view.draw(&c, g);
                screen_view.draw(&c, g);
                input_view.draw(&c, g);
                state_view.draw(&c, g);
            });
        }
    }
}