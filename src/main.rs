use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_left: i32,
}

fn main() {
    let mut game = Game::new();
    let line1 = game.add_sprite("line1", "sprite/separator.png");
    line1.translation.x = -66.66;
    let line2 = game.add_sprite("line2", "sprite/separator.png");
    line2.translation.x = 66.66;
    let line3 = game.add_sprite("line3", "sprite/separator.png");
    line3.translation.y = -66.66;
    line3.rotation = UP;
    let line4 = game.add_sprite("line4", "sprite/separator.png");
    line4.translation.y = 66.66;
    line4.rotation = UP;
    game.run(GameState { health_left: 42 });
}
