//! Yet Another mruby (yamrb) runtime layer.
//! Provides value representation, opcode tables, helpers, and the VM itself
//! so mruby bytecode can execute inside Rust.
#[cfg(feature = "opcode-coverage")]
pub mod coverage;
pub mod helpers;
pub mod op;
pub mod optable;
pub mod shared_memory;
pub mod value;
pub mod vm;

pub mod prelude;
