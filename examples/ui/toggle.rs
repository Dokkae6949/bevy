//! This example illustrates how to create a toggle button that changes text based on its
//! toggle state.

use bevy::{color::palettes::basic::*, prelude::*, winit::WinitSettings};

#[derive(Debug, Clone)]
enum ToggleState {
    On,
    Off,
}

impl ToggleState {
    pub fn toggle(&mut self) {
        *self = match self {
            ToggleState::On => ToggleState::Off,
            ToggleState::Off => ToggleState::On,
        };
    }
}

#[derive(Debug, Clone, Component)]
struct ToggleButton {
    pub state: ToggleState,
    pub text_on: String,
    pub text_off: String,
}

impl ToggleButton {
    pub fn text(&self) -> String {
        match self.state {
            ToggleState::On => self.text_on.clone(),
            ToggleState::Off => self.text_off.clone(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut ToggleButton,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut button, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                button.state.toggle();
                text.sections[0].value = button.text();
            }
            _ => {}
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ToggleButton {
                    state: ToggleState::Off,
                    text_on: "On".to_string(),
                    text_off: "Off".to_string(),
                })
                .with_child(TextBundle::from_section(
                    "Off",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    }));
        });
}
