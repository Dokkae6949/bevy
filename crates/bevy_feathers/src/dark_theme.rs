//! The standard `bevy_feathers` dark theme.
use crate::{palette, tokens::color::*};
use bevy_color::{Alpha, Luminance};
use bevy_platform::collections::HashMap;

use crate::theme::ThemeProps;

/// Create a [`ThemeProps`] object and populate it with the colors for the default dark theme.
pub fn create_dark_theme() -> ThemeProps {
    ThemeProps {
        color: HashMap::from([
            // ------------------------------------
            // CONTAINERS
            (container::WINDOW.into(), palette::GRAY_0),
            (container::PRIMARY.into(), palette::GRAY_1),
            (container::SECONDARY.into(), palette::GRAY_2),
            // ------------------------------------
            // SURFACE
            (surface::CONTRAST.into(), palette::GRAY_0),
            (surface::DISABLED.into(), palette::GRAY_2.with_alpha(0.5)),
            (surface::MUTED.into(), palette::GRAY_2.with_alpha(0.6)),
            (surface::BASE.into(), palette::GRAY_3),
            (surface::ELEVATED.into(), palette::GRAY_3.lighter(0.05)),
            (surface::FOCUSED.into(), palette::GRAY_3.lighter(0.1)),
            (surface::ACTIVE.into(), palette::GRAY_3.lighter(0.15)),
            (surface::INVERSE.into(), palette::LIGHT_GRAY_1),
            // SURFACE.ACCENT
            (
                surface::accent::CONTRAST.into(),
                palette::ACCENT.darker(0.2),
            ),
            (
                surface::accent::DISABLED.into(),
                palette::ACCENT.with_alpha(0.5),
            ),
            (
                surface::accent::MUTED.into(),
                palette::ACCENT.with_alpha(0.6),
            ),
            (surface::accent::BASE.into(), palette::ACCENT),
            (
                surface::accent::ELEVATED.into(),
                palette::ACCENT.lighter(0.05),
            ),
            (
                surface::accent::FOCUSED.into(),
                palette::ACCENT.lighter(0.1),
            ),
            (
                surface::accent::ACTIVE.into(),
                palette::ACCENT.lighter(0.15),
            ),
            // SURFACE.ERROR
            (surface::error::CONTRAST.into(), palette::ERROR.darker(0.2)),
            (
                surface::error::DISABLED.into(),
                palette::ERROR.with_alpha(0.5),
            ),
            (surface::error::MUTED.into(), palette::ERROR.with_alpha(0.6)),
            (surface::error::BASE.into(), palette::ERROR),
            (
                surface::error::ELEVATED.into(),
                palette::ERROR.lighter(0.05),
            ),
            (surface::error::FOCUSED.into(), palette::ERROR.lighter(0.1)),
            (surface::error::ACTIVE.into(), palette::ERROR.lighter(0.15)),
            // SURFACE.WARNING
            (
                surface::warning::CONTRAST.into(),
                palette::WARNING.darker(0.2),
            ),
            (
                surface::warning::DISABLED.into(),
                palette::WARNING.with_alpha(0.5),
            ),
            (
                surface::warning::MUTED.into(),
                palette::WARNING.with_alpha(0.6),
            ),
            (surface::warning::BASE.into(), palette::WARNING),
            (
                surface::warning::ELEVATED.into(),
                palette::WARNING.lighter(0.05),
            ),
            (
                surface::warning::FOCUSED.into(),
                palette::WARNING.lighter(0.1),
            ),
            (
                surface::warning::ACTIVE.into(),
                palette::WARNING.lighter(0.15),
            ),
            // SURFACE.SUCCESS
            (
                surface::success::CONTRAST.into(),
                palette::SUCCESS.darker(0.2),
            ),
            (
                surface::success::DISABLED.into(),
                palette::SUCCESS.with_alpha(0.5),
            ),
            (
                surface::success::MUTED.into(),
                palette::SUCCESS.with_alpha(0.6),
            ),
            (surface::success::BASE.into(), palette::SUCCESS),
            (
                surface::success::ELEVATED.into(),
                palette::SUCCESS.lighter(0.05),
            ),
            (
                surface::success::FOCUSED.into(),
                palette::SUCCESS.lighter(0.1),
            ),
            (
                surface::success::ACTIVE.into(),
                palette::SUCCESS.lighter(0.15),
            ),
            // SURFACE.INFO
            (surface::info::CONTRAST.into(), palette::INFO.darker(0.2)),
            (
                surface::info::DISABLED.into(),
                palette::INFO.with_alpha(0.5),
            ),
            (surface::info::MUTED.into(), palette::INFO.with_alpha(0.6)),
            (surface::info::BASE.into(), palette::INFO),
            (surface::info::ELEVATED.into(), palette::INFO.lighter(0.05)),
            (surface::info::FOCUSED.into(), palette::INFO.lighter(0.1)),
            (surface::info::ACTIVE.into(), palette::INFO.lighter(0.15)),
            // ------------------------------------
            // BORDER
            (border::CONTRAST.into(), palette::WARM_GRAY_1.darker(0.12)),
            (
                border::DISABLED.into(),
                palette::WARM_GRAY_1.with_alpha(0.5),
            ),
            (border::MUTED.into(), palette::WARM_GRAY_1.with_alpha(0.6)),
            (border::BASE.into(), palette::WARM_GRAY_1),
            (border::ELEVATED.into(), palette::WARM_GRAY_1.lighter(0.05)),
            (border::FOCUSED.into(), palette::WARM_GRAY_1.lighter(0.1)),
            (border::ACTIVE.into(), palette::WARM_GRAY_1.lighter(0.15)),
            // BORDER.ACCENT
            (border::accent::CONTRAST.into(), palette::ACCENT.darker(0.2)),
            (
                border::accent::DISABLED.into(),
                palette::ACCENT.with_alpha(0.5),
            ),
            (
                border::accent::MUTED.into(),
                palette::ACCENT.with_alpha(0.6),
            ),
            (border::accent::BASE.into(), palette::ACCENT),
            (
                border::accent::ELEVATED.into(),
                palette::ACCENT.lighter(0.05),
            ),
            (border::accent::FOCUSED.into(), palette::ACCENT.lighter(0.1)),
            (border::accent::ACTIVE.into(), palette::ACCENT.lighter(0.15)),
            // BORDER.ERROR
            (border::error::CONTRAST.into(), palette::ERROR.darker(0.2)),
            (
                border::error::DISABLED.into(),
                palette::ERROR.with_alpha(0.5),
            ),
            (border::error::MUTED.into(), palette::ERROR.with_alpha(0.6)),
            (border::error::BASE.into(), palette::ERROR),
            (border::error::ELEVATED.into(), palette::ERROR.lighter(0.05)),
            (border::error::FOCUSED.into(), palette::ERROR.lighter(0.1)),
            (border::error::ACTIVE.into(), palette::ERROR.lighter(0.15)),
            // BORDER.WARNING
            (
                border::warning::CONTRAST.into(),
                palette::WARNING.darker(0.2),
            ),
            (
                border::warning::DISABLED.into(),
                palette::WARNING.with_alpha(0.5),
            ),
            (
                border::warning::MUTED.into(),
                palette::WARNING.with_alpha(0.6),
            ),
            (border::warning::BASE.into(), palette::WARNING),
            (
                border::warning::ELEVATED.into(),
                palette::WARNING.lighter(0.05),
            ),
            (
                border::warning::FOCUSED.into(),
                palette::WARNING.lighter(0.1),
            ),
            (
                border::warning::ACTIVE.into(),
                palette::WARNING.lighter(0.15),
            ),
            // BORDER.SUCCESS
            (
                border::success::CONTRAST.into(),
                palette::SUCCESS.darker(0.2),
            ),
            (
                border::success::DISABLED.into(),
                palette::SUCCESS.with_alpha(0.5),
            ),
            (
                border::success::MUTED.into(),
                palette::SUCCESS.with_alpha(0.6),
            ),
            (border::success::BASE.into(), palette::SUCCESS),
            (
                border::success::ELEVATED.into(),
                palette::SUCCESS.lighter(0.05),
            ),
            (
                border::success::FOCUSED.into(),
                palette::SUCCESS.lighter(0.1),
            ),
            (
                border::success::ACTIVE.into(),
                palette::SUCCESS.lighter(0.15),
            ),
            // BORDER.INFO
            (border::info::CONTRAST.into(), palette::INFO.darker(0.2)),
            (border::info::DISABLED.into(), palette::INFO.with_alpha(0.5)),
            (border::info::MUTED.into(), palette::INFO.with_alpha(0.6)),
            (border::info::BASE.into(), palette::INFO),
            (border::info::ELEVATED.into(), palette::INFO.lighter(0.05)),
            (border::info::FOCUSED.into(), palette::INFO.lighter(0.1)),
            (border::info::ACTIVE.into(), palette::INFO.lighter(0.15)),
            // ------------------------------------
            // FOREGROUND
            (foreground::CONTRAST.into(), palette::WHITE),
            (foreground::DISABLED.into(), palette::WHITE.with_alpha(0.5)),
            (foreground::MUTED.into(), palette::WHITE.with_alpha(0.6)),
            (foreground::BASE.into(), palette::WHITE),
            (foreground::ELEVATED.into(), palette::WHITE),
            (foreground::FOCUSED.into(), palette::WHITE),
            (foreground::ACTIVE.into(), palette::WHITE),
            // FOREGROUND.ACCENT
            (
                foreground::accent::CONTRAST.into(),
                palette::ACCENT.lighter(0.4),
            ),
            (
                foreground::accent::DISABLED.into(),
                palette::ACCENT.with_alpha(0.5),
            ),
            (
                foreground::accent::MUTED.into(),
                palette::ACCENT.with_alpha(0.6),
            ),
            (foreground::accent::BASE.into(), palette::ACCENT),
            (
                foreground::accent::ELEVATED.into(),
                palette::ACCENT.lighter(0.05),
            ),
            (
                foreground::accent::FOCUSED.into(),
                palette::ACCENT.lighter(0.1),
            ),
            (
                foreground::accent::ACTIVE.into(),
                palette::ACCENT.lighter(0.15),
            ),
            // FOREGROUND.ERROR
            (
                foreground::error::CONTRAST.into(),
                palette::ERROR.darker(0.2),
            ),
            (
                foreground::error::DISABLED.into(),
                palette::ERROR.with_alpha(0.5),
            ),
            (
                foreground::error::MUTED.into(),
                palette::ERROR.with_alpha(0.6),
            ),
            (foreground::error::BASE.into(), palette::ERROR),
            (
                foreground::error::ELEVATED.into(),
                palette::ERROR.lighter(0.05),
            ),
            (
                foreground::error::FOCUSED.into(),
                palette::ERROR.lighter(0.1),
            ),
            (
                foreground::error::ACTIVE.into(),
                palette::ERROR.lighter(0.15),
            ),
            // FOREGROUND.WARNING
            (
                foreground::warning::CONTRAST.into(),
                palette::WARNING.darker(0.2),
            ),
            (
                foreground::warning::DISABLED.into(),
                palette::WARNING.with_alpha(0.5),
            ),
            (
                foreground::warning::MUTED.into(),
                palette::WARNING.with_alpha(0.6),
            ),
            (foreground::warning::BASE.into(), palette::WARNING),
            (
                foreground::warning::ELEVATED.into(),
                palette::WARNING.lighter(0.05),
            ),
            (
                foreground::warning::FOCUSED.into(),
                palette::WARNING.lighter(0.1),
            ),
            (
                foreground::warning::ACTIVE.into(),
                palette::WARNING.lighter(0.15),
            ),
            // FOREGROUND.SUCCESS
            (
                foreground::success::CONTRAST.into(),
                palette::SUCCESS.darker(0.2),
            ),
            (
                foreground::success::DISABLED.into(),
                palette::SUCCESS.with_alpha(0.5),
            ),
            (
                foreground::success::MUTED.into(),
                palette::SUCCESS.with_alpha(0.6),
            ),
            (foreground::success::BASE.into(), palette::SUCCESS),
            (
                foreground::success::ELEVATED.into(),
                palette::SUCCESS.lighter(0.05),
            ),
            (
                foreground::success::FOCUSED.into(),
                palette::SUCCESS.lighter(0.1),
            ),
            (
                foreground::success::ACTIVE.into(),
                palette::SUCCESS.lighter(0.15),
            ),
            // FOREGROUND.INFO
            (foreground::info::CONTRAST.into(), palette::INFO.darker(0.2)),
            (
                foreground::info::DISABLED.into(),
                palette::INFO.with_alpha(0.5),
            ),
            (
                foreground::info::MUTED.into(),
                palette::INFO.with_alpha(0.6),
            ),
            (foreground::info::BASE.into(), palette::INFO),
            (
                foreground::info::ELEVATED.into(),
                palette::INFO.lighter(0.05),
            ),
            (foreground::info::FOCUSED.into(), palette::INFO.lighter(0.1)),
            (foreground::info::ACTIVE.into(), palette::INFO.lighter(0.15)),
        ]),
    }
}
