use std::io::Read;
use std::process::ExitCode;

#[macro_use]
extern crate proc_macros;

pub(crate) mod compiler;
pub(crate) mod logging;
pub(crate) mod vm;

#[cfg(test)]
mod tests;

#[cfg(doctest)]
mod tests;

fn main() -> ExitCode {
    // let path = Path::new("test.yk");
    // let path_display = path.display();
    //
    // let file = match File::open(path) {
    //     Ok(file) => file,
    //     Err(why) => panic!("Failed to open file {}: {}", path_display, why)
    // };
    //
    // let diagnostics_handler = Rc::new(RefCell::new(diagnostics::collecting_handler()));
    // let lexer = YKLexer::new(file, diagnostics_handler.clone());
    // let mut parser = YKParser::new(lexer, diagnostics_handler.clone());
    // let program = parser.parse();

    return ExitCode::from(0);
}
