use std::{
    io::Error,
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
        if is_active(&args.timespec, args.timezone) {
            let mut child = execute_subcommand(&args.command);
            let mut kill_sent = false;

            loop {
                match child.try_wait() {
                    Err(err) => panic!("{}", err),
                    Ok(Some(exit)) => {
                        if kill_sent {
                            break;
                        } else {
                            // The underlying process exited on its own, so during will also exit
                            // and propagate the exit code.
                            std::process::exit(exit.code().unwrap_or(0));
                        }
                    }
                    Ok(None) => {}
                }

                if !is_active(&args.timespec, args.timezone) {
                    kill_subcommand(&child);
                    kill_sent = true;
                }

                std::thread::sleep(Duration::from_secs(1));
            }
        }

        std::thread::sleep(Duration::from_secs(1))
    }
}

fn is_active(timespec: &TimeSpec, timezone: Option<chrono_tz::Tz>) -> bool {
    if let Some(tz) = timezone {
        timespec.is_active(Utc::now().with_timezone(&tz))
    } else {
        timespec.is_active(Local::now())
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

fn kill_subcommand(child: &Child) {
    unsafe {
        // A child could technically ignore SIGTERM, but SIGQUIT could be more damaging by leaving
        // orphaned subprocesses of our child if we don't allow any graceful shutdown.
        let err = libc::kill(child.id() as libc::pid_t, libc::SIGTERM);
        if err == -1 {
            let os_error = Error::last_os_error();
            panic!("failed to kill child process: {:?}", os_error);
        };
    }
}
