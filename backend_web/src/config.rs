use clap::Parser;

/// See .env.sample in the root for details
#[derive(Parser)]
pub struct AppConfig {
    /// The address you want to use to run the server
    #[clap(env)]
    pub server_address: String,
}
