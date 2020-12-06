use yew_pack::cli::{LaunchCommand, LaunchOptions};

#[async_std::main]
async fn main() -> yew_pack::error::Result<()> {
    yew_pack::logging::set_up_logging();

    let opts: LaunchOptions = argh::from_env();
    let mut config = yew_pack::config::Config::new()?;

    match opts.command {
        LaunchCommand::Build(options) => {
            config.with_build_options(&options);
            yew_pack::builder::build(&config, &(options.into()))?;
        }

        LaunchCommand::Develop(options) => {
            config.with_develop_options(&options);
            yew_pack::develop::start(&config, &(options.into())).await?;
        }

        _ => {
            todo!("Command not currently implemented");
        }
    }

    Ok(())
}
