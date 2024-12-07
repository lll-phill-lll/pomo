use std::{env, process};
use std::thread;
use std::time::Duration;

mod constants;

mod images;
use images::{Image, TomatoImage, TreeImage};

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
            print!("  ");
        }
        println!();
    }
}

fn create_image(image_type: &str) -> Box<dyn Image> {
    match image_type.to_lowercase().as_str() {
        "tomato" => Box::new(TomatoImage::new()),
        "tree" => Box::new(TreeImage::new()),
        _ => {
            eprintln!("Wrong image, '{}' is not supported. Available: 'tomato', 'tree'.", image_type);
            process::exit(1);
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn run_animation(image: Box<dyn Image>, duration: u64) {
    let mut elapsed_seconds = 0;
    let total_seconds = duration * 60;

    loop {
        let percentage = elapsed_seconds as f32 / total_seconds as f32;

        clear_screen();


        println!("{}", image.get_string(percentage));
        draw_graphic_timer(
            (total_seconds - elapsed_seconds) / 60,
            (total_seconds - elapsed_seconds) % 60,
        );

        if elapsed_seconds >= total_seconds {
            break;
        }

        thread::sleep(Duration::from_secs(1));
        elapsed_seconds += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <image_type> <duration_min>", args[0]);
        eprintln!("Exanple: {} tomato 5", args[0]);
        process::exit(1);
    }

    let image_type = &args[1];
    let duration: u64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: time must be int minutes.");
        process::exit(1);
    });

    let image = create_image(image_type);
    run_animation(image, duration);
}
