use bevy::app::App;

mod food;
mod movement;
mod portal;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, food::plugin, portal::plugin));
}
