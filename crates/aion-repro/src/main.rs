use aion_cli::KernelApi;
use clap::{Parser, Subcommand};
use serde_json::json;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "aion-repro", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
    },
    Replay {
        artifact_path: String,
    },
    Diff {
        left_path: String,
        right_path: String,
    },
    Why {
        left_path: String,
        right_path: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<u8, String> {
    let kernel = KernelApi::load()?;
    match cli.command {
        Command::Run { command } => {
            let spec = json!({ "command": command });
            let artifact = kernel.run_execute(&spec.to_string())?;
            let path = "repro_runs/last.json";
            let _ = kernel.run_store(path, &artifact)?;
            println!("{artifact}");
            Ok(0)
        }
        Command::Replay { artifact_path } => {
            let artifact = kernel.run_load(&artifact_path)?;
            let value: serde_json::Value =
                serde_json::from_str(&artifact).map_err(|e| format!("repro replay: {e}"))?;
            let stdout = value
                .get("stdout")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            print!("{stdout}");
            Ok(0)
        }
        Command::Diff {
            left_path,
            right_path,
        }
        | Command::Why {
            left_path,
            right_path,
        } => {
            let a = kernel.run_load(&left_path)?;
            let b = kernel.run_load(&right_path)?;
            let diff = kernel.run_diff(&a, &b)?;
            println!("{diff}");
            Ok(0)
        }
    }
}
