//! Where all the magic is centralised.
//!
//! TODO Separate the ELisp-related logic into it's own Plugin.

use bevy::prelude::*;

/// Title for the Visualization Engine.
pub const TITLE: &'static str = "Bulb";

/// The Engine plugin which brings everything together.
/// Currently it only provides a basic Bevy-based fullscreen window.
#[derive(Copy, Clone)]
pub struct BulbEnginePlugin;
impl Plugin for BulbEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: <Option<Window>>::from(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                title: TITLE.into(),
                ..default()
            }),
            ..default()
        }));

        app.insert_resource(ClearColor(Color::BLACK));
    }
}
