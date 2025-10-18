use bevy::{prelude::*, sprite::Anchor};
use bevy::text::TextBounds;

use crate::mouse::MouseState;
use crate::styles::UiStyles;
use crate::touch::TouchState;

pub struct TooltipPlugin;

#[derive(Component, Clone)]
pub struct Tooltip {
    pub text: String,
    pub area: Vec2,
}

impl Tooltip {
    pub fn with_text(text: &str) -> Tooltip {
        Tooltip { 
            text: text.to_string(),
            area: Vec2::new(128., 64.),
        }
    }
}

#[derive(Component, PartialEq)]
pub enum TooltipState {
    Hidden,
    Visible(Entity),
}

#[derive(Component)]
pub struct TooltipView(Entity);

impl TooltipView {
    pub fn create(
        style: &UiStyles,
        tooltip: &Tooltip,
        transform: Transform,
        entity: Entity,
    ) -> impl Bundle {(
        Name::new("Tooltip"),
        TooltipView(entity),
        //TooltipView(entity),
        Sprite::from_color(style.tooltip.background_color, Vec2::ZERO),
        transform,
        Anchor::TOP_LEFT,
        //TooltipState::Hidden,
        Visibility::Inherited,
        children![(
            Anchor::TOP_LEFT,
            Text2d::new(&tooltip.text),
            style.tooltip.font.clone(),
            TextColor(style.tooltip.text_color),
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            TextBounds::from(tooltip.area),
            Transform::from_translation(Vec3::Z),
        )]
    )}
}

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, add_state)
            .add_systems(Update, process_touch);
    }
}

fn add_state(
    mut commands: Commands,
    entities: Query<Entity, Added<Tooltip>>,
) {
    entities
        .into_iter()
        .for_each(|entity| {
            commands
                .entity(entity)
                .try_insert(TooltipState::Hidden);
        });
}

fn process_touch(
    mut commands: Commands,
    style: Res<UiStyles>,
    mouse: Res<MouseState>,
    mut view: Query<&mut Sprite, With<TooltipView>>,
    entities: Query<(Entity, &GlobalTransform, &TouchState, &mut TooltipState, &Tooltip), Changed<TouchState>>,
) {
    let appear_after = 0.5;

    entities
        .into_iter()
        .for_each(|(entity, gt, state, mut tooltip_state, tooltip)| match (state, &mut *tooltip_state, mouse.is_idle_at_least(appear_after)) {
            (TouchState::None, TooltipState::Visible(view), _) | (_, TooltipState::Visible(view), false) => {
                commands.entity(*view).despawn();
                *tooltip_state = TooltipState::Hidden;
            },
            (TouchState::Touching { duration }, TooltipState::Hidden, true) if *duration >= appear_after  => {
                let id = commands.spawn(TooltipView::create(
                    &style, 
                    tooltip, 
                    Transform::from_translation(gt.translation()),
                    entity,
                )).id();
                *tooltip_state = TooltipState::Visible(id);
            },
            (TouchState::Touching { duration }, TooltipState::Visible(view_id), true) => {
                let time = mouse.idle_duration().unwrap_or(0.).min(*duration);

                let progress = (time - 0.5).clamp(0.0, 1.0);
                if let Ok(mut tooltip_sprite) = view.get_mut(*view_id) {
                    tooltip_sprite.custom_size = Some(tooltip.area * progress);
                }
                //let mut tooltip_sprite = view.get_mut(*view_id).expect("maybe not expect?");
            },
            _ => {},

        });
}

