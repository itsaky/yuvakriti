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
pub mod tokens;
mod scope;
mod resolve;
mod symtab;

#[cfg(test)]
mod tests;
#[cfg(doctest)]
mod tests;
mod attr;
