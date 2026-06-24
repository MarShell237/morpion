use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    grid: [[(u8, Vec2); 3]; 3],
    is_player_to_play: bool,
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
    let grid: [[(u8, Vec2); 3]; 3] = [
        [
            (0, Vec2::new(-133.33, 133.33)),
            (0, Vec2::new(0.0, 133.33)),
            (0, Vec2::new(133.33, 133.33)),
        ],
        [
            (0, Vec2::new(-133.33, 0.0)),
            (0, Vec2::new(0.0, 0.0)),
            (0, Vec2::new(133.33, 0.0)),
        ],
        [
            (0, Vec2::new(-133.33, -133.33)),
            (0, Vec2::new(0.0, -133.33)),
            (0, Vec2::new(133.33, -133.33)),
        ],
    ];
    game.add_logic(update_grid);
    game.add_logic(draw_content_cell);
    game.run(GameState {
        grid,
        is_player_to_play: true,
    });
}

fn update_grid(engine: &mut Engine, game_state: &mut GameState) {
    if engine.mouse_state.pressed(MouseButton::Left) {
        let mut i: usize = 3;
        let mut j: usize = 3;
        let mut cell_content_number: u8 = 0;
        if let Some(location) = engine.mouse_state.location() {
            i = get_index_cell(-location.x as f32) as usize;
            j = get_index_cell(location.y as f32) as usize;
        }
        if game_state.is_player_to_play {
            cell_content_number = 1;
        } else {
            cell_content_number = 2;
        }
        game_state.grid[j][i].0 = cell_content_number;
    }
}

fn get_index_cell(coord: f32) -> u8 {
    if coord < 200.0 && coord > 66.66 {
        return 0;
    }

    if coord < 66.66 && coord > -66.66 {
        return 1;
    }

    if coord < -66.66 && coord > -200.0 {
        return 2;
    }

    3
}

fn draw_content_cell(engine: &mut Engine, game_state: &mut GameState) {
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
