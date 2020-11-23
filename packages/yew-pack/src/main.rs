use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;
use tide::{log, Request};
use yew_pack::cargo::{crate_root, workspace_root};
use yew_pack::cli::{LaunchCommand, LaunchOptions};

fn main() -> yew_pack::error::Result<()> {
    // env_logger::init();
    // tide::log::with_level(tide::log::LevelFilter::Trace);

    let up: LaunchOptions = argh::from_env();

    match up.command {
        LaunchCommand::Test(options) => {
            todo!("Pass-through to wasm-bindgen test");
            Ok(())
        }
        LaunchCommand::Build(options) => {
            yew_pack::builder::build_yew_project(options).expect("Building failed");
            Ok(())
        }
        LaunchCommand::Develop(options) => {
            log::info!("Starting development server 🚀");
            let server = yew_pack::develop::launch_development_server(options);
            async_std::task::block_on(server);
            Ok(())
        }
    }
}
