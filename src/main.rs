use bevy::prelude::*;

use bevy_inspector_egui::{
    bevy_egui, prelude::*, quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin,
};

#[derive(Component, Reflect, Default)]
struct Deck;

#[derive(Component, Reflect, Default)]
struct City;

#[derive(Component, Reflect, Default)]
struct Edge(City, City);

#[derive(Component, Reflect, Default)]
struct ResearchStation;

#[derive(Component, Reflect, Default)]
struct Outbreak;

#[derive(Component, Reflect)]
struct InfectionCount(Color, u8);

#[derive(Component, Reflect)]
enum Color {
    Blue,
    Yellow,
    Black,
    Red,
}

fn add_cities(mut commands: Commands) {
    let mut cities = Vec::new();
    cities.push(
        commands
            .spawn((City, Name::new("Atlanta"), ResearchStation, Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Chicago"), Color::Blue))
            .id(),
    );
    cities.push(commands.spawn((City, Name::new("Essen"), Color::Blue)).id());
    cities.push(
        commands
            .spawn((City, Name::new("London"), Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Madrid"), Color::Blue))
            .id(),
    );
    cities.push(commands.spawn((City, Name::new("Milan"), Color::Blue)).id());
    cities.push(
        commands
            .spawn((City, Name::new("Montréal"), Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("New York"), Color::Blue))
            .id(),
    );
    cities.push(commands.spawn((City, Name::new("Paris"), Color::Blue)).id());
    cities.push(
        commands
            .spawn((City, Name::new("St. Petersburg"), Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("San Francisco"), Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Washington"), Color::Blue))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Bogotá"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Buenos Aires"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Johannesburg"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Khartoum"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Kinshasa"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Lagos"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Lima"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Los Angeles"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Mexico City"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Miami"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Santiago"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("São Paulo"), Color::Yellow))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Algiers"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Baghdad"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Cairo"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Chennai"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Delhi"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Istanbul"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Karachi"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Kolkata"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Moscow"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Mumbai"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Riyadh"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Tehran"), Color::Black))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Bangkok"), Color::Red))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Beijing"), Color::Red))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Ho Chi Minh City"), Color::Red))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Hong Kong"), Color::Red))
            .id(),
    );
    cities.push(
        commands
            .spawn((City, Name::new("Jakarta"), Color::Red))
            .id(),
    );
    cities.push(commands.spawn((City, Name::new("Manila"), Color::Red)).id());
    cities.push(commands.spawn((City, Name::new("Osaka"), Color::Red)).id());
    cities.push(commands.spawn((City, Name::new("Seoul"), Color::Red)).id());
    cities.push(
        commands
            .spawn((City, Name::new("Shanghai"), Color::Red))
            .id(),
    );
    cities.push(commands.spawn((City, Name::new("Sydney"), Color::Red)).id());
    cities.push(commands.spawn((City, Name::new("Taipei"), Color::Red)).id());
    cities.push(commands.spawn((City, Name::new("Tokyo"), Color::Red)).id());
    commands.spawn((Name::new("Cities"))).push_children(&cities);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .register_type::<City>()
        .register_type::<Deck>()
        .register_type::<Edge>()
        .register_type::<ResearchStation>()
        .register_type::<Outbreak>()
        .register_type::<Name>()
        .register_type::<Color>()
        .register_type::<InfectionCount>()
        .add_startup_system(add_cities)
        .run();
}
