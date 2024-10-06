mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{CauseOfDeath, GameMode, GameScreen, HealthLevel, Location};

#[test]
fn go_from_dorm_to_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);
    replay_game(&mut game_ui, "4↓r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.location(), Location::PUNK);
    });
}

#[test]
fn death_on_the_way_from_dorm_to_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(&mut game_ui, "10r");

    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.player().health(), HealthLevel(2));
    });

    // Идём на факультет
    replay_game(&mut game_ui, "4↓r");

    assert_matches!(state.borrow().screen(), GameScreen::GameEnd(state) => {
        assert_matches!(state.player().cause_of_death(), Some(CauseOfDeath::OnTheWayToPUNK));
    });
}
