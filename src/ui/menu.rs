macro_rules! spawn_menu {
    ($cmd:ident, $state:expr, [$(($button_name:literal, $button_event:expr)),*$(,)?]) => {
        {
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
                            row_gap: crate::ui::constants::BUTTON_GAP_HEIGHT,
                            display: Display::Flex,
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
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: crate::ui::constants::BUTTON_BORDER_SIZE,
                                        padding: crate::ui::constants::BUTTON_PADDING,
                                        ..Default::default()
                                    },
                                    border_radius: crate::ui::constants::BUTTON_BORDER_RADIUS,
                                    background_color: crate::ui::constants::NORMAL_BUTTON.into(),
                                    border_color: crate::ui::constants::BUTTON_BORDER_COLOR.into(),
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
