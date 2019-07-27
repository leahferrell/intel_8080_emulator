use graphics::types::Color;
use graphics::types::Radius;
use graphics::{Context, Graphics};
use crate::ui::settings::DEFAULT_COLORS;
use graphics::rectangle::Border;

pub struct InputViewSettings {
    pub size: [f64; 2],
    pub border_color: Color,
    pub pixel_color: Color,
    pub background_color: Color,
    pub border_radius: Radius,
    pub position: [f64; 2],
}

impl InputViewSettings {
    pub fn new() -> InputViewSettings {
        InputViewSettings {
            size: [224.0,196.0],
            border_color: DEFAULT_COLORS.input_border,
            pixel_color: DEFAULT_COLORS.pixel_on,
            background_color: DEFAULT_COLORS.input_background,
            border_radius: 1.0,
            position: [276.0, 296.0],
        }
    }
}

pub struct InputView {
    pub settings: InputViewSettings,
}

impl InputView {
    pub fn new(settings: InputViewSettings) -> InputView {
        InputView {
            settings,
        }
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G){
        use graphics::Rectangle;

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
    }
}