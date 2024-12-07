use super::Image;
use crate::constants;

/// TreeImage implementation
pub struct TreeImage {
    template: Vec<&'static str>,
    color_map: Vec<Vec<char>>,
}

impl TreeImage {
    pub fn new() -> Self {
        let template = vec![
              "            **          ",
              "           ████         ",
              "          ██████        ",
              "         ████████       ",
              "        ██████████      ",
              "       ████████████     ",
              "      ██████████████    ",
              "     ████████████████   ",
              "    ██████████████████  ",
              "           ████         ",
              "           ████         ",
        ];
        let color_map = vec![
              "            RR          ",
              "           GGGG         ",
              "          GMGGGG        ",
              "         GGGGGGGG       ",
              "        GGGWGGGGGG      ",
              "       GGYGGGGMGGGG     ",
              "      GGGGGGGGGGGGGG    ",
              "     GGGGGGGMGGGGGGGG   ",
              "    GGGGGGGGGGGGGWGGGG  ",
              "           BBBB         ",
              "           BBBB         ",
        ]
        .iter()
        .map(|line| line.chars().collect())
        .collect();

        Self { template, color_map }
    }
}

impl Image for TreeImage {
    fn get_string(&self, percentage: u64) -> String {
        let total_pixels: usize = self
            .template
            .iter()
            .flat_map(|line| line.chars())
            .filter(|&ch| ch != ' ')
            .count();

        let filled_pixels = (percentage * total_pixels as u64) / 100;

        let mut filled_count = 0;
        let mut result = String::new();

        for (i, line) in self.template.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch != ' ' && filled_count < filled_pixels {
                    // Получаем цвет из карты
                    let color_char = self.color_map[i][j];
                    let color = match color_char {
                        'R' => constants::RED,
                        'G' => constants::GREEN,
                        'W' => constants::WHITE,
                        'M' => constants::MAGENTA,
                        'Y' => constants::YELLOW,
                        'B' => constants::BLACK,
                        _ => constants::EMPTY_COLOR,
                    };

                    result.push_str(&color);
                    result.push(ch);
                    result.push_str(&constants::RESET_COLOR);
                    filled_count += 1;
                } else if ch != ' ' {
                    result.push_str(constants::EMPTY_COLOR);
                    result.push(ch);
                    result.push_str(&constants::RESET_COLOR);
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        result
    }
}
