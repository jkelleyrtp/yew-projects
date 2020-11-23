use crate::error::Error;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct LaunchOptions {
    #[argh(subcommand)]
    pub command: LaunchCommand,
}

/// The various kinds of commands that `wasm-pack` can execute.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum LaunchCommand {
    /// 🛠 Start a development server
    Develop(DevelopOptions),

    /// 🏗️  build your npm package!
    Build(BuildOptions),

    /// 👩‍🔬  test your wasm!
    Test(TestOptions),
}

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "publish")]
pub struct PublishOptions {}

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "test")]
pub struct TestOptions {}

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
pub struct BuildOptions {}

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "develop")]
pub struct DevelopOptions {}
