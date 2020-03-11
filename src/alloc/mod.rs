//! Memory allocation APIs

pub use core::alloc::{Layout, LayoutErr};

#[cfg(feature = "unstable_core_alloc")]
#[rustversion::since(2020-01-30)]
pub use core::alloc::AllocRef;

#[cfg(feature = "unstable_core_alloc")]
#[rustversion::before(2020-01-30)]
pub use core::alloc::{Alloc as AllocRef, Excess};

#[cfg(feature = "unstable_core_alloc")]
pub use core::alloc::{AllocErr, CannotReallocInPlace};

#[cfg(feature = "unstable_core_alloc")]
pub type UnstableLayoutMethods = ();

#[cfg(not(feature = "unstable_core_alloc"))]
mod stable;
#[cfg(not(feature = "unstable_core_alloc"))]
pub use stable::{Alloc as AllocRef, Excess, AllocErr, CannotReallocInPlace, UnstableLayoutMethods};

pub fn handle_alloc_error(layout: Layout) -> ! {
    panic!("encountered allocation error: {:?}", layout)
}

