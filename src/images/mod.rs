pub mod tomato;
pub mod tree;
pub mod default_clock;

pub use tomato::TomatoImage;
pub use tree::TreeImage;

/// Image interface
/// returns colored image depending on the current timer percentage
pub trait Image {
    fn get_string(&self, percentage: u64) -> String;
}

pub use default_clock::DefaultClock;
/// Timer interface
/// returns digits of timer
pub trait Clock {
    fn get_string(&self, minutes: u64, seconds: u64) -> String;
}
