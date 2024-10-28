use bevy::app::App;

mod food;
mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, food::plugin));
}
