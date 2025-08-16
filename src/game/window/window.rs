use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowMode, WindowResolution},
};

pub struct WindowSettingsPlugin;
impl Plugin for WindowSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, init_window)
            .add_systems(Startup, cursor_lock)
            .add_systems(Update, grab_mouse);
    }
}

fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.resolution = WindowResolution::new(1920., 1080.);
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    }
}

fn cursor_lock()
{
    let window = Window {
        cursor_options: CursorOptions {
            grab_mode: CursorGrabMode::Confined,
            ..default()
        },
        ..default()
    };
}
fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
)
{
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }

}