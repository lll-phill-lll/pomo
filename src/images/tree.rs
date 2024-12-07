use super::Image;

use crate::constants::{FILLED_COLOR, RESET_COLOR, EMPTY_COLOR};
/// TreeImage implementation
pub struct TreeImage {
    template: Vec<&'static str>,
}

impl TreeImage {
    pub fn new() -> Self {
        Self {
            template: vec![
                "            *          ",
                "           ███         ",
                "          █████        ",
                "         ███████       ",
                "        █████████      ",
                "       ███████████     ",
                "      █████████████    ",
                "     ███████████████   ",
                "    █████████████████  ",
                "           ███         ",
                "           ███         ",
            ],
        }
    }
}

impl Image for TreeImage {
    fn get_string(&self, percentage: f32) -> String {
        // skip all space symbols while counting how many symbols to color
        let total_pixels: usize = self
            .template
            .iter()
            .flat_map(|line| line.chars())
            .filter(|&ch| ch != ' ')
            .count();
        let filled_pixels = (percentage * total_pixels as f32).round() as usize;

        let mut filled_count = 0;
        let mut result = String::new();

        for line in &self.template {
            for ch in line.chars() {
                if ch != ' ' && filled_count < filled_pixels {
                    result.push_str(&format!("{}{}{}", FILLED_COLOR, ch, RESET_COLOR));
                    filled_count += 1;
                } else if ch != ' ' {
                    result.push_str(&format!("{}{}{}", EMPTY_COLOR, ch, RESET_COLOR));
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        result
    }
}
