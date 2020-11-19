use argh::FromArgs;

/// Everything required to configure and run the `wasm-pack build` command.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
pub struct BuildOptions {}
