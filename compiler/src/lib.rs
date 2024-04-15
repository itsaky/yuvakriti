pub mod args;
pub mod ast;
pub mod bytecode;
pub mod compiler;
pub mod diagnostics;
pub mod features;
pub mod lexer;
pub mod location;
pub mod messages;
pub mod parser;
mod resolve;
mod scope;
mod symtab;
pub mod tokens;

mod attr;
#[cfg(test)]
mod tests;
#[cfg(doctest)]
mod tests;
