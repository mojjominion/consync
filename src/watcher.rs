use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;

use crate::{
    callbacks::executor::Executor,
    config::AppConfig,
    filters::{is_create_or_delete, is_file_interesting},
};

fn async_watcher(app_config: &AppConfig) -> Result<(RecommendedWatcher, Receiver<Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let config_cloned = app_config.clone();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res: Result<Event>| {
            let r = res.as_ref().unwrap();
            let is_operation = is_create_or_delete(r.kind);
            let is_file = is_file_interesting(&r.paths[0], None, &config_cloned);

            if is_operation && is_file {
                futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            };
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch(app_config: &AppConfig) -> Result<()> {
    let (mut watcher, mut rx) = async_watcher(app_config)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let path = Path::new(app_config.root.as_str());
    watcher.watch(path, RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if let Some(file_name) = event.paths.first() {
                    let executor = Executor {
                        app_config: app_config.clone(),
                        file_name: file_name.to_string_lossy().to_string(),
                        kind: event.kind,
                    };
                    executor.run();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
