use std::{env, process};
use std::thread;
use std::time::Duration;

mod constants;

mod images;
use images::{Image, Clock, DefaultClock, TomatoImage, TreeImage};

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

fn create_clock() -> Box<dyn Clock> {
    Box::new(DefaultClock::new())
}

fn clear_screen() {
    println!("{}", constants::CRLF);
}

fn run_animation(image: Box<dyn Image>, clock: Box<dyn Clock>, duration: u64) {
    let mut elapsed_seconds = 0;
    let total_seconds = duration * 60;

    loop {
        let percentage = elapsed_seconds * 100 / total_seconds;

        clear_screen();

        println!("{}", image.get_string(percentage));
        println!("{}", clock.get_string(elapsed_seconds / 60, elapsed_seconds % 60));

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
    let clock = create_clock();
    run_animation(image, clock, duration);
}
