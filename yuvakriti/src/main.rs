use std::process::ExitCode;

use clap::CommandFactory;
use clap::FromArgMatches;
use crate::args::SubCommand;
use crate::args::YkArgs;
use crate::compile::do_compile;
use crate::disassemble::do_disassemble;
use crate::run::do_run;

mod args;
mod compile;
mod disassemble;
mod run;

fn main() -> ExitCode {
    let mut command = YkArgs::command();
    let matches = &command.get_matches_mut();
    let args = match YkArgs::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            panic!("Failed to parse command line arguments")
        }
    };

    if let Some(subcommand) = &args.subcommand {
        match match subcommand {
            SubCommand::Compile(args) => do_compile(args),
            SubCommand::Run(args) => do_run(args),
            SubCommand::Disassemble(args) => do_disassemble(args),
        } {
            Ok(_) => {}
            Err(_) => {
                command.print_help().unwrap();
                return ExitCode::FAILURE;
            },
        }
    };

    ExitCode::SUCCESS
}
