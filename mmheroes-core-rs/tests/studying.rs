mod common;
use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::{
    BrainLevel, CauseOfDeath, GameMode, GameScreen, Subject, Time,
};

#[test]
fn overstudy_to_zero_health() {
    initialize_game!((1641333345581, GameMode::Normal) => state, game_ui);
    replay_game(&mut game_ui, "13r");
    assert_matches!(
          state.borrow().screen(),
          GameScreen::GameEnd(state)
            if matches!(state.player().cause_of_death(), Some(CauseOfDeath::Overstudied))
    );
    replay_game(&mut game_ui, "2r");
    assert_matches!(state.borrow().screen(), GameScreen::Ding(_));
    replay_game(&mut game_ui, "2r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(_));
}

/// Проверяем, что в случае отрицательного brain level попытка подготовиться к зачёту
/// ни к чему не приводит: знание предмета не увеличивается, время не тратится.
#[test]
fn study_with_negative_brain_level() {
    initialize_game!((1641336778475, GameMode::Normal) => state, game_ui);
    replay_game(&mut game_ui, "3r2↓r2↓r4↓r2↑2r4↓r3↑r↓2r3↑r↓2r4↓r↓2r3↑r↑2r3↑r↑2r3↑r↑2r3↑2r3↑2r2↑2r3↑2r3↑2r2↑r↓3r2↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::Study(state) => {
        assert_eq!(state.player().brain(), BrainLevel(-1));
        assert_eq!(
            state
                .player()
                .status_for_subject(Subject::AlgebraAndNumberTheory)
                .knowledge(),
            BrainLevel(5)
        );
        assert_eq!(state.current_time(), Time(15));
    });
    replay_game(&mut game_ui, "r");
    let borrowed_state = state.borrow();
    match borrowed_state.screen() {
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.player().brain(), BrainLevel(-1));
            assert_eq!(
                state
                    .player()
                    .status_for_subject(Subject::AlgebraAndNumberTheory)
                    .knowledge(),
                BrainLevel(5)
            );
            assert_eq!(state.current_time(), Time(15));
        }
        _ => panic!("Unexpected screen"),
    }
}
