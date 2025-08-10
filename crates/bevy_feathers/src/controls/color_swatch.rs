use bevy_asset::Handle;
use bevy_color::Alpha;
use bevy_ecs::{
    bundle::Bundle, children, component::Component, reflect::ReflectComponent, spawn::SpawnRelated,
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_ui::{BackgroundColor, BorderRadius, Node, PositionType, Val};
use bevy_ui_render::ui_material::MaterialNode;

use crate::{
    alpha_pattern::{AlphaPattern, AlphaPatternMaterial},
    palette,
};

pub mod tokens {
    pub mod sizing {
        use crate::tokens::sizing::*;

        pub const SIZE: f32 = control::height::STANDARD;
    }

    pub mod radii {
        use crate::tokens::radii;

        pub const BORDER: f32 = radii::STANDARD;
    }
}

/// Marker identifying a color swatch.
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
pub struct ColorSwatch;

/// Marker identifying the color swatch foreground, the piece that actually displays the color
/// in front of the alpha pattern. This exists so that users can reach in and change the color
/// dynamically.
#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Clone, Default)]
pub struct ColorSwatchFg;

/// Template function to spawn a color swatch.
///
/// # Arguments
/// * `overrides` - a bundle of components that are merged in with the normal swatch components.
pub fn color_swatch<B: Bundle>(overrides: B) -> impl Bundle {
    (
        Node {
            height: Val::Px(tokens::sizing::SIZE),
            min_width: Val::Px(tokens::sizing::SIZE),
            ..Default::default()
        },
        ColorSwatch,
        AlphaPattern,
        MaterialNode::<AlphaPatternMaterial>(Handle::default()),
        BorderRadius::all(Val::Px(tokens::radii::BORDER)),
        overrides,
        children![(
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                right: Val::Px(0.),
                ..Default::default()
            },
            ColorSwatchFg,
            BackgroundColor(palette::ACCENT.with_alpha(0.5)),
            BorderRadius::all(Val::Px(tokens::radii::BORDER))
        ),],
    )
}
