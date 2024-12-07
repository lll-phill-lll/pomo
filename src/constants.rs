pub const CRLF: &str = "\x1B[2J\x1B[H";

pub const FILLED_COLOR: &str = "\x1B[32m";
pub const EMPTY_COLOR: &str = "\x1B[90m";

// ANSI escape codes for text colors in terminal

// Reset to default terminal colors
pub const RESET_COLOR: &str = "\x1b[0m";

// Basic colors
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

// Bright colors
pub const BRIGHT_BLACK: &str = "\x1b[30;1m"; // Bright black is usually gray
pub const BRIGHT_RED: &str = "\x1b[31;1m";
pub const BRIGHT_GREEN: &str = "\x1b[32;1m";
pub const BRIGHT_YELLOW: &str = "\x1b[33;1m";
pub const BRIGHT_BLUE: &str = "\x1b[34;1m";
pub const BRIGHT_MAGENTA: &str = "\x1b[35;1m";
pub const BRIGHT_CYAN: &str = "\x1b[36;1m";
pub const BRIGHT_WHITE: &str = "\x1b[37;1m";

// Background colors
pub const BG_BLACK: &str = "\x1b[48;5;16m";
pub const BG_RED: &str = "\x1b[48;5;196m";
pub const BG_GREEN: &str = "\x1b[48;5;46m";
pub const BG_YELLOW: &str = "\x1b[48;5;226m";
pub const BG_BLUE: &str = "\x1b[48;5;21m";
pub const BG_MAGENTA: &str = "\x1b[48;5;201m";
pub const BG_CYAN: &str = "\x1b[48;5;51m";
pub const BG_WHITE: &str = "\x1b[48;5;15m";

// Bright background colors
pub const BG_BRIGHT_BLACK: &str = "\x1b[48;5;233m";
pub const BG_BRIGHT_RED: &str = "\x1b[48;5;9m";
pub const BG_BRIGHT_GREEN: &str = "\x1b[48;5;40m";
pub const BG_BRIGHT_YELLOW: &str = "\x1b[48;5;220m";
pub const BG_BRIGHT_BLUE: &str = "\x1b[48;5;32m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[48;5;170m";
pub const BG_BRIGHT_CYAN: &str = "\x1b[48;5;51m";
pub const BG_BRIGHT_WHITE: &str = "\x1b[48;5;255m";

// High intensity colors (often mapped to bright colors)
pub const HIGH_INTENSITY_RED: &str = "\x1b[1;31m";
pub const HIGH_INTENSITY_GREEN: &str = "\x1b[1;32m";
pub const HIGH_INTENSITY_YELLOW: &str = "\x1b[1;33m";
pub const HIGH_INTENSITY_BLUE: &str = "\x1b[1;34m";
pub const HIGH_INTENSITY_MAGENTA: &str = "\x1b[1;35m";
pub const HIGH_INTENSITY_CYAN: &str = "\x1b[1;36m";
pub const HIGH_INTENSITY_WHITE: &str = "\x1b[1;37m";

