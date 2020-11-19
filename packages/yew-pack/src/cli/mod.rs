use crate::error::Error;
use argh::FromArgs;

mod build;
mod develop;
mod publish;
mod test;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct LaunchOptions {
    #[argh(subcommand)]
    nested: Command,
}

/// The various kinds of commands that `wasm-pack` can execute.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Command {
    /// 🛠 Start a development server
    Develop(develop::DevelopOptions),

    /// 🏗️  build your npm package!
    Build(build::BuildOptions),

    /// 👩‍🔬  test your wasm!
    Test(test::TestOptions),
}

/// Run a command with the given logger!
pub fn run_yew_pack(command: Command) -> std::result::Result<(), Error> {
    // Run the correct command based off input and store the result of it so that we can clear
    // the progress bar then return it
    match command {
        Command::Develop(_) => {
            log::info!("hello");
            Ok(())
        }

        Command::Build(_) => {
            log::info!("hello");
            Ok(())
        }

        Command::Test(_) => {
            log::info!("hello");
            Ok(())
        }
    }
}
