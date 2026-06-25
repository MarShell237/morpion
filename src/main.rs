use rand::prelude::IndexedRandom;
// use rand::rng;
use rand::RngExt;
// use rand::*;
use rusty_engine::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum Round {
    PlayerTurn,
    AiTurn,
    Verification,
}

#[derive(Resource)]
struct GameState {
    grid: [[(u8, Vec2); 3]; 3],
    is_player_to_play: bool,
    wait: bool,
}

fn main() {
    let mut game = Game::new();
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
    line5.translation.y = 200.0;
    let line6 = game.add_sprite("line6", "sprite/separator.png");
    line6.translation.y = -200.0;
    let line7 = game.add_sprite("line7", "sprite/separator.png");
    line7.translation.x = -200.0;
    line7.rotation = UP;
    let line8 = game.add_sprite("line8", "sprite/separator.png");
    let line9 
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
    game.add_logic(draw_content_cell);
    game.add_logic(print_messages);
    game.run(GameState {
        grid,
        is_player_to_play: human_begin,
        wait: true,
    });
}

fn player_manager(engine: &mut Engine, game_state: &mut GameState) {
    if !game_state.is_player_to_play {
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
        game_state.wait = false;
    }
}

fn ai_manager(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.is_player_to_play {
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
        game_state.wait = false;
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

fn print_messages(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.wait {
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
    game_state.wait = true;
}
