use crate::error::Result;
use async_std::future;
use async_std::prelude::*;
use notify::{watcher, RecursiveMode, Watcher};
use std::ffi::OsString;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

// TODO: Build a sexier frontend that the webserver can talk to
pub async fn launch_development_server(options: crate::cli::DevelopOptions) -> Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/")
        .get(|_| async { Ok(tide::Body::from_file("public/index.html").await?) })
        .serve_dir("public/")?;

    let d = app.listen("127.0.0.1:8080").join(watch_src_dir()).await;

    Ok(())
}

async fn watch_src_dir() -> Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // async_std::sync::
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("/home/test/notify", RecursiveMode::Recursive)
        .unwrap();

        rx.
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
