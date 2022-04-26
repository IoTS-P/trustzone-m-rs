use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

#[derive(Parser)]
#[clap(name = "xtask")]
#[clap(about = "Program that help you build and debug zihai hypervisor", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build ELF for application
    Make {},
    /// Emulate application system in QEMU
    Qemu {},
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Make {} => {
            println!("xtask: make secure and non secure application");
            xtask_build_all();
        }
        Commands::Qemu {} => {
            println!("xtask: make application and run in QEMU");
            xtask_build_all();
            xtask_run();
        }
    }
}

const DEFAULT_TARGET: &'static str = "thumbv8m.main-none-eabi";

fn xtask_build_all() {
    xtask_build_secure();
    xtask_build_non_secure();
}

fn xtask_build_secure() {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("secure-app"));
    command.arg("build");
    command.args(&["--package", "secure-app"]);
    command.args(&["--target", DEFAULT_TARGET]);
    let status = command.status().unwrap();
    if !status.success() {
        eprintln!("xtask: cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_build_non_secure() {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command.current_dir(project_root().join("non-secure-app"));
    command.arg("build");
    command.args(&["--package", "non-secure-app"]);
    command.args(&["--target", DEFAULT_TARGET]);
    let status = command.status().unwrap();
    if !status.success() {
        eprintln!("xtask: cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_run() {
    let mut command = Command::new("qemu-system-arm");
    command.current_dir(
        project_root()
            .join("target")
            .join(DEFAULT_TARGET)
            .join("debug"),
    );
    command.args(&["-cpu", "cortex-m33"]);
    command.args(&["-machine", "mps2-an505"]);
    command.arg("-nographic");
    command.arg("-semihosting");
    command.args(&["-kernel", "secure-app"]);
    command.args(&["-device", "loader,file=non-secure-app"]);

    let status = command.status().expect("run program");

    if !status.success() {
        eprintln!("xtask: qemu failed with {}", status);
        process::exit(status.code().unwrap_or(1));
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
