use bevy::prelude::*;
use clap::Parser;
#[allow(unused)]
use tracing::*;

use crate::{assets::GameStartSettings, states::GameState};

#[derive(Parser, Debug, Resource)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub play: bool,

    #[clap(long)]
    pub level: Option<String>,
}

pub struct CmdLinePlugin;

#[derive(Debug, Clone, Default, SystemSet, PartialEq, Eq, Hash)]
pub struct CmdLineSet;

impl Plugin for CmdLinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(<Args as clap::Parser>::parse())
            .add_systems(
                OnExit(GameState::LoadingAssets),
                set_start_level.in_set(CmdLineSet),
            )
            .add_systems(
                OnEnter(GameState::MainMenu),
                start_play.run_if(run_once()).in_set(CmdLineSet),
            );
    }
}

fn set_start_level(args: Res<Args>, mut game_start: ResMut<GameStartSettings>) {
    if let Some(level) = args.level.clone() {
        debug!(?level, "set starting level");
        game_start.level = level;
    }
}

fn start_play(args: Res<Args>, mut next: ResMut<NextState<GameState>>) {
    if args.play {
        debug!("starting game directly");
        next.set(GameState::Playing);
    }
}
