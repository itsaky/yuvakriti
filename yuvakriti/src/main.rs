use std::process::ExitCode;

use clap::CommandFactory;
use clap::FromArgMatches;

use crate::args::SubCommand;
use crate::args::YkArgs;
use crate::compile::do_compile;
use crate::disassemble::do_disassemble;
use crate::run::do_run;

pub const YK_DEBUG: &str = "YK_DEBUG";

mod args;
mod compile;
mod disassemble;
mod run;

fn main() -> ExitCode {
    let debug_logging = std::env::var(YK_DEBUG).map(|val| val == "1").is_ok();

    let mut command = YkArgs::command();
    let matches = &command.get_matches_mut();
    let mut args = match YkArgs::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            panic!("Failed to parse command line arguments")
        }
    };

    let level = if args.verbose {
        log::LevelFilter::Trace
    } else if debug_logging {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };

    logging::init_logger(level).unwrap();

    if let Some(subcommand) = &mut args.subcommand {
        match match subcommand {
            SubCommand::Compile(args) => do_compile(args),
            SubCommand::Run(args) => do_run(args),
            SubCommand::Disassemble(args) => do_disassemble(args),
        } {
            Ok(_) => {}
            Err(_) => {
                let sub = command.find_subcommand_mut(subcommand.name()).unwrap();
                let command_help = sub.render_long_help();
                println!("{}", command_help);
                return ExitCode::FAILURE;
            }
        }
    };

    ExitCode::SUCCESS
}
