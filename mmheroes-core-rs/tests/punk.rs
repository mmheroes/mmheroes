mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::high_scores;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{GameMode, GameScreen, Location};

#[test]
fn look_at_baobab() {
    let high_scores = high_scores![
        "Оля" => 142,
        "Вероника" => 192,
        "Наташа" => 144,
        "Катя" => 113,
        "Рита" => 120,
    ];
    initialize_game!((0, GameMode::Normal, Some(high_scores.clone())) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    replay_game(game_ui, "4↓r↓r");
    assert_matches!(state.borrow().screen(), GameScreen::HighScores(_));
    assert_eq!(game_ui.high_scores, high_scores);
    replay_game(game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(state) => {
        assert_matches!(state.location(), Location::PUNK);
    });
}
