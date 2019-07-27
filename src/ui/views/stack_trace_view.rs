use graphics::types::Color;
use graphics::types::Radius;
use graphics::{Context, Graphics};
use crate::ui::settings::DEFAULT_COLORS;
use graphics::rectangle::Border;
use crate::ui::controllers::arcade_controller::ArcadeController;
use crate::ui::settings;

use piston_window::*;
use graphics::character::CharacterCache;

pub struct StackTraceViewSettings {
    pub size: [f64; 2],
    pub border_color: Color,
    pub pixel_color: Color,
    pub background_color: Color,
    pub border_radius: Radius,
    pub position: [f64; 2],
}

impl StackTraceViewSettings {
    pub fn new() -> StackTraceViewSettings {
        StackTraceViewSettings {
            size: [236.0,470.0],
            border_color: DEFAULT_COLORS.stack_trace_border,
            pixel_color: DEFAULT_COLORS.pixel_on,
            background_color: DEFAULT_COLORS.stack_trace_background,
            border_radius: 1.0,
            position: [20.0; 2],
        }
    }
}

pub struct StackTraceView {
    pub settings: StackTraceViewSettings,
}

impl StackTraceView {
    pub fn new(settings: StackTraceViewSettings) -> StackTraceView {
        StackTraceView {
            settings,
        }
    }

    pub fn draw<G: Graphics, C>(
        &self,
        controller: &ArcadeController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G
    ) where C: CharacterCache<Texture = G::Texture> {
        let ref settings = self.settings;

        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size[0], settings.size[1],
        ];

        Rectangle::new(settings.background_color)
            .border(Border {
                color: settings.border_color,
                radius: settings.border_radius
            })
            .draw(board_rect, &c.draw_state, c.transform, g);

        let transform = c.transform.trans(10.0, 100.0);

        //let stack = controller.getStack();

        text::Text::new_color(settings::BLACK, 12).draw(
            "Hey I am a string",
            glyphs,
            &c.draw_state,
            transform, g
        );
    }
}