use clap::Parser;
use poopoo_xtask::builder::{Builder, PoopooBuilder};
use poopoo_xtask::cli::CLIArgs;
use std::sync::LazyLock;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    tracing::debug!("Parsing CLI arguments...");
    let args = CLIArgs::parse();
    tracing::debug!("Parsed CLI arguments.");

    tracing::debug!("Creating a PoopooBuilder...");
    let builder = PoopooBuilder::new(args, CARGO.to_string());
    tracing::debug!("Created a PoopooBuilder.");

    builder.run()?;

    Ok(())
}

static CARGO: LazyLock<String> =
    LazyLock::new(|| std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
