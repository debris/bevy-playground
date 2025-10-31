use bevy::prelude::*;

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_healthbar)
            .add_systems(Update, update_healthbar);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u64);

#[derive(Component, Deref, DerefMut)]
pub struct MaxHealth(pub u64);

#[derive(Component)]
pub struct Healthbar;

#[derive(Component, Deref, DerefMut)]
pub struct HealthbarYOffset(pub f32);

#[derive(Component)]
struct HealthbarBar;

#[derive(Component)]
struct HealthbarText;

/// Ids of child components of the healthbar.
#[derive(Component)]
struct HealthbarComponents {
    //background_bar: Entity,
    bar: Entity,
    text: Entity,
}

// progress range 0.5
// max_width range any
fn rect_x_center(progress: f32, max_width: f32) -> f32 {
    -(1.0 - progress) * max_width / 2.0
}

fn rect_x_width(progress: f32, max_width: f32) -> f32 {
    progress * max_width
}

fn health_text(health: u64, max_health: u64) -> String {
    format!("{}/{}", health, max_health)
}

fn draw_healthbar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Health, &MaxHealth, Option<&HealthbarYOffset>), Added<Healthbar>>
) {
    for (entity, health, max_health, y_offset) in query {
        let progress = ((health.0 as f64 / max_health.0 as f64) as f32).max(0.).min(1.);
        let max_width = 200.0;
        let height = 20.0;
        let rectangle = Rectangle::new(rect_x_width(progress, max_width), height);
        let y_offset = y_offset.map(|y| y.0).unwrap_or(0.);

        let bar = commands.spawn((
            HealthbarBar,
            Mesh2d(meshes.add(rectangle)),
            MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
            Transform::from_xyz(rect_x_center(progress, max_width), y_offset, 1.),
            Visibility::Inherited,
            )).id();

        let text = commands.spawn((
            HealthbarText,
            Text2d::new(health_text(health.0, max_health.0)),
            Transform::from_xyz(0., y_offset, 1.),
        )).id();

        commands
            .entity(entity)
            .add_children(&[bar, text])
            .try_insert(HealthbarComponents {
                bar,
                text,
            });
    }
}

fn update_healthbar(
    mut meshes: ResMut<Assets<Mesh>>,
    mut bars: Query<(&mut Transform, &Mesh2d), With<HealthbarBar>>,
    mut texts: Query<&mut Text2d, With<HealthbarText>>,
    query: Query<(&Health, &MaxHealth, &HealthbarComponents, Option<&HealthbarYOffset>), (Changed<Health>, With<Healthbar>)>,
) {
    for (health, max_health, components, y_offset) in query {
        // update text
        if let Some(mut text) = texts.get_mut(components.text).ok() {
            text.0 = health_text(health.0, max_health.0);
        }

        if let Some((mut transform, mesh2d)) = bars.get_mut(components.bar).ok() {
            let progress = ((health.0 as f64 / max_health.0 as f64) as f32).max(0.).min(1.);
            let max_width = 200.0;
            let height = 20.0;

            let y_offset = y_offset.map(|y| y.0).unwrap_or(0.);
            *transform = Transform::from_xyz(rect_x_center(progress, max_width), y_offset, 1.);

            if let Some(mesh) = meshes.get_mut(mesh2d) {
                *mesh = Rectangle::new(rect_x_width(progress, max_width), height).into();
            }
        }
    }
}

mod tests {
    
    #[test]
    fn test_rect_x_center() {
        use super::*;

        assert_eq!(0.0, rect_x_center(1.0, 200.0));
        assert_eq!(-90.0, rect_x_center(0.1, 200.0));
        assert_eq!(-80.0, rect_x_center(0.2, 200.0));
    }
}

