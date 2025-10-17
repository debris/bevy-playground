use bevy::{prelude::*, sprite::Anchor};
use bevy::text::TextBounds;

use crate::styles::UiStyles;
use crate::touch::TouchState;

pub struct TooltipPlugin;

#[derive(Component, Clone)]
pub struct Tooltip {
    pub text: &'static str,
    pub area: Vec2,
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
            Text2d::new(tooltip.text),
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
            //.add_systems(Update, on_mouse_move);
            
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
                .insert(TooltipState::Hidden);
        });
}

fn process_touch(
    mut commands: Commands,
    style: Res<UiStyles>,
    mut view: Query<&mut Sprite, With<TooltipView>>,
    entities: Query<(Entity, &GlobalTransform, &TouchState, &mut TooltipState, &Tooltip), Changed<TouchState>>,
) {
    entities
        .into_iter()
        .for_each(|(entity, gt, state, mut tooltip_state, tooltip)| match (state, &mut *tooltip_state) {
            (TouchState::None, TooltipState::Visible(entity)) => {
                commands.entity(*entity).despawn();
                *tooltip_state = TooltipState::Hidden;
            },
            (TouchState::Touching { duration }, TooltipState::Hidden) if *duration > 0.5 => {
                let id = commands.spawn(TooltipView::create(
                    &style, 
                    tooltip, 
                    Transform::from_translation(gt.translation()),
                    entity,
                )).id();
                *tooltip_state = TooltipState::Visible(id);
            },
            (TouchState::Touching { duration }, TooltipState::Visible(view_id)) => {
                let progress = (duration - 0.5).clamp(0.0, 1.0);
                let mut tooltip_sprite = view.get_mut(*view_id).expect("maybe not expect?");
                tooltip_sprite.custom_size = Some(tooltip.area * progress);
            },
            //(TouchState::None, TooltipState::Hidden) => {},
            _ => {},

        });


            //match state {
                //TouchState::None => {
                    //// TODO:tooltip should be invisible
                    //tooltip_state.set_if_neq(TooltipState::Hidden);
                //},
                //TouchState::Touching { duration } if *duration > 0.5=> {
                    //// make it visible
                    ////tooltip_state.set_if_neq(TooltipState::Visible);
                //},
                //_ => {
                    // nothing here
                //}
            //}
        //});
}

//fn show_tooltip(
    //mut commands: Commands,
    //style: Res<UiStyles>,
    //entities: Query<(Entity, &Tooltip, &TooltipState), Changed<TooltipState>>
//) {
                    //commands
                        //.entity(entity)
                        //.with_child(TooltipView::create(&style, tooltip));
//}

//fn setup_tooltip(
    //mut commands: Commands,
    //style: Res<UiStyles>,
    //entities: Query<(Entity, &TooltipData), (Without<TooltipEntity>, Without<Tooltip>)>
//fn show_tooltip(
//) {
    //if entities.is_empty() {
        //return
    //}

    //for (entity, tooltip_data) in entities {
        //let tooltip_id = commands.spawn((
            //Name::new("Tooltip"),
            //Sprite::from_color(style.tooltip.background_color, Vec2::ZERO),
            //Transform::from_translation(Vec3::Z),
            //Anchor::TOP_LEFT,
            //tooltip_data.clone(),
            //Tooltip,
            //TooltipOwner(entity),
            //TooltipState::Hidden,
            //Visibility::Hidden,
            //children![(
                //Text2d::new(tooltip_data.text),
                //style.tooltip.font.clone(),
                //TextColor(style.tooltip.text_color),
                //TextLayout::new(Justify::Left, LineBreak::WordBoundary),
                //TextBounds::from(tooltip_data.area),
                //Transform::from_translation(Vec3::Z),
                //Anchor::TOP_LEFT,
            //)]
        //)).id();

        //commands.entity(entity).insert(TooltipEntity(tooltip_id));
    //}
//}

//fn prepare_tooltip(
    //style: Res<UiStyles>,
    //entities: Query<(&TouchState, &TooltipEntity, &GlobalTransform), (Changed<Touchable>, Without<Tooltip>)>,
    //mut tooltips: Query<(&mut TooltipState, &mut Visibility, &mut Transform), With<Tooltip>>,
//) {
    //for (touch_state, tooltip, transform) in entities {
        //if let Ok((mut tooltip_state, mut visibility, mut tooltip_transform)) = tooltips.get_mut(tooltip.0) {
            //if touch_state.is_touching() {
                //if tooltip_state.is_hidden() {
                    //*tooltip_state = TooltipState::Pending(Timer::from_seconds(style.tooltip.appear_delay, TimerMode::Once));
                    //tooltip_transform.translation.x = transform.translation().x + 16.0;
                    //tooltip_transform.translation.y = transform.translation().y - 16.0;
                //}
            //} else {
                //*tooltip_state = TooltipState::Hidden;
                //*visibility = Visibility::Hidden;
            //}
        //}
    //}
//}

//fn show_tooltip(
    //time: Res<Time>,
    //style: Res<UiStyles>,
    //mut entities: Query<(&mut TooltipState, &mut Visibility, &mut Sprite, &TooltipData), With<Tooltip>>
//) {
    //for (mut state, mut visibility, mut sprite, data) in &mut entities {
        //if let TooltipState::Pending(timer) = &mut *state {
            //timer.tick(time.delta());
            //if timer.is_finished() {
                //sprite.custom_size = Some(Vec2::ZERO);
                //*state = TooltipState::Appearing(Timer::from_seconds(style.tooltip.appear_animation_time, TimerMode::Once));
                //*visibility = Visibility::Visible;
            //}
        //} else if let TooltipState::Appearing(timer) = &mut *state {
            //timer.tick(time.delta());
            //let fraction = timer.fraction();
            //sprite.custom_size = Some(data.area * fraction);
            //if timer.is_finished() {
                //*state = TooltipState::Visible;
            //}
        //}
    //}
//}

//fn on_mouse_move(
    //mut commands: Commands,
    //cursor_evr: MessageReader<CursorMoved>,
    //mut entities: Query<&mut TooltipState>,
    //tooltips: Query<(Entity, &TooltipView), With<TooltipView>>,
//) {
    //if cursor_evr.is_empty() {
        //return
    //}

    //tooltips
        //.into_iter()
        //.for_each(|(entity, view)| {
            //commands.entity(entity).despawn();
            //println!("here!");
            ////let mut state = entities.get_mut(view.0).expect("to exist");
            //println!("here2!");
            //[>state = TooltipState::Hidden; 
        //});
//}

