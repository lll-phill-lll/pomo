use super::Clock;

const DIGITS: [&[&str]; 10] = [
    &[
        "███",
        "█ █",
        "█ █",
        "█ █",
        "███",
    ],
    &[
        "  █",
        "  █",
        "  █",
        "  █",
        "  █",
    ],
    &[
        "███",
        "  █",
        "███",
        "█  ",
        "███",
    ],
    &[
        "███",
        "  █",
        "███",
        "  █",
        "███",
    ],
    &[
        "█ █",
        "█ █",
        "███",
        "  █",
        "  █",
    ],
    &[
        "███",
        "█  ",
        "███",
        "  █",
        "███",
    ],
    &[
        "███",
        "█  ",
        "███",
        "█ █",
        "███",
    ],
    &[
        "███",
        "  █",
        "  █",
        "  █",
        "  █",
    ],
    &[
        "███",
        "█ █",
        "███",
        "█ █",
        "███",
    ],
    &[
        "███",
        "█ █",
        "███",
        "  █",
        "███",
    ],
];

pub struct DefaultClock;

impl DefaultClock {
    pub fn new() -> Self {
        DefaultClock
    }
}

impl Clock for DefaultClock {
    fn get_string(&self, minutes: u64, seconds: u64) -> String {
        let min_tens = (minutes / 10) as usize;
        let min_ones = (minutes % 10) as usize;
        let sec_tens = (seconds / 10) as usize;
        let sec_ones = (seconds % 10) as usize;

        let timer_graphic = vec![
            DIGITS[min_tens],
            DIGITS[min_ones],
            &[" ██ ", " ██ ", "    ", " ██ ", " ██ "],  // Разделитель
            DIGITS[sec_tens],
            DIGITS[sec_ones],
        ];

        let mut result = String::new();

        for row in 0..5 {
            result.push_str(" ");
            for part in &timer_graphic {
                result.push_str(part[row]);
                result.push_str("  "); // Пустое место между цифрами
            }
            result.push_str("\n");
        }

        result
    }
}
