pub mod callbacks;
pub mod cli;
pub mod filters;
pub mod read_dir;
pub mod watcher;

fn run() {
    let path = dirs::config_dir().unwrap_or_default();
    println!("watching {}", path.to_string_lossy());

    futures::executor::block_on(async {
        if let Err(e) = watcher::async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}

/// Async, futures channel based event watching
fn main() {
    let args_provided = cli::check_cli_context();
    if args_provided.is_some() {
        return;
    }

    // Init file creation/removal watcher
    run();
}
