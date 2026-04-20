use aion_cli::KernelApi;
use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_BASELINE: &str = ".aion/baseline.json";

#[derive(Parser, Debug)]
#[command(name = "aion-guard", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Record {
        #[arg(long)]
        baseline: Option<PathBuf>,
        #[arg(long)]
        cmd: String,
    },
    Check {
        #[arg(long)]
        baseline: Option<PathBuf>,
        #[arg(long)]
        cmd: String,
        #[arg(long)]
        compare_duration: bool,
        #[arg(long, default_value_t = 5.0)]
        duration_tolerance: f64,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<u8, String> {
    let kernel = KernelApi::load()?;
    match cli.command {
        Command::Record { baseline, cmd } => {
            let path = baseline.unwrap_or_else(|| PathBuf::from(DEFAULT_BASELINE));
            let spec = json!({ "cmd": cmd });
            let artifact = kernel.run_execute(&spec.to_string())?;
            let _ = kernel.run_store(&path.to_string_lossy(), &artifact)?;
            println!("{}", path.display());
            Ok(0)
        }
        Command::Check {
            baseline,
            cmd,
            compare_duration,
            duration_tolerance,
        } => {
            let path = baseline.unwrap_or_else(|| PathBuf::from(DEFAULT_BASELINE));
            let baseline_artifact = kernel.run_load(&path.to_string_lossy())?;
            let spec = json!({ "cmd": cmd });
            let actual_artifact = kernel.run_execute(&spec.to_string())?;
            let diff = kernel.run_diff(&baseline_artifact, &actual_artifact)?;
            let diff_value: serde_json::Value = serde_json::from_str(&diff)
                .map_err(|e| format!("guard check: invalid diff response: {e}"))?;
            let changed = diff_value
                .get("changed")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            if changed {
                println!("{diff}");
                return Ok(1);
            }
            if compare_duration {
                let baseline_ms = extract_duration(&baseline_artifact)?;
                let actual_ms = extract_duration(&actual_artifact)?;
                let allowed = (baseline_ms as f64) * (duration_tolerance / 100.0);
                let drift = baseline_ms.abs_diff(actual_ms) as f64;
                if drift > allowed {
                    println!(
                        "{{\"changed\":true,\"fields\":[{{\"field\":\"duration_ms\",\"baseline\":{},\"actual\":{}}}]}}",
                        baseline_ms, actual_ms
                    );
                    return Ok(1);
                }
            }
            Ok(0)
        }
    }
}

fn extract_duration(artifact_json: &str) -> Result<u64, String> {
    let value: serde_json::Value = serde_json::from_str(artifact_json)
        .map_err(|e| format!("guard check: invalid artifact JSON: {e}"))?;
    Ok(value
        .get("duration_ms")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0))
}
