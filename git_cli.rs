[dependencies]
clap = { version = "4.4", features = ["derive"] }
dialoguer = "0.11"


use clap::{Arg, ArgAction, Command};
use dialoguer::{Confirm, MultiSelect};
use std::process::{Command as ShellCommand, Stdio};

fn main() {
    let matches = Command::new("git-prune-branches")
        .version("1.0")
        .about("Delete local branches that no longer exist on remote")
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Only show which branches would be deleted")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("Automatically confirm deletion")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Select branches interactively")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("keep")
                .long("keep")
                .help("Branch to keep even if it doesn't exist on remote")
                .action(ArgAction::Append)
                .num_args(1),
        )
        .arg(
            Arg::new("remote")
                .long("remote")
                .help("Remote name (default: origin)")
                .default_value("origin"),
        )
        .get_matches();

    let dry_run = matches.get_flag("dry-run");
    let confirm_all = matches.get_flag("yes");
    let interactive = matches.get_flag("interactive");
    let remote = matches.get_one::<String>("remote").unwrap();
    let mut keep_branches: Vec<String> = vec!["main".into(), "master".into()];

    if let Some(user_keep) = matches.get_many::<String>("keep") {
        keep_branches.extend(user_keep.cloned());
    }

    // Fetch and prune
    ShellCommand::new("git")
        .args(&["fetch", "--prune", remote])
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to fetch remote");

    // Get all local branches
    let output = ShellCommand::new("git")
        .args(&["branch"])
        .output()
        .expect("Failed to list branches");

    let branches = String::from_utf8_lossy(&output.stdout);
    let mut to_delete = Vec::new();

    for line in branches.lines() {
        let branch = line.trim().trim_start_matches("* ").to_string();
        if keep_branches.contains(&branch) {
            continue;
        }

        let status = ShellCommand::new("git")
            .args(&["show-ref", "--verify", "--quiet", &format!("refs/remotes/{}/{}", remote, branch)])
            .status()
            .expect("Failed to check remote branch");

        if !status.success() {
            to_delete.push(branch);
        }
    }

    if to_delete.is_empty() {
        println!("No local branches to delete.");
        return;
    }

    // Interactive selection
    if interactive {
        let selections = MultiSelect::new()
            .with_prompt("Select branches to delete")
            .items(&to_delete)
            .interact()
            .unwrap();

        to_delete = selections.into_iter().map(|i| to_delete[i].clone()).collect();
    }

    // Handle deletion
    for branch in to_delete {
        if dry_run {
            println!("[dry-run] Would delete: {}", branch);
        } else {
            let mut delete = true;

            if !confirm_all && !interactive {
                delete = Confirm::new()
                    .with_prompt(format!("Delete branch '{}'? ", branch))
                    .default(false)
                    .interact()
                    .unwrap();
            }

            if delete {
                println!("Deleting: {}", branch);
                let _ = ShellCommand::new("git")
                    .args(&["branch", "-D", &branch])
                    .status();
            } else {
                println!("Skipped: {}", branch);
            }
        }
    }
}