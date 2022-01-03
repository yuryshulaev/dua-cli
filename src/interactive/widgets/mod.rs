mod entries;
mod footer;
mod header;
mod help;
mod main;
mod mark;

pub use entries::*;
pub use footer::*;
pub use header::*;
pub use help::*;
pub use main::*;
pub use mark::*;

use tui::style::Color;

pub const COLOR_MARKED: Color = Color::Magenta;
pub const COLOR_MARKED_DARK: Color = Color::Magenta;

fn entry_color(fg: Option<Color>, is_file: bool, is_marked: bool) -> Option<Color> {
    match (is_file, is_marked) {
        (true, false) => fg,
        (true, true) => COLOR_MARKED_DARK.into(),
        (false, true) => COLOR_MARKED.into(),
        (false, false) => fg,
    }
}
