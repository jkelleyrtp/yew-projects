pub mod builder;
pub mod cargo;
pub mod cli;
pub mod develop;
pub mod error;
pub mod server;
pub mod watch;

pub fn run(options: cli::LaunchOptions)  -> Result<(), ()>{
    use cli::LaunchCommand;
    match options.command {
    // match up.command {
        LaunchCommand::Build(options) => {
            todo!("package the app into a public directory");
            Ok(())
        }
        LaunchCommand::Test(options) => {
            todo!("Pass-through to wasm-bindgen test");
            Ok(())
        }
        LaunchCommand::Develop(options) => {
            // let server = yew_pack::develop::launch_development_server(options);
            // tokio::task::spawn_blocking(server);
            Ok(())
        }
    }
}

