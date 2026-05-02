pub mod registry;
pub use registry::*;

pub mod comms;
pub use comms::*;
pub fn setup() {
    let _ = registry_exists();
}
