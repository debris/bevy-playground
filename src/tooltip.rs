use bevy::{prelude::*, sprite::Anchor};
use bevy::text::TextBounds;

use crate::styles::UiStyles;
use crate::touch::Touchable;

pub struct TooltipPlugin;

#[derive(Component, Clone)]
pub struct TooltipData {
    pub text: &'static str,
    pub area: Vec2,
}

#[derive(Component)]
pub struct Tooltip;

#[derive(Component)]
pub struct TooltipEntity(Entity);

#[allow(dead_code)]
#[derive(Component)]
pub struct TooltipOwner(Entity);

#[derive(Component)]
pub enum TooltipState {
    Hidden,
    Pending(Timer),
    Appearing(Timer),
    Visible,
}

impl TooltipState {
    fn is_hidden(&self) -> bool {
       match *self {
           TooltipState::Hidden => true,
           _ => false,
       }
    }
}

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_tooltip)
            .add_systems(Update, prepare_tooltip)
            .add_systems(Update, show_tooltip)
            .add_systems(Update, on_mouse_move);
    }
}

fn setup_tooltip(
    mut commands: Commands,
    style: Res<UiStyles>,
    entities: Query<(Entity, &TooltipData), (Without<TooltipEntity>, Without<Tooltip>)>
) {
    if entities.is_empty() {
        return
    }

    for (entity, tooltip_data) in entities {
        let tooltip_id = commands.spawn((
            Name::new("Tooltip"),
            Sprite::from_color(style.tooltip.background_color, Vec2::ZERO),
            Transform::from_xyz(0., 0., 0.),
            Anchor::TOP_LEFT,
            tooltip_data.clone(),
            Tooltip,
            TooltipOwner(entity),
            TooltipState::Hidden,
            Visibility::Hidden,
            children![(
                Text2d::new(tooltip_data.text),
				style.tooltip.font.clone(),
                TextColor(style.tooltip.text_color),
			    TextLayout::new(Justify::Left, LineBreak::WordBoundary),
                TextBounds::from(tooltip_data.area),
                Transform::from_translation(Vec3::Z),
                Anchor::TOP_LEFT,
            )]
        )).id();

        commands.entity(entity).insert(TooltipEntity(tooltip_id));
    }
}

fn prepare_tooltip(
    style: Res<UiStyles>,
    entities: Query<(&Touchable, &TooltipEntity, &GlobalTransform), (Changed<Touchable>, Without<Tooltip>)>,
    mut tooltips: Query<(&mut TooltipState, &mut Visibility, &mut Transform), With<Tooltip>>,
) {
    for (touchable, tooltip, transform) in entities {
        if let Ok((mut tooltip_state, mut visibility, mut tooltip_transform)) = tooltips.get_mut(tooltip.0) {
            if touchable.touched {
                if tooltip_state.is_hidden() {
                    *tooltip_state = TooltipState::Pending(Timer::from_seconds(style.tooltip.appear_delay, TimerMode::Once));
                    tooltip_transform.translation.x = transform.translation().x + 16.0;
                    tooltip_transform.translation.y = transform.translation().y - 16.0;
                }
            } else {
                *tooltip_state = TooltipState::Hidden;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn show_tooltip(
    time: Res<Time>,
    style: Res<UiStyles>,
    mut entities: Query<(&mut TooltipState, &mut Visibility, &mut Sprite, &TooltipData), With<Tooltip>>
) {
    for (mut state, mut visibility, mut sprite, data) in &mut entities {
        if let TooltipState::Pending(timer) = &mut *state {
            timer.tick(time.delta());
            if timer.is_finished() {
                sprite.custom_size = Some(Vec2::ZERO);
                *state = TooltipState::Appearing(Timer::from_seconds(style.tooltip.appear_animation_time, TimerMode::Once));
                *visibility = Visibility::Visible;
            }
        } else if let TooltipState::Appearing(timer) = &mut *state {
            timer.tick(time.delta());
            let fraction = timer.fraction();
            sprite.custom_size = Some(data.area * fraction);
            if timer.is_finished() {
                *state = TooltipState::Visible;
            }
        }
    }
}

fn on_mouse_move(
    cursor_evr: MessageReader<CursorMoved>,
    mut tooltips: Query<(&mut TooltipState, &mut Visibility), With<Tooltip>>,
) {
    if cursor_evr.is_empty() {
        return
    }

    for (mut tooltip_state, mut visibility) in &mut tooltips {
        *tooltip_state = TooltipState::Hidden;
        *visibility = Visibility::Hidden;
    }
}

