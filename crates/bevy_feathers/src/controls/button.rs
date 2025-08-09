use bevy_app::{Plugin, PreUpdate};
use bevy_core_widgets::{Activate, Callback, CoreButton};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    hierarchy::{ChildOf, Children},
    lifecycle::RemovedComponents,
    query::{Added, Changed, Has, Or},
    reflect::ReflectComponent,
    schedule::IntoScheduleConfigs,
    spawn::{SpawnRelated, SpawnableList},
    system::{Commands, In, Query},
};
use bevy_input_focus::tab_navigation::TabIndex;
use bevy_picking::{hover::Hovered, PickingSystems};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_ui::{AlignItems, InteractionDisabled, JustifyContent, Node, Pressed, UiRect, Val};

use crate::{
    constants::{fonts, size},
    cursor::EntityCursor,
    font_styles::InheritableFont,
    handle_or_path::HandleOrPath,
    rounded_corners::RoundedCorners,
    theme::{ThemeBackgroundColor, ThemeFontColor},
};

pub mod tokens {
    pub mod color {
        use crate::tokens::color::*;

        pub const BACKGROUND: &str = surface::BASE;
        pub const BACKGROUND_DISABLED: &str = surface::DISABLED;
        pub const BACKGROUND_HOVERED: &str = surface::FOCUSED;
        pub const BACKGROUND_PRESSED: &str = surface::ACTIVE;

        pub const BACKGROUND_PRIMARY: &str = surface::accent::BASE;
        pub const BACKGROUND_PRIMARY_DISABLED: &str = surface::accent::DISABLED;
        pub const BACKGROUND_PRIMARY_HOVERED: &str = surface::accent::FOCUSED;
        pub const BACKGROUND_PRIMARY_PRESSED: &str = surface::accent::ACTIVE;

        pub const TEXT: &str = foreground::BASE;
        pub const TEXT_DISABLED: &str = foreground::DISABLED;

        pub const TEXT_PRIMARY: &str = foreground::BASE;
        pub const TEXT_PRIMARY_DISABLED: &str = foreground::DISABLED;
    }
}

/// Color variants for buttons. This also functions as a component used by the dynamic styling
/// system to identify which entities are buttons.
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
pub enum ButtonVariant {
    /// The standard button appearance
    #[default]
    Normal,
    /// A button with a more prominent color, this is used for "call to action" buttons,
    /// default buttons for dialog boxes, and so on.
    Primary,
}

/// Parameters for the button template, passed to [`button`] function.
#[derive(Default)]
pub struct ButtonProps {
    /// Color variant for the button.
    pub variant: ButtonVariant,
    /// Rounded corners options
    pub corners: RoundedCorners,
    /// Click handler
    pub on_click: Callback<In<Activate>>,
}

/// Template function to spawn a button.
///
/// # Arguments
/// * `props` - construction properties for the button.
/// * `overrides` - a bundle of components that are merged in with the normal button components.
/// * `children` - a [`SpawnableList`] of child elements, such as a label or icon for the button.
pub fn button<C: SpawnableList<ChildOf> + Send + Sync + 'static, B: Bundle>(
    props: ButtonProps,
    overrides: B,
    children: C,
) -> impl Bundle {
    (
        Node {
            height: size::ROW_HEIGHT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
            flex_grow: 1.0,
            ..Default::default()
        },
        CoreButton {
            on_activate: props.on_click,
        },
        props.variant,
        Hovered::default(),
        EntityCursor::System(bevy_window::SystemCursorIcon::Pointer),
        TabIndex(0),
        props.corners.to_border_radius(4.0),
        ThemeBackgroundColor(tokens::color::BACKGROUND),
        ThemeFontColor(tokens::color::TEXT),
        InheritableFont {
            font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
            font_size: 14.0,
        },
        overrides,
        Children::spawn(children),
    )
}

fn update_button_styles(
    q_buttons: Query<
        (
            Entity,
            &ButtonVariant,
            Has<InteractionDisabled>,
            Has<Pressed>,
            &Hovered,
            &ThemeBackgroundColor,
            &ThemeFontColor,
        ),
        Or<(Changed<Hovered>, Added<Pressed>, Added<InteractionDisabled>)>,
    >,
    mut commands: Commands,
) {
    for (button_ent, variant, disabled, pressed, hovered, bg_color, font_color) in q_buttons.iter()
    {
        set_button_colors(
            button_ent,
            variant,
            disabled,
            pressed,
            hovered.0,
            bg_color,
            font_color,
            &mut commands,
        );
    }
}

fn update_button_styles_remove(
    q_buttons: Query<(
        Entity,
        &ButtonVariant,
        Has<InteractionDisabled>,
        Has<Pressed>,
        &Hovered,
        &ThemeBackgroundColor,
        &ThemeFontColor,
    )>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
    mut removed_pressed: RemovedComponents<Pressed>,
    mut commands: Commands,
) {
    removed_disabled
        .read()
        .chain(removed_pressed.read())
        .for_each(|ent| {
            if let Ok((button_ent, variant, disabled, pressed, hovered, bg_color, font_color)) =
                q_buttons.get(ent)
            {
                set_button_colors(
                    button_ent,
                    variant,
                    disabled,
                    pressed,
                    hovered.0,
                    bg_color,
                    font_color,
                    &mut commands,
                );
            }
        });
}

fn set_button_colors(
    button_ent: Entity,
    variant: &ButtonVariant,
    disabled: bool,
    pressed: bool,
    hovered: bool,
    bg_color: &ThemeBackgroundColor,
    font_color: &ThemeFontColor,
    commands: &mut Commands,
) {
    let bg_token = match (variant, disabled, pressed, hovered) {
        (ButtonVariant::Normal, true, _, _) => tokens::color::BACKGROUND_DISABLED,
        (ButtonVariant::Normal, false, true, _) => tokens::color::BACKGROUND_PRESSED,
        (ButtonVariant::Normal, false, false, true) => tokens::color::BACKGROUND_HOVERED,
        (ButtonVariant::Normal, false, false, false) => tokens::color::BACKGROUND,
        (ButtonVariant::Primary, true, _, _) => tokens::color::BACKGROUND_PRIMARY_DISABLED,
        (ButtonVariant::Primary, false, true, _) => tokens::color::BACKGROUND_PRIMARY_PRESSED,
        (ButtonVariant::Primary, false, false, true) => tokens::color::BACKGROUND_PRIMARY_HOVERED,
        (ButtonVariant::Primary, false, false, false) => tokens::color::BACKGROUND_PRIMARY,
    };

    let font_color_token = match (variant, disabled) {
        (ButtonVariant::Normal, true) => tokens::color::TEXT_DISABLED,
        (ButtonVariant::Normal, false) => tokens::color::TEXT,
        (ButtonVariant::Primary, true) => tokens::color::TEXT_PRIMARY_DISABLED,
        (ButtonVariant::Primary, false) => tokens::color::TEXT_PRIMARY,
    };

    // Change background color
    if bg_color.0 != bg_token {
        commands
            .entity(button_ent)
            .insert(ThemeBackgroundColor(bg_token));
    }

    // Change font color
    if font_color.0 != font_color_token {
        commands
            .entity(button_ent)
            .insert(ThemeFontColor(font_color_token));
    }
}

/// Plugin which registers the systems for updating the button styles.
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PreUpdate,
            (update_button_styles, update_button_styles_remove).in_set(PickingSystems::Last),
        );
    }
}
