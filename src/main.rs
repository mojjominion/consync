use std::fs::File;

use daemonize::Daemonize;

use crate::watcher::async_watch;
pub mod callbacks;
pub mod filters;
pub mod watcher;

fn run() {
    let path = dirs::config_dir().unwrap_or_default();
    println!("watching {}", path.to_string_lossy());

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}

/// Async, futures channel based event watching
fn main() {
    let stdout = File::create("/tmp/consync-daemon.out").unwrap();
    let stderr = File::create("/tmp/consync-daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/consync.pid") // Every method except `new` and `start`
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon")
        .group(2)
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| run());

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    };
}
