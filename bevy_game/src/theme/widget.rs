use bevy::{
    asset::AssetLoader,
    ecs::{entity, system::EntityCommands},
    prelude::*,
    text::FontLoader,
};
use ui_palette::{
    BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, NODE_BACKGROUND,
};

use super::UiFont;
use crate::theme::prelude::*;

pub trait Widget {
    /// Spawn a simple button with text
    fn button(&mut self, text: impl Into<String>, font: &Res<UiFont>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widget for T {
    // 按钮组件
    fn button(&mut self, text: impl Into<String>, font: &Res<UiFont>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Px(12.0)),
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..Default::default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 24.0,
                        font: font.0.clone(),
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let entity = self.spawn((
            Name::new("label"),
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 23.0,
                    ..Default::default()
                },
            )
            .with_style(Style {
                width: Val::Px(500.0),
                ..default()
            }),
        ));
        entity
    }
}

pub trait Containers {
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Ui root"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
    }
}
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
