pub mod tomato;
pub mod tree;

pub use tomato::TomatoImage;
pub use tree::TreeImage;

/// Image interface
/// returns colored image depending on the current timer percentage
pub trait Image {
    fn get_string(&self, percentage: f32) -> String;
}
