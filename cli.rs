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




#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    fn setup_test_environment() -> (PathBuf, PathBuf) {
        // Create temporary directories for testing
        let src_dir = PathBuf::from("test_src");
        let dest_dir = PathBuf::from("test_dest");

        // Ensure they are clean before the test
        if src_dir.exists() {
            fs::remove_dir_all(&src_dir).unwrap();
        }
        if dest_dir.exists() {
            fs::remove_dir_all(&dest_dir).unwrap();
        }

        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();

        // Create test files in the source directory
        let file_path = src_dir.join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        let sub_dir = src_dir.join("sub_dir");
        fs::create_dir_all(&sub_dir).unwrap();

        let sub_file_path = sub_dir.join("sub_test_file.txt");
        let mut sub_file = File::create(&sub_file_path).unwrap();
        writeln!(sub_file, "Subdirectory content").unwrap();

        (src_dir, dest_dir)
    }

    #[test]
    fn test_copy_folder_success() {
        let (src_dir, dest_dir) = setup_test_environment();

        // Call the function
        copy_folder(src_dir.to_str().unwrap(), dest_dir.clone()).unwrap();

        // Assert that the files were copied
        assert!(dest_dir.join("test_file.txt").exists());
        assert!(dest_dir.join("sub_dir").exists());
        assert!(dest_dir.join("sub_dir/sub_test_file.txt").exists());

        // Clean up
        fs::remove_dir_all(src_dir).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }

    #[test]
    fn test_copy_folder_source_not_found() {
        let dest_dir = PathBuf::from("test_dest");

        // Ensure destination directory exists
        if dest_dir.exists() {
            fs::remove_dir_all(&dest_dir).unwrap();
        }
        fs::create_dir_all(&dest_dir).unwrap();

        let result = copy_folder("non_existent_folder", dest_dir);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::NotFound
        );

        // Clean up
        fs::remove_dir_all(dest_dir).unwrap();
    }

    #[test]
    fn test_setup_command() {
        let (src_dir, dest_dir) = setup_test_environment();

        // Simulate `setup` command
        if let Err(e) = copy_folder(src_dir.to_str().unwrap(), dest_dir.clone()) {
            panic!("Setup command failed: {}", e);
        }

        assert!(dest_dir.join("test_file.txt").exists());
        assert!(dest_dir.join("sub_dir/sub_test_file.txt").exists());

        // Clean up
        fs::remove_dir_all(src_dir).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }

    #[test]
    fn test_cargo_command() {
        let (src_dir, dest_dir) = setup_test_environment();

        // Simulate `cargo` command
        if let Err(e) = copy_folder(src_dir.to_str().unwrap(), dest_dir.clone()) {
            panic!("Cargo command failed: {}", e);
        }

        assert!(dest_dir.join("test_file.txt").exists());
        assert!(dest_dir.join("sub_dir/sub_test_file.txt").exists());

        // Clean up
        fs::remove_dir_all(src_dir).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }
}