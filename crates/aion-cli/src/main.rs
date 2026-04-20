use clap::{Parser, Subcommand};
use std::process::{Command, ExitCode};

#[derive(Parser, Debug)]
#[command(name = "aion", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: ToolCommand,
}

#[derive(Subcommand, Debug)]
enum ToolCommand {
    Repro {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },
    Guard {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let status = match cli.command {
        ToolCommand::Repro { args } => Command::new("aion-repro").args(args).status(),
        ToolCommand::Guard { args } => Command::new("aion-guard").args(args).status(),
    };
    match status {
        Ok(s) => ExitCode::from(s.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!("aion: failed to launch tool: {e}");
            ExitCode::from(2)
        }
    }
}
