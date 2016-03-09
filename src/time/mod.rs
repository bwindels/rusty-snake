#[cfg (target_os="macos")]
mod machtime;
#[cfg (target_os="macos")]
pub use self::machtime::{Clock, Instant};