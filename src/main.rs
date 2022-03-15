use rusty_engine::prelude::*;

struct GameState {}

fn main() {
    let mut game = Game::new();
    //game setup
    let window_descriptor = WindowDescriptor {
        title: "Road Racer".into(),
        width: 600.0,
        height: 1000.0,
        ..Default::default()
    };
    game.window_settings(window_descriptor);
    game.add_logic(game_logic);

    game.run(GameState {});

}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic
}