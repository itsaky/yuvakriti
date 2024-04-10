pub use vm::CodeExecutor;
pub use vm::Value;
pub use vm::Variable;
pub use vm::YKVM;

pub mod args;
mod vm;

#[cfg(test)]
mod tests;

#[cfg(doctest)]
mod tests;
