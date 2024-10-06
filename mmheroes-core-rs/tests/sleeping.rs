mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{GameMode, GameScreen, Time};

#[test]
fn try_to_sleep() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.current_time(), Time(8))
    });
    replay_game(&mut game_ui, "3↓r");
    assert_matches!(state.borrow().screen(), GameScreen::Sleep(_));
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.current_time(), Time(8))
    });
}
