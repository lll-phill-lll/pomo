use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::{thread, time::Duration};

const TIMER_DURATION: u64 = 1; // Длительность таймера в минутах

// Графическое представление цифр (5x3)
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

fn draw_tomato(percentage: f32) {
    let tomato_template = [
        "           ██     █       ",
        "        ███ ██  ██        ",
        "    ████░░██░███░░████    ",
        "  ██░░░░░█░░██░░░██░░░██  ",
        "██░░░░░░░░░█░░░░░░░█░░░░██",
        "██░░░░░░░░░░░░░░░░░░░░░░██",
        "██░░░░░░░░░░░░░░░░░░░░░░██",
        "██░░░░░░░░░░░░░░░░░░░░░░██",
        "██░░░██░░░░░░░░░░░░░░░░░██",
        "  ██░░░░██░░░░░░░░░░░░██  ",
        "    ██░░░░░░░░░░░░░░██    ",
        "      ██████████████      ",
        "                          ",
    ];

    let filled_lines = (percentage * tomato_template.len() as f32).round() as usize;

    for (i, line) in tomato_template.iter().enumerate() {
        if i < filled_lines {
            stdout()
                .execute(SetForegroundColor(Color::Green))
                .unwrap()
                .execute(Print(line))
                .unwrap();
        } else {
            stdout()
                .execute(SetForegroundColor(Color::Red))
                .unwrap()
                .execute(Print(line))
                .unwrap();
        }
        println!();
    }
    stdout().execute(ResetColor).unwrap();
}

fn draw_graphic_timer(minutes: u64, seconds: u64) {
    let min_tens = (minutes / 10) as usize;
    let min_ones = (minutes % 10) as usize;
    let sec_tens = (seconds / 10) as usize;
    let sec_ones = (seconds % 10) as usize;

    let timer_graphic = vec![
        DIGITS[min_tens],
        DIGITS[min_ones],
        &[" ██ ", " ██ ", "    ", " ██ ", " ██ "],
        DIGITS[sec_tens],
        DIGITS[sec_ones],
    ];

    for row in 0..5 {
        print!(" ");
        for part in &timer_graphic {
            print!("{}", part[row]);
            print!("  "); // Отступ между цифрами
        }
        println!();
    }
}


fn main() {
    let total_seconds = TIMER_DURATION * 60;
    let mut elapsed_seconds = 0;

    loop {
        let percentage = elapsed_seconds as f32 / total_seconds as f32;

        // Очистка экрана
        stdout()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(cursor::MoveTo(0, 0))
            .unwrap();

        // Рисуем помидор и таймер
        draw_tomato(percentage);
        draw_graphic_timer(
            (total_seconds - elapsed_seconds) / 60,
            (total_seconds - elapsed_seconds) % 60,
        );

        // Проверяем завершение таймера
        if elapsed_seconds >= total_seconds {
            break;
        }

        // Ждём одну секунду
        thread::sleep(Duration::from_secs(1));
        elapsed_seconds += 1;
    }

    // Завершающее сообщение
    stdout()
        .execute(Print("Pomodoro завершён!\n"))
        .unwrap();
}
