use std::fs;
use std::path::PathBuf;
use argh::FromArgs;

/// CLI for copying folders.
#[derive(FromArgs, Debug)]
struct Cli {
    /// the command to execute: "setup" or "cargo"
    #[argh(subcommand)]
    command: Command,
}

/// Subcommands for the CLI.
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Setup(SetupCommand),
    Cargo(CargoCommand),
}

/// Copy the setup folder.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "setup")]
struct SetupCommand {
    /// destination path
    #[argh(positional)]
    destination: PathBuf,
}

/// Copy the cargo folder.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "cargo")]
struct CargoCommand {
    /// destination path
    #[argh(positional)]
    destination: PathBuf,
}

fn copy_folder(src: &str, destination: PathBuf) -> std::io::Result<()> {
    let src_path = PathBuf::from(src);

    if !src_path.exists() || !src_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source folder '{}' does not exist.", src),
        ));
    }

    if !destination.exists() {
        fs::create_dir_all(&destination)?;
    }

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let src_file = entry.path();
        let dest_file = destination.join(entry.file_name());

        if src_file.is_dir() {
            copy_folder(src_file.to_str().unwrap(), dest_file)?;
        } else {
            fs::copy(&src_file, &dest_file)?;
        }
    }

    Ok(())
}

fn main() {
    let cli: Cli = argh::from_env();

    match cli.command {
        Command::Setup(cmd) => {
            if let Err(e) = copy_folder("setup-folder", cmd.destination) {
                eprintln!("Error copying setup folder: {}", e);
            } else {
                println!("Successfully copied setup folder.");
            }
        }
        Command::Cargo(cmd) => {
            if let Err(e) = copy_folder("cargo-folder", cmd.destination) {
                eprintln!("Error copying cargo folder: {}", e);
            } else {
                println!("Successfully copied cargo folder.");
            }
        }
    }
}