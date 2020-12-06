use crate::{
    builder::BuildConfig,
    cli::{BuildOptions, DevelopOptions},
    config::Config,
    error::Result,
};
use async_std::{prelude::FutureExt, sync::Sender};

use async_std::future;
use async_std::prelude::*;
use futures::{future::Either, select};
use log::info;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::{net::SocketAddr, path::PathBuf};

pub struct DevelopConfig {}
impl Into<DevelopConfig> for DevelopOptions {
    fn into(self) -> DevelopConfig {
        DevelopConfig {}
    }
}

pub async fn start(config: &Config, options: &DevelopConfig) -> Result<()> {
    log::info!("Starting development server 🚀");

    let Config { out_dir, .. } = config;

    // Spawn the server onto a seperate task
    // This lets the task progress while we handle file updates
    let server = async_std::task::spawn(launch_server(out_dir.clone()));
    let watcher = async_std::task::spawn(watch_directory(config.clone()));

    match server.race(watcher).await {
        Err(e) => log::warn!("Error running development server, {:?}", e),
        _ => {}
    }

    Ok(())
}

async fn watch_directory(config: Config) -> Result<()> {
    // Create a channel to receive the events.
    let (watcher_tx, watcher_rx) = async_std::sync::channel(100);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        async_std::task::block_on(watcher_tx.send(res));
    })
    .expect("failed to make watcher");

    let src_dir = crate::cargo::crate_root()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(src_dir.join("src"), RecursiveMode::Recursive)
        .expect("Failed to watch dir");

    let build_config = BuildConfig::default();

    'run: loop {
        crate::builder::build(&config, &build_config)?;

        // Wait for the message with a debounce
        let msg = watcher_rx
            .recv()
            .join(future::ready(1).delay(Duration::from_millis(2000)))
            .await;

        info!("File updated, rebuilding app");
    }
    Ok(())
}

async fn launch_server(outdir: PathBuf) -> Result<()> {
    let crate_dir = crate::cargo::crate_root()?;
    let workspace_dir = crate::cargo::workspace_root()?;

    let mut app = tide::with_state(ServerState::new(outdir.to_owned()));
    let p = outdir.display().to_string();

    app.at("/")
        .get(|req: tide::Request<ServerState>| async move {
            log::info!("Connected to development server");
            let state = req.state();
            Ok(tide::Body::from_file(state.serv_path.clone().join("index.html")).await?)
        })
        .serve_dir(p)?;

    let port = "8080";
    let serve_addr = format!("127.0.0.1:{}", port);

    info!("App available at http://{}", serve_addr);
    app.listen(serve_addr).await?;
    Ok(())
}

/// https://github.com/http-rs/tide/blob/main/examples/state.rs
/// Tide seems to prefer using state instead of injecting into the app closure
/// The app closure needs to be static and
#[derive(Clone)]
struct ServerState {
    serv_path: PathBuf,
}
impl ServerState {
    fn new(serv_path: PathBuf) -> Self {
        Self { serv_path }
    }
}
