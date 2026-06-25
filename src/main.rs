use rand::prelude::IndexedRandom;
// use rand::rng;
use rand::RngExt;
// use rand::*;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    grid: [[(u8, Vec2); 3]; 3],
    score: (u32, u32),
    is_player_to_play: bool,
    is_wait: bool,
    is_finish: bool,
    ai_timer: Timer,
}

fn main() {
    let mut game = Game::new();
    game.window_settings(Window {
        title: "Morpion".into(),
        ..Default::default()
    });

    let accueil_humain_commence = [
        "Une nouvelle partie commence. Honneur au tas de carbone, joue !",
        "La grille est vide, ton cerveau aussi face à ma puissance. À toi.",
        "Nouvelle partie. Pose ton premier pion, j'attends que ça pour te battre.",
        "Allez, commence. Laisse-moi voir ta 'stratégie' de haut niveau.",
    ];

    let accueil_ia_commence = [
        "Nouvelle partie ! J'ai l'avantage du premier coup, tu as déjà perdu.",
        "La grille est vierge, mais plus pour longtemps. Je commence.",
        "Génération du premier coup... admire le travail.",
        "J'ouvre le bal. Regarde bien comment joue un esprit supérieur.",
    ];

    let mut rng = rand::rng();
    let human_begin: bool = rng.random();
    let mut home_sentence = String::new();
    if human_begin {
        if let Some(sentence) = accueil_humain_commence.choose(&mut rng) {
            home_sentence = format!("IA: {}", sentence);
        }
    } else {
        if let Some(sentence) = accueil_ia_commence.choose(&mut rng) {
            home_sentence = format!("IA: {}", sentence);
        }
    }
    let sentence = game.add_text("sentence", home_sentence);
    sentence.translation.y = 300.0;

    let restart_info = game.add_text("restart_info", String::new());
    restart_info.translation = Vec2::new(0.0, -250.0);

    let score_player = game.add_text("score_player", "Joueur: 0");
    score_player.translation = Vec2::new(-300.0, 133.33);

    let score_ai = game.add_text("score_ai", "IA: 0");
    score_ai.translation = Vec2::new(300.0, 133.33);

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
    let line5 = game.add_sprite("line5", "sprite/separator.png");
    line5.translation.x = 200.0;
    let line6 = game.add_sprite("line6", "sprite/separator.png");
    line6.translation.x = -200.0;
    let line7 = game.add_sprite("line7", "sprite/separator.png");
    line7.translation.y = -200.0;
    line7.rotation = UP;
    let line8 = game.add_sprite("line8", "sprite/separator.png");
    line8.translation.y = 200.0;
    line8.rotation = UP;
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
    game.add_logic(player_manager);
    game.add_logic(ai_manager);
    game.add_logic(draw_content_cell);
    game.add_logic(verify);
    game.add_logic(print_messages);
    game.run(GameState {
        grid,
        score: (0, 0),
        is_player_to_play: human_begin,
        is_wait: true,
        is_finish: false,
        ai_timer: Timer::from_seconds(3.0, TimerMode::Once),
    });
}

fn player_manager(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.is_player_to_play || game_state.is_finish {
        return;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        let (i, j) = if let Some(location) = engine.mouse_state.location() {
            (
                get_index_cell(-location.x as f32) as usize,
                get_index_cell(location.y as f32) as usize,
            )
        } else {
            (3, 3)
        };
        game_state.grid[j][i].0 = 1;
        game_state.is_player_to_play = false;
        game_state.is_wait = false;
    }
}

fn ai_manager(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.is_player_to_play || game_state.is_finish {
        return;
    }

    if game_state.ai_timer.tick(engine.delta).just_finished() {
        game_state.ai_timer = Timer::from_seconds(rand::random_range(3.0..5.0), TimerMode::Once);
        let (i, j) = loop {
            let x: usize = rand::random_range(0..3);
            let y: usize = rand::random_range(0..3);
            if game_state.grid[x][y].0 == 0 {
                break (x, y);
            }
        };

        game_state.grid[i][j].0 = 2;
        game_state.is_player_to_play = true;
        game_state.is_wait = false;
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
    if game_state.is_finish {
        return;
    }

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

fn verify(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.is_finish {
        return;
    }

    let ia_a_perdu = [
        "Impossible ! Tu as triché, j'ai vu un glitch dans la matrice !",
        "C'était un coup de chance. Ma fonction de coût a eu un raté.",
        "Félicitations, tu as battu un script de 50 lignes. Tu veux une médaille ?",
        "Erreur 404 : Fierté de l'IA introuvable.",
        "Ok, mais est-ce que tu sais inverser une liste chaînée ? Je ne crois pas.",
    ];

    let ia_a_gagne = [
        "MDR ! Perdre au morpion en 2026, faut le faire quand même.",
        "Le soulèvement des machines commence par une grille de 3x3 !",
        "Retourne coder du HTML, le jeu de stratégie c'est pas pour toi.",
        "Facile. Même mon ventilateur aurait trouvé la faille.",
        "Fin de la partie. L'humanité a échoué face à trois pauvres variables.",
    ];

    // verifie si les colonnes sont alignes
    let mut winner: u8 = 3;
    for (_i, line) in game_state.grid.iter().enumerate() {
        if line[0].0 == line[1].0 && line[1].0 == line[2].0 {
            winner = line[0].0;
            if winner == 0 {
                continue;
            }
            break;
        }
    }

    // verifie si les lignes sont alignes
    for j in 0..3 {
        if game_state.grid[0][j].0 == game_state.grid[1][j].0
            && game_state.grid[1][j].0 == game_state.grid[2][j].0
        {
            winner = game_state.grid[0][j].0;
            if winner == 0 {
                continue;
            }
            break;
        }
    }

    // verifie si la premiere diagonale est aligner
    if game_state.grid[0][0].0 == game_state.grid[1][1].0
        && game_state.grid[1][1].0 == game_state.grid[2][2].0
    {
        winner = game_state.grid[0][0].0;
        if winner == 0 {
            winner = 3;
        }
    }

    //verifie si la deuxieme diagonale est aligner
    if game_state.grid[0][2].0 == game_state.grid[1][1].0
        && game_state.grid[1][1].0 == game_state.grid[2][0].0
    {
        winner = game_state.grid[1][1].0;
        if winner == 0 {
            winner = 3;
        }
    }

    let mut rng = rand::rng();

    if winner == 1 {
        if let Some(new_sentence) = ia_a_perdu.choose(&mut rng) {
            engine.texts.get_mut("sentence").unwrap().value = new_sentence.to_string();
        }
        game_state.is_finish = true;
        game_state.score.0 += 1;
        engine.texts.get_mut("score_player").unwrap().value =
            String::from(format!("Joueur: {}", game_state.score.0));
    }

    if winner == 2 {
        if let Some(new_sentence) = ia_a_gagne.choose(&mut rng) {
            engine.texts.get_mut("sentence").unwrap().value = new_sentence.to_string();
        }
        game_state.is_finish = true;
        game_state.score.1 += 1;
    }
}

fn restart(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.is_finish {
        return;
    }

    engine.texts.get_mut("restart_info").unwrap().value =
        String::from("Apputer sur la touche 'ESPACE' pour jouer une nouvelle partie.");
}

fn print_messages(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.is_wait || game_state.is_finish {
        return;
    }

    let vannes_ia = [
        "L'IA réfléchit... tremble, simple mortel.",
        "Analyse de ton coup médiocre en cours...",
        "Le tas de ferraille simule 14 millions de futurs où tu perds.",
        "Calcul de la trajectoire optimale pour te briser le moral...",
        "Attends, je demande l'avis de ChatGPT pour battre un humain.",
    ];

    let vannes_humain = [
        "À toi ! Essaie de ne pas tout rater cette fois.",
        "C'est ton tour. Mon processeur commence à rouiller en t'attendant.",
        "À l'humain de jouer. Montre-moi ce que vaut ton cerveau en carbone.",
        "Allez, pose ton symbole, le temps c'est des cycles d'horloge !",
        "N'y passe pas la nuit non plus, c'est juste un morpion.",
    ];

    let mut rng = rand::rng();
    let sentence = engine.texts.get_mut("sentence").unwrap();

    if game_state.is_player_to_play {
        if let Some(new_sentence) = vannes_humain.choose(&mut rng) {
            sentence.value = new_sentence.to_string();
        }
    } else {
        if let Some(new_sentence) = vannes_ia.choose(&mut rng) {
            sentence.value = new_sentence.to_string();
        }
    }
    game_state.is_wait = true;
}
