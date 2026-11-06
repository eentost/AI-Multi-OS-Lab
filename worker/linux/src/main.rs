use clap::{Parser, Subcommand};
use serde::Serialize;
use std::process::{Command, Output};
use thiserror::Error;

#[derive(Parser)]
#[command(name = "multios-linux-worker")]
struct Cli {
    #[command(subcommand)]
    command: WorkerCommand,
}

#[derive(Subcommand)]
enum WorkerCommand {
    Capabilities,
    List,
    Start { name: String },
    Shutdown { name: String },
}

#[derive(Serialize)]
struct Capabilities {
    worker: &'static str,
    hypervisor: &'static str,
    operations: Vec<&'static str>,
}

#[derive(Error, Debug)]
enum WorkerError {
    #[error("invalid VM name")]
    InvalidName,
    #[error("failed to execute virsh: {0}")]
    Spawn(#[from] std::io::Error),
    #[error("virsh failed: {0}")]
    CommandFailed(String),
}

fn validate_name(name: &str) -> Result<(), WorkerError> {
    if !name.is_empty()
        && name.len() <= 63
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && name.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
    {
        Ok(())
    } else {
        Err(WorkerError::InvalidName)
    }
}

fn run_virsh(args: &[&str]) -> Result<Output, WorkerError> {
    let output = Command::new("virsh").args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(WorkerError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

fn execute(command: WorkerCommand) -> Result<(), WorkerError> {
    match command {
        WorkerCommand::Capabilities => {
            let value = Capabilities {
                worker: "linux",
                hypervisor: "libvirt-kvm",
                operations: vec!["capabilities", "list", "start", "shutdown"],
            };
            println!("{}", serde_json::to_string_pretty(&value).expect("serialize"));
        }
        WorkerCommand::List => print!("{}", String::from_utf8_lossy(&run_virsh(&["list", "--all"])?.stdout)),
        WorkerCommand::Start { name } => {
            validate_name(&name)?;
            print!("{}", String::from_utf8_lossy(&run_virsh(&["start", &name])?.stdout));
        }
        WorkerCommand::Shutdown { name } => {
            validate_name(&name)?;
            print!("{}", String::from_utf8_lossy(&run_virsh(&["shutdown", &name])?.stdout));
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = execute(Cli::parse().command) {
        eprintln!("worker_error: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::validate_name;

    #[test]
    fn validates_vm_names_without_shell_metacharacters() {
        assert!(validate_name("lab-win11-01").is_ok());
        assert!(validate_name("../host").is_err());
        assert!(validate_name("vm;reboot").is_err());
        assert!(validate_name("").is_err());
    }
}
