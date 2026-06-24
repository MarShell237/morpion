use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    grid: [[(u8, Vec2); 3]; 3],
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
    // let o1 = game.add_text("o1", "o");
    // o1.translation.x = -133.33;
    // o1.translation.y = 133.33;
    // o1.font_size = 100.0;
    // let x1 = game.add_text("x1", "x");
    // x1.font_size = 100.0;
    let grid: [[(u8, Vec2); 3]; 3] = [
        [
            (1, Vec2::new(-133.33, 133.33)),
            (0, Vec2::new(0.0, 133.33)),
            (1, Vec2::new(133.33, 133.33)),
        ],
        [
            (0, Vec2::new(-133.33, 0.0)),
            (2, Vec2::new(0.0, 0.0)),
            (0, Vec2::new(133.33, 0.0)),
        ],
        [
            (2, Vec2::new(-133.33, -133.33)),
            (0, Vec2::new(0.0, -133.33)),
            (0, Vec2::new(133.33, -133.33)),
        ],
    ];
    game.add_logic(game_logic);
    game.run(GameState { grid });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    for (i, line) in game_state.grid.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            let mut cell_content_string = String::new();
            if cell.0 == 1 {
                cell_content_string = "o".to_string();
            }

            if cell.0 == 2 {
                cell_content_string = "x".to_string();
            }

            let cell_content = engine.add_text(format!("cell{}{}", i, j), cell_content_string);
            cell_content.translation = cell.1;
            cell_content.font_size = 100.0;
        }
    }
}
