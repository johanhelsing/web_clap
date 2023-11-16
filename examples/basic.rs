use clap::Parser;
use serde::Deserialize;
use tracing::info;
use web_clap::WebParser;

#[derive(Parser, Debug, Clone, Deserialize)]
#[serde(default = "WebParser::from_no_args")]
pub struct Args {
    #[clap(short, long, default_value = "Alice")]
    pub name: String,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    // Same as regular Args::parse(), but works on web as well as native
    let args = Args::platform_parse();

    info!("Hello {}", args.name);
}
