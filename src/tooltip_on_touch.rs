use bevy::{platform::collections::HashMap, prelude::*};

use crate::core::prelude::*;

pub struct TooltipOnTouchPlugin;

#[derive(Component)]
pub struct TooltipOnTouch(pub String);

#[derive(Component)]
pub struct TooltipView;

#[derive(Component, Default)]
struct Tooltips {
    text_by_entity: HashMap<Entity, String>,
}

#[derive(Component, Deref, DerefMut, PartialEq)]
struct TooltipViewNeedsRefresh(pub bool);

impl Tooltips {
    fn insert_tooltip(&mut self, entity: Entity, tooltip: String) {
        //self.text_by_entity.clear();
        self.text_by_entity.insert(entity, tooltip);
    }

    fn remove_tooltip(&mut self, entity: Entity) {
        self.text_by_entity.remove(&entity);
    }

    fn any(&self) -> Option<&String> {
        self.text_by_entity.values().next()
    }
}

impl Default for TooltipOnTouch {
    fn default() -> Self {
        TooltipOnTouch("".into())
    }
}

impl Plugin for TooltipOnTouchPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_tooltip_view)
            .add_systems(Update, tooltip_on_touch)
            .add_systems(Update, display_tooltip);
    }
}

fn setup_tooltip_view(
    mut commands: Commands,
    tooltip_view: Single<Entity, Added<TooltipView>>
) {
    commands
        .entity(*tooltip_view)
        .try_insert((
            Text2d::new(""),
            Tooltips::default(),
            TooltipViewNeedsRefresh(true),
        ));
}

fn tooltip_on_touch(
    mut tooltip_view: Single<(&mut Tooltips, &mut TooltipViewNeedsRefresh), With<TooltipView>>,
    entities: Query<(Entity, &TooltipOnTouch, &TouchState), Changed<TouchState>>,
) {
    entities.into_iter()
        .for_each(|(entity, tooltip, state)| {
            let (ref mut tooltips, ref mut needs_refresh) = *tooltip_view;
            match state {
                TouchState::JustTouched => {
                    tooltips.insert_tooltip(entity, tooltip.0.clone());
                    needs_refresh.set_if_neq(TooltipViewNeedsRefresh(true));
                },
                TouchState::None => {
                    tooltips.remove_tooltip(entity);
                    needs_refresh.set_if_neq(TooltipViewNeedsRefresh(true));
                },
                _ => {},
            }
        });
}

fn display_tooltip(
    mut tooltip_view: Single<(&Tooltips, &mut Text2d, &mut TooltipViewNeedsRefresh), Changed<TooltipViewNeedsRefresh>>
) {
    let (ref tooltips, ref mut text, ref mut needs_refresh) = *tooltip_view;
    if needs_refresh.0 == true {
        // does not trigger changed
        needs_refresh.0 = false;
        if let Some(tooltip) = tooltips.any() {
            **text = Text2d::new(tooltip);
        } else {
            **text = Text2d::new("");
        }
    }

}
