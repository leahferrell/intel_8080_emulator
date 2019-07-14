use graphics::types::Color;
use graphics::types::Radius;
use graphics::{Context, Graphics};
use crate::ui::settings::DEFAULT_COLORS;
use graphics::rectangle::Border;

pub struct ScreenViewSettings {
    pub size: [f64; 2],
    pub border_color: Color,
    pub pixel_color: Color,
    pub background_color: Color,
    pub border_radius: Radius,
    pub position: [f64; 2],
}

impl ScreenViewSettings {
    pub fn new() -> ScreenViewSettings {
        ScreenViewSettings {
            size: [224.0,256.0],
            border_color: DEFAULT_COLORS.screen_border,
            pixel_color: DEFAULT_COLORS.pixel_on,
            background_color: DEFAULT_COLORS.screen_background,
            border_radius: 1.0,
            position: [276.0, 20.0],
        }
    }
}

pub struct ScreenView {
    pub settings: ScreenViewSettings,
}

impl ScreenView {
    pub fn new(settings: ScreenViewSettings) -> ScreenView {
        ScreenView {
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