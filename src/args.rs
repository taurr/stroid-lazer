use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser, Debug, Resource)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub play: bool,

    #[clap(long)]
    pub level: Option<String>,
}
