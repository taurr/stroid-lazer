macro_rules! spawn_menu {
    ($cmd:ident, $state:expr, [$(($button_name:literal, $button_event:expr)),*$(,)?]) => {
        {
            let id = $cmd.spawn((
                StateScoped($state),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: crate::ui::constants::BACKDROP_COLOR.into(),
                    ..Default::default()
                },
            ))
            .with_children(|cmd| {
                cmd.spawn(NodeBundle {
                        style: Style {
                            width: crate::ui::constants::BUTTON_WIDTH,
                            row_gap: crate::ui::constants::BUTTON_GAP_HEIGHT,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                .with_children(|cmd| {
                    $(
                        let button_text = $button_name;
                        let button_name = format!("{} Button", $button_name);
                        cmd.spawn((
                                Name::new(button_name),
                                crate::ui::interaction::InteractionIdComponent($button_event),
                                ButtonBundle {
                                    style: Style {
                                        height: crate::ui::constants::BUTTON_HEIGHT,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: crate::ui::constants::BORDER_SIZE,
                                        ..Default::default()
                                    },
                                    background_color: crate::ui::constants::NORMAL_BUTTON.into(),
                                    border_color: crate::ui::constants::BORDER_COLOR.into(),
                                    border_radius: crate::ui::constants::BORDER_RADIUS,
                                    ..Default::default()
                                },
                            ))
                        .with_children(|cmd| {
                            cmd.spawn(TextBundle::from_section(
                                button_text,
                                TextStyle {
                                    font_size: crate::ui::constants::BUTTON_FONT_SIZE,
                                    color: crate::ui::constants::TEXT_COLOR,
                                    ..Default::default()
                                },
                            ));
                        });
                    )*
                });
            }).id();
            id
        }
    };
}
