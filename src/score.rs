use bevy::{log::tracing_subscriber::fmt::format, prelude::*};

#[derive(Component, Deref, DerefMut)]
pub struct Score(pub u64);

pub struct ScorePlugin;

#[derive(Component)]
pub struct ScoreLabel;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_score)
            .add_systems(Update, display_score);
    }
}

fn setup_score(
    mut commands: Commands,
) {
    commands.spawn(Score(0));
}

fn display_score(
    score: Single<&Score, Changed<Score>>,
    labels: Query<&mut Text2d, With<ScoreLabel>>,
) {
    labels
        .into_iter()
        .for_each(|mut text| {
            *text = Text2d::new(format!("score: {}", score.0));
        });
}
