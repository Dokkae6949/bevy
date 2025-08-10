pub mod family {
    /// Default regular font path
    pub const REGULAR: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Regular.ttf";
    /// Regular italic font path
    pub const ITALIC: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Italic.ttf";
    /// Bold font path
    pub const BOLD: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-Bold.ttf";
    /// Bold italic font path
    pub const BOLD_ITALIC: &str = "embedded://bevy_feathers/assets/fonts/FiraSans-BoldItalic.ttf";
    /// Monospace font path
    pub const MONO: &str = "embedded://bevy_feathers/assets/fonts/FiraMono-Medium.ttf";
}

pub mod size {
    pub mod heading {
        pub const SM: f32 = 20.;
        pub const MD: f32 = 24.;
        pub const LG: f32 = 32.;
    }

    pub mod body {
        pub const SM: f32 = 12.;
        pub const MD: f32 = 14.;
        pub const LG: f32 = 16.;
    }

    pub mod caption {
        pub const SM: f32 = 6.;
        pub const MD: f32 = 8.;
        pub const LG: f32 = 10.;
    }
}
