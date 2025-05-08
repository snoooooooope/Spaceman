mod app;
mod args;
mod core;
mod error;
mod ui;
mod utils;

use anyhow::Result;
use args::Args;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut app = app::App::new(&args)
        .map_err(|e| anyhow::anyhow!("Failed to initialize application: {}", e))?;
    
    app.run(&args.path)
        .map_err(|e| anyhow::anyhow!("Application error: {}", e))?;

    Ok(())
}
