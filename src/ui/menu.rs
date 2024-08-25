use bevy::prelude::*;

use crate::ui::{
    constants::{BUTTON_MARGIN, BUTTON_PADDING},
    interaction::InteractionId,
};

pub trait ButtonBuilderExt {
    fn spawn_button<E: InteractionId + 'static>(&mut self, text: &str, button_event: E) -> Entity;
}

impl<'a> ButtonBuilderExt for ChildBuilder<'a> {
    fn spawn_button<E: InteractionId + 'static>(&mut self, text: &str, button_event: E) -> Entity {
        self.spawn((
            Name::new(format!("{} Button", text)),
            crate::ui::interaction::InteractionIdComponent(button_event),
            ButtonBundle {
                style: Style {
                    margin: BUTTON_MARGIN,
                    padding: BUTTON_PADDING,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: crate::ui::constants::BUTTON_BORDER_SIZE,
                    ..Default::default()
                },
                background_color: crate::ui::constants::NORMAL_BUTTON.into(),
                border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
                border_radius: crate::ui::constants::BUTTON_BORDER_RADIUS,
                ..Default::default()
            },
        ))
        .with_children(|cmd| {
            cmd.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: crate::ui::constants::BUTTON_FONT_SIZE,
                    color: crate::ui::constants::TEXT_COLOR,
                    ..Default::default()
                },
            ));
        })
        .id()
    }
}

macro_rules! spawn_menu {
    ($cmd:ident, $state:expr, [$(($button_name:literal, $button_event:expr)),*$(,)?]) => {
        {
            use crate::ui::menu::ButtonBuilderExt;
            let id = $cmd.spawn((
                StateScoped($state),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: crate::ui::constants::BACKDROP_COLOR.into(),
                    ..Default::default()
                },
            ))
            .with_children(|cmd| {
                cmd.spawn(NodeBundle {
                        style: Style {
                            width: crate::ui::constants::MENU_BUTTON_WIDTH,
                            flex_direction: FlexDirection::Column,
                            display: Display::Flex,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                .with_children(|cmd| {
                    $(
                        cmd.spawn_button($button_name, $button_event);
                    )*
                });
            }).id();
            id
        }
    };
}
