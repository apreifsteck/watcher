extern crate notify;

use std::{env, fs, path, sync::mpsc::channel};
use clap::{Parser};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::time::Duration;

fn watch(path: &path::Path) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}


#[derive(Parser)]
struct Args {
    filename: String,
    cmd: String
}

fn search_dir(dir: &path::Path, file: &path::Path) -> Option<path::PathBuf> {
    let fqpn = dir.join(file);
    if fqpn.as_path().is_file() {
        Some(fqpn)
    } else {
        None
    }
}


/**
 * TODO:
 *  Get it so that you can do watch filename cmd
 *  validate filename
 *  get notified on file change
 *  run cmd on file change
 */

fn main() {
    let cli = Args::parse();
    println!("cmd: {:?}", cli.cmd);
    let fname = path::Path::new(&cli.filename);
    let path = env::current_dir().unwrap();
    let path = path.as_path();
    match search_dir(path, fname) {
        Some(thing) => {
            println!("watching {:?}", thing);
            let _res = watch(&thing);
        },
        None => println!("no path"),
    }
}
