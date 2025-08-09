#![allow(missing_docs)]

pub mod axis {
    pub const X: &str = "feathers.color.axis.x";
    pub const Y: &str = "feathers.color.axis.y";
    pub const Z: &str = "feathers.color.axis.z";
}

pub mod container {
    /// Window background
    pub const WINDOW: &str = "feathers.color.container.window";
    /// Pane header background
    pub const PRIMARY: &str = "feathers.color.container.primary";
    /// Pane body background
    pub const SECONDARY: &str = "feathers.color.container.secondary";
}

pub mod surface {
    pub const CONTRAST: &str = "feathers.color.surface.contrast";
    pub const DISABLED: &str = "feathers.color.surface.disabled";
    pub const MUTED: &str = "feathers.color.surface.muted";
    pub const BASE: &str = "feathers.color.surface.base";
    pub const ELEVATED: &str = "feathers.color.surface.elevated";
    pub const FOCUSED: &str = "feathers.color.surface.focused";
    pub const ACTIVE: &str = "feathers.color.surface.active";
    pub const INVERSE: &str = "feathers.color.surface.inverse";

    pub mod accent {
        pub const CONTRAST: &str = "feathers.color.surface.accent.contrast";
        pub const DISABLED: &str = "feathers.color.surface.accent.disabled";
        pub const MUTED: &str = "feathers.color.surface.accent.muted";
        pub const BASE: &str = "feathers.color.surface.accent.base";
        pub const ELEVATED: &str = "feathers.color.surface.accent.elevated";
        pub const FOCUSED: &str = "feathers.color.surface.accent.focused";
        pub const ACTIVE: &str = "feathers.color.surface.accent.active";
    }

    pub mod error {
        pub const CONTRAST: &str = "feathers.color.surface.error.contrast";
        pub const DISABLED: &str = "feathers.color.surface.error.disabled";
        pub const MUTED: &str = "feathers.color.surface.error.muted";
        pub const BASE: &str = "feathers.color.surface.error.base";
        pub const ELEVATED: &str = "feathers.color.surface.error.elevated";
        pub const FOCUSED: &str = "feathers.color.surface.error.focused";
        pub const ACTIVE: &str = "feathers.color.surface.error.active";
    }

    pub mod warning {
        pub const CONTRAST: &str = "feathers.color.surface.warning.contrast";
        pub const DISABLED: &str = "feathers.color.surface.warning.disabled";
        pub const MUTED: &str = "feathers.color.surface.warning.muted";
        pub const BASE: &str = "feathers.color.surface.warning.base";
        pub const ELEVATED: &str = "feathers.color.surface.warning.elevated";
        pub const FOCUSED: &str = "feathers.color.surface.warning.focused";
        pub const ACTIVE: &str = "feathers.color.surface.warning.active";
    }

    pub mod success {
        pub const CONTRAST: &str = "feathers.color.surface.success.contrast";
        pub const DISABLED: &str = "feathers.color.surface.success.disabled";
        pub const MUTED: &str = "feathers.color.surface.success.muted";
        pub const BASE: &str = "feathers.color.surface.success.base";
        pub const ELEVATED: &str = "feathers.color.surface.success.elevated";
        pub const FOCUSED: &str = "feathers.color.surface.success.focused";
        pub const ACTIVE: &str = "feathers.color.surface.success.active";
    }

    pub mod info {
        pub const CONTRAST: &str = "feathers.color.surface.info.contrast";
        pub const DISABLED: &str = "feathers.color.surface.info.disabled";
        pub const MUTED: &str = "feathers.color.surface.info.muted";
        pub const BASE: &str = "feathers.color.surface.info.base";
        pub const ELEVATED: &str = "feathers.color.surface.info.elevated";
        pub const FOCUSED: &str = "feathers.color.surface.info.focused";
        pub const ACTIVE: &str = "feathers.color.surface.info.active";
    }
}

pub mod border {
    pub const CONTRAST: &str = "feathers.color.border.contrast";
    pub const DISABLED: &str = "feathers.color.border.disabled";
    pub const MUTED: &str = "feathers.color.border.muted";
    pub const BASE: &str = "feathers.color.border.base";
    pub const ELEVATED: &str = "feathers.color.border.elevated";
    pub const FOCUSED: &str = "feathers.color.border.focused";
    pub const ACTIVE: &str = "feathers.color.border.active";

    pub mod accent {
        pub const CONTRAST: &str = "feathers.color.border.accent.contrast";
        pub const DISABLED: &str = "feathers.color.border.accent.disabled";
        pub const MUTED: &str = "feathers.color.border.accent.muted";
        pub const BASE: &str = "feathers.color.border.accent.base";
        pub const ELEVATED: &str = "feathers.color.border.accent.elevated";
        pub const FOCUSED: &str = "feathers.color.border.accent.focused";
        pub const ACTIVE: &str = "feathers.color.border.accent.active";
    }

    pub mod error {
        pub const CONTRAST: &str = "feathers.color.border.status.error.contrast";
        pub const DISABLED: &str = "feathers.color.border.status.error.disabled";
        pub const MUTED: &str = "feathers.color.border.status.error.muted";
        pub const BASE: &str = "feathers.color.border.status.error.base";
        pub const ELEVATED: &str = "feathers.color.border.status.error.elevated";
        pub const FOCUSED: &str = "feathers.color.border.status.error.focused";
        pub const ACTIVE: &str = "feathers.color.border.status.error.active";
    }

    pub mod warning {
        pub const CONTRAST: &str = "feathers.color.border.status.warning.contrast";
        pub const DISABLED: &str = "feathers.color.border.status.warning.disabled";
        pub const MUTED: &str = "feathers.color.border.status.warning.muted";
        pub const BASE: &str = "feathers.color.border.status.warning.base";
        pub const ELEVATED: &str = "feathers.color.border.status.warning.elevated";
        pub const FOCUSED: &str = "feathers.color.border.status.warning.focused";
        pub const ACTIVE: &str = "feathers.color.border.status.warning.active";
    }

    pub mod success {
        pub const CONTRAST: &str = "feathers.color.border.status.success.contrast";
        pub const DISABLED: &str = "feathers.color.border.status.success.disabled";
        pub const MUTED: &str = "feathers.color.border.status.success.muted";
        pub const BASE: &str = "feathers.color.border.status.success.base";
        pub const ELEVATED: &str = "feathers.color.border.status.success.elevated";
        pub const FOCUSED: &str = "feathers.color.border.status.success.focused";
        pub const ACTIVE: &str = "feathers.color.border.status.success.active";
    }

    pub mod info {
        pub const CONTRAST: &str = "feathers.color.border.status.info.contrast";
        pub const DISABLED: &str = "feathers.color.border.status.info.disabled";
        pub const MUTED: &str = "feathers.color.border.status.info.muted";
        pub const BASE: &str = "feathers.color.border.status.info.base";
        pub const ELEVATED: &str = "feathers.color.border.status.info.elevated";
        pub const FOCUSED: &str = "feathers.color.border.status.info.focused";
        pub const ACTIVE: &str = "feathers.color.border.status.info.active";
    }
}

pub mod foreground {
    pub const CONTRAST: &str = "feathers.color.text.contrast";
    pub const DISABLED: &str = "feathers.color.text.disabled";
    pub const MUTED: &str = "feathers.color.control.muted";
    pub const BASE: &str = "feathers.color.text.base";
    pub const ELEVATED: &str = "feathers.color.text.elevated";
    pub const FOCUSED: &str = "feathers.color.text.focused";
    pub const ACTIVE: &str = "feathers.color.text.active";

    pub mod accent {
        pub const CONTRAST: &str = "feathers.color.text.accent.contrast";
        pub const DISABLED: &str = "feathers.color.text.accent.disabled";
        pub const MUTED: &str = "feathers.color.text.accent.muted";
        pub const BASE: &str = "feathers.color.text.accent.base";
        pub const ELEVATED: &str = "feathers.color.text.accent.elevated";
        pub const FOCUSED: &str = "feathers.color.text.accent.focused";
        pub const ACTIVE: &str = "feathers.color.text.accent.active";
    }

    pub mod error {
        pub const CONTRAST: &str = "feathers.color.text.status.error.contrast";
        pub const DISABLED: &str = "feathers.color.text.status.error.disabled";
        pub const MUTED: &str = "feathers.color.text.status.error.muted";
        pub const BASE: &str = "feathers.color.text.status.error.base";
        pub const ELEVATED: &str = "feathers.color.text.status.error.elevated";
        pub const FOCUSED: &str = "feathers.color.text.status.error.focused";
        pub const ACTIVE: &str = "feathers.color.text.status.error.active";
    }

    pub mod warning {
        pub const CONTRAST: &str = "feathers.color.text.status.warning.contrast";
        pub const DISABLED: &str = "feathers.color.text.status.warning.disabled";
        pub const MUTED: &str = "feathers.color.text.status.warning.muted";
        pub const BASE: &str = "feathers.color.text.status.warning.base";
        pub const ELEVATED: &str = "feathers.color.text.status.warning.elevated";
        pub const FOCUSED: &str = "feathers.color.text.status.warning.focused";
        pub const ACTIVE: &str = "feathers.color.text.status.warning.active";
    }

    pub mod success {
        pub const CONTRAST: &str = "feathers.color.text.status.success.contrast";
        pub const DISABLED: &str = "feathers.color.text.status.success.disabled";
        pub const MUTED: &str = "feathers.color.text.status.success.muted";
        pub const BASE: &str = "feathers.color.text.status.success.base";
        pub const ELEVATED: &str = "feathers.color.text.status.success.elevated";
        pub const FOCUSED: &str = "feathers.color.text.status.success.focused";
        pub const ACTIVE: &str = "feathers.color.text.status.success.active";
    }

    pub mod info {
        pub const CONTRAST: &str = "feathers.color.text.status.info.contrast";
        pub const DISABLED: &str = "feathers.color.text.status.info.disabled";
        pub const MUTED: &str = "feathers.color.text.status.info.muted";
        pub const BASE: &str = "feathers.color.text.status.info.base";
        pub const ELEVATED: &str = "feathers.color.text.status.info.elevated";
        pub const FOCUSED: &str = "feathers.color.text.status.info.focused";
        pub const ACTIVE: &str = "feathers.color.text.status.info.active";
    }
}
