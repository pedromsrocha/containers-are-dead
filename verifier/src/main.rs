mod wasmtime;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
enum Command {
    WasmtimeModule {
        path: PathBuf,
    },
    WasmtimeComponent {
        path: PathBuf,
        #[arg(allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },
    Spin,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::parse();

    match cmd {
        Command::WasmtimeModule { path } => {
            wasmtime::run_module(&path)?;
        }
        Command::WasmtimeComponent { path, args } => {
            wasmtime::run_component(&path, &args)?;
        }
        Command::Spin => run_with_spin(),
    }

    // let venv =
    //     .join(".venv");

    // if venv.try_exists().ok() != Some(true) {
    //     // Create the project-specific Python virtual environment.
    //     println!("\n=== Creating the Python virtual environment ===\n");
    //     cmd!("uv", "sync", "--no-install-workspace", "--managed-python")
    //         .stderr_to_stdout()
    //         .run()?;
    // }

    // println!("\n=== Syncing Python packages ===\n");
    // run_in_venv(
    //     cmd!("uv", "sync", "--all-packages", "--managed-python"),
    //     &venv,
    // )?;

    // println!("\n=== Testing the Rust crate ===\n");
    // run_in_venv(cmd!("cargo", "test"), &venv)?;

    // println!("\n=== Testing the Python package ===\n");
    // run_in_venv(cmd!("uv", "run", "--managed-python", "pytest"), &venv)?;

    Ok(())
}

fn run_with_spin() {
    todo!()
}
