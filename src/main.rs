use std::{
    process::{Child, Command},
    time::Duration,
};

use chrono::{Local, Utc};
use clap::Parser;
use timespec::TimeSpec;

mod timespan;
mod timespec;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// TimeSpec during which the command should run.
    #[arg(short, long)]
    timespec: TimeSpec,

    #[arg(long = "tz", visible_alias = "timezone")]
    timezone: Option<chrono_tz::Tz>,

    #[arg(required = true, num_args = 1, allow_hyphen_values = true)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    loop {
        let is_active = if let Some(tz) = args.timezone {
            args.timespec.is_active(Utc::now().with_timezone(&tz))
        } else {
            args.timespec.is_active(Local::now())
        };

        if is_active {
            let mut child = execute_subcommand(&args.command);
            let mut kill_sent = false;

            loop {
                let is_active = if let Some(tz) = args.timezone {
                    args.timespec.is_active(Utc::now().with_timezone(&tz))
                } else {
                    args.timespec.is_active(Local::now())
                };

                match child.try_wait() {
                    Err(err) => panic!("{}", err),
                    Ok(Some(exit)) => {
                        if kill_sent {
                            break;
                        } else {
                            std::process::exit(exit.code().unwrap_or(0));
                        }
                    }
                    Ok(None) => {}
                }

                if !is_active {
                    if let Err(err) = Command::new("kill").arg(format!("{}", child.id())).spawn() {
                        panic!("Failed to send kill command to child process: {}", err);
                    }
                    kill_sent = true;
                }

                std::thread::sleep(Duration::from_secs(1));
            }
        }

        std::thread::sleep(Duration::from_secs(1))
    }
}

fn execute_subcommand(command_and_args: &[String]) -> Child {
    let mut command = Command::new(&command_and_args[0]);
    if command_and_args.len() > 1 {
        command.args(&command_and_args[1..]);
    }

    if let Ok(child) = command.spawn() {
        child
    } else {
        panic!("Failed to execute child process");
    }
}
