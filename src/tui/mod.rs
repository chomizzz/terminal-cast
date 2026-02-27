pub mod keyboard;
pub mod render;
pub mod terminal;

// Raccourcis pour utilisation facile
pub use keyboard::{handle_keyboard_events, KeyboardAction};
pub use render::draw_ui;
pub use terminal::{restore_terminal, setup_terminal};
