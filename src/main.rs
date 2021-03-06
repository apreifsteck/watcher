extern crate notify;

use clap::Parser;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::{env, path, process::Command, sync::mpsc::channel};

fn watch_with_callback(path: &path::Path, mut callback: impl FnMut()) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(250))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(notify::DebouncedEvent::Write(_path)) => callback(),
            Err(e) => println!("watch error: {:?}", e),
            _ => (),
        }
    }
}

#[derive(Parser)]
struct Args {
    #[clap(parse(try_from_str=parse_file))]
    filename: path::PathBuf,
    #[clap(parse(try_from_str=parse_cmd))]
    cmd: Command,
}

impl Args {
    fn sub_args(&mut self) {
        let new_args: Vec<&std::ffi::OsStr> = self
            .cmd
            .get_args()
            .map(|arg| {
                if arg == "@1" {
                    self.filename.as_os_str()
                } else {
                    arg
                }
            })
            .collect();

        let mut subbed_cmd = Command::new(self.cmd.get_program());
        subbed_cmd.args(new_args);
        self.cmd = subbed_cmd;
    }
}

fn execute_cmd(cmd: &mut Command) {
    cmd.spawn().expect("something went wrong");
    println!();
}

fn parse_file(file: &str) -> Result<path::PathBuf, String> {
    env::current_dir()
        .or(Err(String::from("could not get current directory")))
        .and_then(|cur_dir| {
            let p = path::Path::new(file);
            let fqpn = cur_dir.join(p);
            if fqpn.is_file() || fqpn.is_dir() {
                Ok(fqpn)
            } else {
                Err(String::from("is not valid file or directory"))
            }
        })
}

fn parse_cmd(cmd: &str) -> Result<Command, String> {
    let arg_vec: Vec<&str> = cmd.split(' ').collect();
    match arg_vec.split_at(1) {
        ([base_cmd], []) => Ok(Command::new(base_cmd)),
        ([base_cmd], args) => {
            let mut new_cmd = Command::new(base_cmd);
            new_cmd.args(args);
            Ok(new_cmd)
        }
        _ => Err(String::from("Bogus command")),
    }
}

/**
 * TODO:
 *  Get it so you can watch a directory: DONE
 *  Get it so that you can specify multiple files or paths.      
 *  validate filename: needs better error handling?
 */

fn main() {
    let mut cli = Args::parse();
    cli.sub_args();
    let _res = watch_with_callback(cli.filename.as_path(), || execute_cmd(&mut cli.cmd));
}
