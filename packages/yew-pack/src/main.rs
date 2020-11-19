use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;
// use yew_pack::{cli::util::get_workspace_root, error::Error};
use yew_pack::cargo::{crate_root, workspace_root};

fn main() -> yew_pack::error::Result<()> {
    env_logger::init();

    let socket: SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let args = ["asd"].iter();
    let args = args.collect::<Vec<_>>();

    let module = "asd";
    let tmpdir = std::path::Path::new("blah");

    let root = workspace_root().expect("Faield");
    let cratc = crate_root().expect("Faield");
    println!("Root is {:#?}", root);
    println!("c is {:#?}", cratc);
    // Otherwise we're executing in a browser. Spawn a server which serves up
    // the local generated files over an HTTP server.
    // let srv = yew_pack::server::spawn(&socket, &module, &tmpdir, &args)
    //     .context("failed to spawn server")?;
    // let addr = srv.server_addr();
    Ok(())
}
