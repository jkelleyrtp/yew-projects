//! Build and pack a yew project into a static set of output content
use argh::FromArgs;

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "publish")]
pub struct PublishOptions {}
