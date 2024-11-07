use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(super) struct Cli {
    #[arg(long)]
    pub(super) host: Option<String>,
    #[arg(long)]
    pub(super) port: Option<u16>,
    #[arg(long)]
    pub(super) html_dir: Option<PathBuf>,
}
