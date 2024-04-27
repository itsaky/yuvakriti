pub use vm::YKVM;

pub mod args;
mod value;
mod vm;

#[cfg(test)]
mod tests;

mod macros;
mod memory;
mod object;
#[cfg(doctest)]
mod tests;
