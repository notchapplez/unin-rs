pub mod registry;
pub use registry::*;
pub fn setup() {
    let _ = registry::registry_exists();
}