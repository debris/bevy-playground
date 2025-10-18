use bevy::prelude::*;

use crate::{grid::Index, grid_highlight::GridHighlightRequest, scale_on_touch::ScaleOnTouch, touch::{TouchArea, TouchState}};

pub struct CardPlugin;

#[derive(Component)]
pub struct Card;


impl Card {
    pub fn create(position: Vec2) -> impl Bundle {
        let card_area = Vec2::new(64., 96.);

        (
            Name::new("Card"),
            Card,
            Sprite::from_color(Color::linear_rgba(0., 0., 1., 0.2), card_area),
            TouchArea {
                area: card_area,
            },
            ScaleOnTouch(1.2),
            Transform::from_xyz(position.x, position.y, 0.),
        )
    }
}

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, card_highlight);
    }
}

fn card_highlight(
    cards: Query<&TouchState, (With<Card>, Changed<TouchState>)>,
    mut request: MessageWriter<GridHighlightRequest>
) {
    cards
        .into_iter()
        .for_each(|state| {
            if state.is_touching() {
                request.write(GridHighlightRequest {
                    tiles: vec![
                        Index::new(0, 0),
                        Index::new(1, 1),
                        Index::new(1, 2),
                    ]
                });
            } else {
                request.write(GridHighlightRequest {
                    tiles: vec![]
                });
            }
        });
}

