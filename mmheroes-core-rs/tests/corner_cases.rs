mod common;

use assert_matches::*;
use common::*;
use mmheroes_core::logic::*;
use mmheroes_core::ui::Input;

#[test]
fn overstudy_to_zero_health() {
    let mut game = Game::new(GameMode::Normal, 1641333345581);
    let mut game_ui =
        TestGameUI::new(&mut game, None, TestRendererRequestConsumer::new());
    game_ui.continue_game(Input::Enter);
    replay_game(&mut game_ui, "13r");
    assert_matches!(
          game.screen(),
          GameScreen::GameEnd(state)
            if matches!(state.player().cause_of_death(), Some(CauseOfDeath::Overstudied))
    );
}

/// Проверяем, что в случае отрицательного brain level попытка подготовиться к зачёту
/// ни к чему не приводит: знание предмета не увеличивается, время не тратится.
#[test]
fn study_with_negative_brain_level() {
    let mut game = Game::new(GameMode::Normal, 1641336778475);
    let mut game_ui =
        TestGameUI::new(&mut game, None, TestRendererRequestConsumer::new());
    game_ui.continue_game(Input::Enter);
    replay_game(&mut game_ui, "3r2↓r2↓r4↓r2↑2r4↓r3↑r↓2r3↑r↓2r4↓r↓2r3↑r↑2r3↑r↑2r3↑r↑2r3↑2r3↑2r2↑2r3↑2r3↑2r2↑r↓3r2↓2r");
    match game_ui.game().screen() {
        GameScreen::Study(state) => {
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
    replay_game(&mut game_ui, "r");
    match game_ui.game().screen() {
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
