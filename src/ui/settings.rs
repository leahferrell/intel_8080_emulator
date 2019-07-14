use graphics::types::Color;

pub struct Colors {
    pub screen_border: Color,
    pub input_border: Color,
    pub state_border: Color,
    pub stack_trace_border: Color,
    pub screen_background: Color,
    pub input_background: Color,
    pub state_background: Color,
    pub stack_trace_background: Color,
    pub pixel_on: Color,
    pub pixel_off: Color,
}

pub const DEFAULT_COLORS: Colors = Colors {
    screen_border: MOUNTAIN_SHADOW_BLUE,
    input_border: MOUNTAIN_SHADOW_BLUE,
    state_border: MOUNTAIN_SHADOW_BLUE,
    stack_trace_border: MOUNTAIN_SHADOW_BLUE,
    screen_background: BLACK,
    input_background: MOUNTAIN_SHADOW_BLUE,
    state_background: GOLDENROD_YELLOW,
    stack_trace_background: GOLDENROD_YELLOW,
    pixel_off: BLACK,
    pixel_on: WHITE
};

/// Named colors
pub const DARK_GREY: Color = [0.0,0.0,0.0,0.8];
pub const BLACK: Color = [0.0,0.0,0.0,1.0];
pub const WHITE: Color = [255.0/255.0,255.0/255.0,255.0/255.0,1.0];
pub const NAVY: Color = [0.0,0.0,128.0/255.0,1.0];
pub const MOUNTAIN_SHADOW_BLUE: Color = [16.0/255.0,19.0/255.0,87.0/255.0,1.0];
pub const INVISIBLE: Color = [0.0,0.0,0.0,0.0];
pub const OLD_MAKEUP_PINK: Color = [254.0/255.0,164.0/255.0,159.0/255.0,1.0];
pub const GOLDENROD_YELLOW: Color = [251.0/255.0,175.0/255.0,8.0/255.0,1.0];