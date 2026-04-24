pub mod registry;
pub mod gradraw;

pub use registry::*;
pub fn setup() {
    let _ = registry::registry_exists();
}