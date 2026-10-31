use bevy::prelude::*;
use bevy_skein::SkeinPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(SkeinPlugin::default());
}
