//! The standard `bevy_feathers` dark theme.
use crate::{palette, tokens::color::*};
use bevy_color::{Alpha, Luminance};
use bevy_platform::collections::HashMap;

use crate::theme::ThemeProps;

/// Create a [`ThemeProps`] object and populate it with the colors for the default dark theme.
pub fn create_dark_theme() -> ThemeProps {
    ThemeProps {
        color: HashMap::from([
            // Accent primary colors.
            (
                accent::primary::DISABLED.into(),
                palette::ACCENT.with_alpha(0.5),
            ),
            (
                accent::primary::MUTED.into(),
                palette::ACCENT.with_alpha(0.6),
            ),
            (accent::primary::BASE.into(), palette::ACCENT),
            (
                accent::primary::FOCUSED.into(),
                palette::ACCENT.lighter(0.05),
            ),
            (accent::primary::ACTIVE.into(), palette::ACCENT.lighter(0.1)),
            // Status colors.
            (status::ERROR.into(), palette::ERROR),
            (status::WARNING.into(), palette::WARNING),
            (status::SUCCESS.into(), palette::SUCCESS),
            (status::INFO.into(), palette::INFO),
            // Axis colors
            (axis::X.into(), palette::X_AXIS),
            (axis::Y.into(), palette::Y_AXIS),
            (axis::Z.into(), palette::Z_AXIS),
            // Container colors
            (container::DARK.into(), palette::GRAY_0),
            (container::BASE.into(), palette::GRAY_1),
            (container::LIGHT.into(), palette::GRAY_2),
            // Surface colors
            (surface::DISABLED.into(), palette::GRAY_2.with_alpha(0.5)),
            (surface::MUTED.into(), palette::GRAY_2.with_alpha(0.6)),
            (surface::BASE.into(), palette::GRAY_3),
            (surface::FOCUSED.into(), palette::GRAY_3.lighter(0.05)),
            (surface::ACTIVE.into(), palette::GRAY_3.lighter(0.1)),
            // Control colors
            (control::DISABLED.into(), palette::GRAY_1.with_alpha(0.5)),
            (control::MUTED.into(), palette::GRAY_1.with_alpha(0.6)),
            (control::BASE.into(), palette::GRAY_2),
            (control::FOCUSED.into(), palette::GRAY_2.lighter(0.05)),
            (control::ACTIVE.into(), palette::GRAY_2.lighter(0.1)),
            // Border colors
            (
                border::DISABLED.into(),
                palette::WARM_GRAY_1.with_alpha(0.5),
            ),
            (border::MUTED.into(), palette::WARM_GRAY_1.with_alpha(0.6)),
            (border::BASE.into(), palette::WARM_GRAY_1),
            (border::FOCUSED.into(), palette::WARM_GRAY_1.lighter(0.5)),
            (border::ACTIVE.into(), palette::WARM_GRAY_1.lighter(0.6)),
            // Text colors
            (text::DISABLED.into(), palette::WHITE.with_alpha(0.5)),
            (text::MUTED.into(), palette::WHITE.with_alpha(0.6)),
            (text::BASE.into(), palette::WHITE),
            (text::FOCUSED.into(), palette::WHITE),
            (text::ACTIVE.into(), palette::WHITE),
            // Selection colors
            (selection::BASE.into(), palette::WHITE.with_alpha(0.1)),
            (selection::FOCUSED.into(), palette::WHITE.with_alpha(0.15)),
        ]),
    }
}
