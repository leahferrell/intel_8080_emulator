use piston::input::GenericEvent;

use crate::ui::models::screen::Screen;

pub struct ArcadeController {
    pub screen: Screen,
    pub selected_button: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

impl ArcadeController {
    pub fn new(screen: Screen) -> ArcadeController {
        ArcadeController {
            screen,
            selected_button: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            /*let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_button = Some([cell_x, cell_y]);
            }*/

            println!("Mouse click at: {:?}", self.cursor_pos);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("Key pressed: {:?}", key);

            /*if let Some(ind) = self.selected_cell {
                // Set cell value.
                match key {
                    Key::D1 => self.screen.set(ind, 1),
                    Key::D2 => self.screen.set(ind, 2),
                    Key::D3 => self.screen.set(ind, 3),
                    Key::D4 => self.screen.set(ind, 4),
                    Key::D5 => self.screen.set(ind, 5),
                    Key::D6 => self.screen.set(ind, 6),
                    Key::D7 => self.screen.set(ind, 7),
                    Key::D8 => self.screen.set(ind, 8),
                    Key::D9 => self.screen.set(ind, 9),
                    _ => {}
                }
            }*/
        }
    }
}