use std::error;

use config::AppConfig;

pub mod callbacks;
pub mod cli;
pub mod config;
pub mod filters;
pub mod read_dir;
pub mod watcher;

fn run(app_config: &AppConfig) {
    println!("watching {}", app_config.root);

    futures::executor::block_on(async {
        if let Err(e) = watcher::async_watch(app_config).await {
            println!("error: {:?}", e)
        }
    });
}

/// Async, futures channel based event watching
fn main() -> Result<(), Box<dyn error::Error>> {
    let app_config = config::load_config()?;

    let args_provided = cli::check::cli_context(&app_config);
    if args_provided.is_some() {
        return Ok(());
    }

    // Init file creation/removal watcher
    run(&app_config);
    //
    Ok(())
}
