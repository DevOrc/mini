
pub const BLACK: u16 = 0;
pub const DARK_BLUE: u16 = 1;
pub const GREEN: u16 = 2;
pub const TURQUOISE: u16 = 3;
pub const DARK_RED: u16 = 4;
pub const PURPLE: u16 = 5;
pub const BROWN: u16 = 6;
pub const LIGHT_GRAY: u16 = 7;
pub const DARK_GRAY: u16 = 8;
pub const BLUE: u16 = 9;
pub const LIGHT_GREEN: u16 = 10;
pub const LIGHT_BLUE: u16 = 11;
pub const RED: u16 = 12;
pub const PINK: u16 = 13;
pub const YELLOW: u16 = 14;
pub const WHITE: u16 = 15;

pub fn format_color(foreground: u16, background: u16) -> u16{
    background<<4 | foreground
}
