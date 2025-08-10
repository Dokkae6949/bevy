use bevy_app::{Plugin, PreUpdate};
use bevy_camera::visibility::Visibility;
use bevy_core_widgets::{Callback, CoreCheckbox, ValueChange};
use bevy_ecs::{
    bundle::Bundle,
    children,
    component::Component,
    entity::Entity,
    hierarchy::{ChildOf, Children},
    lifecycle::RemovedComponents,
    query::{Added, Changed, Has, Or, With},
    reflect::ReflectComponent,
    schedule::IntoScheduleConfigs,
    spawn::{Spawn, SpawnRelated, SpawnableList},
    system::{Commands, In, Query},
};
use bevy_input_focus::tab_navigation::TabIndex;
use bevy_math::Rot2;
use bevy_picking::{hover::Hovered, PickingSystems};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_ui::{
    AlignItems, BorderRadius, Checked, Display, FlexDirection, InteractionDisabled, JustifyContent,
    Node, PositionType, UiRect, UiTransform, Val,
};

use crate::{
    cursor::EntityCursor,
    font_styles::InheritableFont,
    handle_or_path::HandleOrPath,
    theme::{ThemeBackgroundColor, ThemeBorderColor, ThemeFontColor},
};

pub mod tokens {
    pub mod color {
        use crate::tokens::color::*;

        pub const BACKGROUND: &str = surface::BASE;
        pub const BACKGROUND_DISABLED: &str = surface::DISABLED;

        pub const BACKGROUND_CHECKED: &str = accent::BASE;
        pub const BACKGROUND_CHECKED_DISABLED: &str = surface::DISABLED;

        pub const BORDER: &str = border::BASE;
        pub const BORDER_HOVERED: &str = border::FOCUSED;
        pub const BORDER_DISABLED: &str = surface::DISABLED;

        pub const MARK: &str = foreground::ELEVATED;
        pub const MARK_DISABLED: &str = foreground::DISABLED;

        pub const TEXT: &str = foreground::BASE;
        pub const TEXT_DISABLED: &str = foreground::DISABLED;
    }

    pub mod sizing {
        use crate::tokens::sizing::*;

        pub const SIZE: f32 = control::size::STANDARD;
    }

    pub mod spacing {
        use crate::tokens::spacing;

        pub const SPACING: f32 = spacing::XS;
    }

    pub mod radii {
        use crate::tokens::radii;

        pub const BORDER_INNER: f32 = radii::SHARP;
        pub const BORDER_OUTER: f32 = radii::STANDARD;
    }

    pub mod typography {
        use crate::tokens::typography::*;

        pub const SIZE: f32 = size::body::MD;
        pub const FAMILY: &str = family::REGULAR;
    }
}

/// Parameters for the checkbox template, passed to [`checkbox`] function.
#[derive(Default)]
pub struct CheckboxProps {
    /// Change handler
    pub on_change: Callback<In<ValueChange<bool>>>,
}

/// Marker for the checkbox frame (contains both checkbox and label)
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
struct CheckboxFrame;

/// Marker for the checkbox outline
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
struct CheckboxOutline;

/// Marker for the checkbox check mark
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
struct CheckboxMark;

/// Template function to spawn a checkbox.
///
/// # Arguments
/// * `props` - construction properties for the checkbox.
/// * `overrides` - a bundle of components that are merged in with the normal checkbox components.
/// * `label` - the label of the checkbox.
pub fn checkbox<C: SpawnableList<ChildOf> + Send + Sync + 'static, B: Bundle>(
    props: CheckboxProps,
    overrides: B,
    label: C,
) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            column_gap: Val::Px(tokens::spacing::SPACING),
            ..Default::default()
        },
        CoreCheckbox {
            on_change: props.on_change,
        },
        CheckboxFrame,
        Hovered::default(),
        EntityCursor::System(bevy_window::SystemCursorIcon::Pointer),
        TabIndex(0),
        ThemeFontColor(tokens::color::TEXT),
        InheritableFont {
            font: HandleOrPath::Path(tokens::typography::FAMILY.to_owned()),
            font_size: tokens::typography::SIZE,
        },
        overrides,
        Children::spawn((
            Spawn((
                Node {
                    width: Val::Px(tokens::sizing::SIZE),
                    height: Val::Px(tokens::sizing::SIZE),
                    border: UiRect::all(Val::Px(tokens::radii::BORDER_INNER)),
                    ..Default::default()
                },
                CheckboxOutline,
                BorderRadius::all(Val::Px(tokens::radii::BORDER_OUTER)),
                ThemeBackgroundColor(tokens::color::BACKGROUND),
                ThemeBorderColor(tokens::color::BORDER),
                children![(
                    // Cheesy checkmark: rotated node with L-shaped border.
                    // TODO: Center checkmark regardless of checkbox size.
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(4.0),
                        top: Val::Px(0.0),
                        width: Val::Px(6.),
                        height: Val::Px(11.),
                        border: UiRect {
                            bottom: Val::Px(2.0),
                            right: Val::Px(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    UiTransform::from_rotation(Rot2::FRAC_PI_4),
                    CheckboxMark,
                    ThemeBorderColor(tokens::color::MARK),
                )],
            )),
            label,
        )),
    )
}

fn update_checkbox_styles(
    q_checkboxes: Query<
        (
            Entity,
            Has<InteractionDisabled>,
            Has<Checked>,
            &Hovered,
            &ThemeFontColor,
        ),
        (
            With<CheckboxFrame>,
            Or<(Changed<Hovered>, Added<Checked>, Added<InteractionDisabled>)>,
        ),
    >,
    q_children: Query<&Children>,
    mut q_outline: Query<(&ThemeBackgroundColor, &ThemeBorderColor), With<CheckboxOutline>>,
    mut q_mark: Query<&ThemeBorderColor, With<CheckboxMark>>,
    mut commands: Commands,
) {
    for (checkbox_ent, disabled, checked, hovered, font_color) in q_checkboxes.iter() {
        let Some(outline_ent) = q_children
            .iter_descendants(checkbox_ent)
            .find(|en| q_outline.contains(*en))
        else {
            continue;
        };
        let Some(mark_ent) = q_children
            .iter_descendants(checkbox_ent)
            .find(|en| q_mark.contains(*en))
        else {
            continue;
        };
        let (outline_bg, outline_border) = q_outline.get_mut(outline_ent).unwrap();
        let mark_color = q_mark.get_mut(mark_ent).unwrap();
        set_checkbox_colors(
            checkbox_ent,
            outline_ent,
            mark_ent,
            disabled,
            checked,
            hovered.0,
            outline_bg,
            outline_border,
            mark_color,
            font_color,
            &mut commands,
        );
    }
}

fn update_checkbox_styles_remove(
    q_checkboxes: Query<
        (
            Entity,
            Has<InteractionDisabled>,
            Has<Checked>,
            &Hovered,
            &ThemeFontColor,
        ),
        With<CheckboxFrame>,
    >,
    q_children: Query<&Children>,
    mut q_outline: Query<(&ThemeBackgroundColor, &ThemeBorderColor), With<CheckboxOutline>>,
    mut q_mark: Query<&ThemeBorderColor, With<CheckboxMark>>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
    mut removed_checked: RemovedComponents<Checked>,
    mut commands: Commands,
) {
    removed_disabled
        .read()
        .chain(removed_checked.read())
        .for_each(|ent| {
            if let Ok((checkbox_ent, disabled, checked, hovered, font_color)) =
                q_checkboxes.get(ent)
            {
                let Some(outline_ent) = q_children
                    .iter_descendants(checkbox_ent)
                    .find(|en| q_outline.contains(*en))
                else {
                    return;
                };
                let Some(mark_ent) = q_children
                    .iter_descendants(checkbox_ent)
                    .find(|en| q_mark.contains(*en))
                else {
                    return;
                };
                let (outline_bg, outline_border) = q_outline.get_mut(outline_ent).unwrap();
                let mark_color = q_mark.get_mut(mark_ent).unwrap();
                set_checkbox_colors(
                    checkbox_ent,
                    outline_ent,
                    mark_ent,
                    disabled,
                    checked,
                    hovered.0,
                    outline_bg,
                    outline_border,
                    mark_color,
                    font_color,
                    &mut commands,
                );
            }
        });
}

fn set_checkbox_colors(
    checkbox_ent: Entity,
    outline_ent: Entity,
    mark_ent: Entity,
    disabled: bool,
    checked: bool,
    hovered: bool,
    outline_bg: &ThemeBackgroundColor,
    outline_border: &ThemeBorderColor,
    mark_color: &ThemeBorderColor,
    font_color: &ThemeFontColor,
    commands: &mut Commands,
) {
    let outline_border_token = match (disabled, hovered) {
        (true, _) => tokens::color::BORDER_DISABLED,
        (false, true) => tokens::color::BORDER_HOVERED,
        _ => tokens::color::BORDER,
    };

    let outline_bg_token = match (disabled, checked) {
        (true, true) => tokens::color::BACKGROUND_CHECKED_DISABLED,
        (true, false) => tokens::color::BACKGROUND_DISABLED,
        (false, true) => tokens::color::BACKGROUND_CHECKED,
        (false, false) => tokens::color::BACKGROUND,
    };

    let mark_token = match disabled {
        true => tokens::color::MARK_DISABLED,
        false => tokens::color::MARK,
    };

    let font_color_token = match disabled {
        true => tokens::color::TEXT_DISABLED,
        false => tokens::color::TEXT,
    };

    // Change outline background
    if outline_bg.0 != outline_bg_token {
        commands
            .entity(outline_ent)
            .insert(ThemeBackgroundColor(outline_bg_token));
    }

    // Change outline border
    if outline_border.0 != outline_border_token {
        commands
            .entity(outline_ent)
            .insert(ThemeBorderColor(outline_border_token));
    }

    // Change mark color
    if mark_color.0 != mark_token {
        commands
            .entity(mark_ent)
            .insert(ThemeBorderColor(mark_token));
    }

    // Change mark visibility
    commands.entity(mark_ent).insert(match checked {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    });

    // Change font color
    if font_color.0 != font_color_token {
        commands
            .entity(checkbox_ent)
            .insert(ThemeFontColor(font_color_token));
    }
}

/// Plugin which registers the systems for updating the checkbox styles.
pub struct CheckboxPlugin;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PreUpdate,
            (update_checkbox_styles, update_checkbox_styles_remove).in_set(PickingSystems::Last),
        );
    }
}
