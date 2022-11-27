use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct Args {
    #[arg(long, env = "NARIGAMA_PROTOHACKERS_HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, env = "NARIGAMA_PROTOHACKERS_PORT", default_value = "4000")]
    pub port: u16,
}
