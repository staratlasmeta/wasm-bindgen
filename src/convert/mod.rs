//! This is mostly an internal module, no stability guarantees are provided. Use
//! at your own risk.

mod closures;
mod impls;
mod slices;
mod traits;

pub use self::impls::*;
pub use self::slices::{WasmSlice, WasmStructArray};
pub use self::traits::*;
