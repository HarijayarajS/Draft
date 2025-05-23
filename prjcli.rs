use argh::FromArgs;
use directories::ProjectDirs;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::Command,
};

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    description: Option<String>,
    git_initialized: bool,
}

#[derive(FromArgs)]
/// Project Manager CLI
struct Cli {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Add(AddCommand),
    List(ListCommand),
    Delete(DeleteCommand),
    Select(SelectCommand),
}

#[derive(FromArgs, Debug)]
/// Add a project
#[argh(subcommand, name = "add")]
struct AddCommand {
    #[argh(positional)]
    name: String,

    #[argh(option, short = 'd')]
    description: Option<String>,

    #[argh(switch, short = 'g')]
    git: bool,
}

#[derive(FromArgs, Debug)]
/// List all projects
#[argh(subcommand, name = "list")]
struct ListCommand;

#[derive(FromArgs, Debug)]
/// Delete a project
#[argh(subcommand, name = "delete")]
struct DeleteCommand {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, Debug)]
/// Select and open a project in VS Code
#[argh(subcommand, name = "select")]
struct SelectCommand;

fn get_storage_path() -> PathBuf {
    ProjectDirs::from("com", "yourname", "projman")
        .expect("Failed to get project dir")
        .data_dir()
        .join("projects.json")
}

fn get_recent_path() -> PathBuf {
    get_storage_path().with_file_name("recent.json")
}

fn load_projects() -> Vec<Project> {
    let path = get_storage_path();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_projects(projects: &[Project]) {
    let data = serde_json::to_string_pretty(projects).unwrap();
    fs::create_dir_all(get_storage_path().parent().unwrap()).ok();
    fs::write(get_storage_path(), data).unwrap();
}

fn save_recent(name: &str) {
    fs::write(get_recent_path(), name).ok();
}

fn load_recent() -> Option<String> {
    fs::read_to_string(get_recent_path()).ok()
}

fn init_git_repo(name: &str) -> bool {
    let path = get_storage_path().parent().unwrap().join(name);
    fs::create_dir_all(&path).ok()?;
    Repository::init(&path).is_ok()
}

fn main() {
    let cli: Cli = argh::from_env();
    let mut projects = load_projects();

    match cli.command {
        Command::Add(cmd) => {
            if projects.iter().any(|p| p.name == cmd.name) {
                eprintln!("Project '{}' already exists.", cmd.name);
            } else {
                let git = if cmd.git { init_git_repo(&cmd.name) } else { false };
                projects.push(Project {
                    name: cmd.name,
                    description: cmd.description,
                    git_initialized: git,
                });
                save_projects(&projects);
                println!(
                    "Project added. Git initialized: {}",
                    if git { "yes" } else { "no" }
                );
            }
        }

        Command::List(_) => {
            if projects.is_empty() {
                println!("No projects found.");
            } else {
                for (i, project) in projects.iter().enumerate() {
                    println!(
                        "{}. {}{} [{}]",
                        i + 1,
                        project.name,
                        project
                            .description
                            .as_ref()
                            .map(|d| format!(": {}", d))
                            .unwrap_or_default(),
                        if project.git_initialized { "Git" } else { "No Git" }
                    );
                }
            }
        }

        Command::Delete(cmd) => {
            let initial_len = projects.len();
            projects.retain(|p| p.name != cmd.name);
            if projects.len() < initial_len {
                save_projects(&projects);
                println!("Deleted '{}'", cmd.name);
            } else {
                println!("Project '{}' not found.", cmd.name);
            }
        }

        Command::Select(_) => {
            if projects.is_empty() {
                println!("No projects found.");
                return;
            }

            let recent = load_recent();
            let mut indexed: Vec<(usize, &Project)> = projects.iter().enumerate().collect();

            if let Some(ref recent_name) = recent {
                if let Some((i, _)) = indexed.iter().find(|(_, p)| &p.name == recent_name).cloned() {
                    let proj = indexed.remove(i);
                    indexed.insert(0, proj);
                }
            }

            for (i, project) in indexed.iter().enumerate() {
                let marker = if Some(&project.name) == recent.as_ref() {
                    "*"
                } else {
                    " "
                };

                println!(
                    "{}{}. {}{} [{}]",
                    marker,
                    i + 1,
                    project.name,
                    project
                        .description
                        .as_ref()
                        .map(|d| format!(": {}", d))
                        .unwrap_or_default(),
                    if project.git_initialized { "Git" } else { "No Git" }
                );
            }

            print!("Enter number to open in VS Code: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let selected = match input.trim().parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid input.");
                    return;
                }
            };

            if let Some((_, project)) = indexed.get(selected - 1) {
                let path = get_storage_path().parent().unwrap().join(&project.name);
                if path.exists() {
                    save_recent(&project.name);
                    match Command::new("code").arg(&path).status() {
                        Ok(s) if s.success() => {
                            println!("Opened '{}'", project.name);
                        }
                        _ => eprintln!("Failed to open in VS Code."),
                    }
                } else {
                    eprintln!("Project folder not found.");
                }
            } else {
                eprintln!("Invalid selection.");
            }
        }
    }
}


/*

[package]
name = "project-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
argh = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
directories = "5.0"
git2 = "0.18"


*/