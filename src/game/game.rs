use bevy::prelude::*;
use super::{window::window, player::player, level::level};
pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins((
            level::LevelPlugin,
            player::PlayerPlugin,
            window::WindowSettingsPlugin));
    }
}
