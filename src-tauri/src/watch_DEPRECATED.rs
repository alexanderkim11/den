/*
===========================

CURRENTLY DEFUNCT

============================
*/





use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::Arc, sync::atomic::AtomicI32, time::Duration};

use notify_debouncer_mini::{new_debouncer_opt, Config as DebouncerConfig};
use notify_debouncer_mini::new_debouncer;

use tauri::Emitter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct DownloadStarted<'a> {
    url: &'a str,
    download_id: usize,
    content_length: usize,
}

#[tauri::command]
/// Example for debouncer mini
pub async fn debouncer_watch(app: tauri::AppHandle, path: String) {
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 1 seconds
    let mut debouncer = new_debouncer(Duration::from_millis(2000), tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .unwrap();


    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => events
            .iter()
            .for_each(|event| println!("Event {event:?}")),  //app.emit("download-started", DownloadStarted{url: "url", download_id: 0usize, content_length: 1usize}).unwrap(),
            Err(error) => println!("error"),
        }
    }
}




fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}


#[tauri::command]
pub async fn async_watch(path: String) {
    let path_ref = &Path::new(&path);
    let (mut watcher, mut rx) = async_watcher().unwrap();


    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path_ref, RecursiveMode::Recursive).unwrap();

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

