#[cfg(feature = "std")]
mod system;

#[cfg(feature = "std")]
pub use self::system::*;

#[cfg(not(feature = "std"))]
mod rust;

#[cfg(not(feature = "std"))]
pub use self::rust::*;
