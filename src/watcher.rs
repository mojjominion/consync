use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;

use crate::{
    callbacks::chezmoi::run_chezmoi,
    filters::{is_create_or_delete, is_file_interesting},
};

fn async_watcher() -> Result<(RecommendedWatcher, Receiver<Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res: Result<Event>| {
            let r = res.as_ref().unwrap();
            if is_create_or_delete(r.kind) && is_file_interesting(&r.paths[0]) {
                futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            };
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if let Some(file_name) = event.paths.get(0) {
                    run_chezmoi(file_name, event.kind);
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
