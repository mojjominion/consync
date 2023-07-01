use crate::watcher::async_watch;
pub mod callbacks;
pub mod filters;
pub mod watcher;

/// Async, futures channel based event watching
fn main() {
    let path = dirs::config_dir().unwrap_or_default();
    println!("watching {}", path.to_string_lossy());

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}
