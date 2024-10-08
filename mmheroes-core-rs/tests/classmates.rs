mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::pasha::PashaInteraction;
use mmheroes_core::logic::Classmate::Pasha;
use mmheroes_core::logic::{Action, GameMode, GameScreen, Time};

#[test]
fn pasha() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_subject_knowledge!(
                state,
                algebra: 1,
                calculus: 0,
                geometry: 2,
                cs: 2,
                english: 2,
                pe: 2,
            );
        }
    );

    // Идём на факультет и проверяем что Паши нет
    replay_game(game_ui, "4↓r");
    assert!(!state
        .borrow()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Pasha)));

    // Отдыхаем до 9, идём и проверяем что Паши нет
    replay_game(game_ui, "2↓r2↓r4↓r");
    assert!(!state
        .borrow()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Pasha)));

    // Отдыхаем до 10, идём и проверяем что Паша появился
    replay_game(game_ui, "2↓r2↓r4↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().got_stipend());
            assert_characteristics!(
                state,
                health: 54,
                money: 0,
                brain: 3,
                stamina: 2,
                charisma: 9
            );
        }
    );
    assert!(state
        .borrow()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Pasha)));

    // Подходим к Паше
    replay_game(game_ui, "3↑r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::PashaInteraction(state, PashaInteraction::Stipend) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().got_stipend());
            assert_characteristics!(
                state,
                health: 54,
                money: 0,
                brain: 3,
                stamina: 2,
                charisma: 9,
            );
        }
    );

    // Убеждаемся что Паша отдал стипендию
    replay_game(game_ui, "r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(state.player().got_stipend());
            assert_characteristics!(
                state,
                health: 54,
                money: 50,
                brain: 3,
                stamina: 2,
                charisma: 9,
            );
            assert_subject_knowledge!(
                state,
                algebra: 1,
                calculus: 0,
                geometry: 2,
                cs: 2,
                english: 2,
                pe: 2,
            );
        }
    );

    // Подходим к Паше снова
    replay_game(game_ui, "3↑r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::PashaInteraction(state, PashaInteraction::Inspiration) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 54,
                money: 50,
                brain: 3,
                stamina: 2,
                charisma: 9,
            );
            assert_subject_knowledge!(
                state,
                algebra: 1,
                calculus: 0,
                geometry: 2,
                cs: 2,
                english: 2,
                pe: 2,
            );
        }
    );

    // Убеждаемся что Паша не уменьшил знания по предметам
    // (потому что они слишком маленькие), по увеличил выносливость
    replay_game(game_ui, "r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 54,
                money: 50,
                brain: 3,
                stamina: 3,
                charisma: 9,
            );
            assert_subject_knowledge!(
                state,
                algebra: 1,
                calculus: 0,
                geometry: 2,
                cs: 2,
                english: 2,
                pe: 2,
            );
        }
    );
}

#[test]
fn pasha_decreases_subject_knowledge() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::CleverStudent);
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 40,
                money: 0,
                brain: 5,
                stamina: 2,
                charisma: 3,
            );
            assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 0,
                geometry: 3,
                cs: 0,
                english: 4,
                pe: 0,
            );
        }
    );

    // Подтягиваем знания по информатике и матанализу
    replay_game(game_ui, "r3↓2r↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 21,
                money: 0,
                brain: 5,
                stamina: 2,
                charisma: 3,
            );
            assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 5,
                geometry: 3,
                cs: 5,
                english: 4,
                pe: 0,
            );
        }
    );

    // Идём на факультет, берём у Паши стипендию
    replay_game(game_ui, "4↓r7↓2r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 18,
                money: 50,
                brain: 5,
                stamina: 2,
                charisma: 3,
            );
           assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 5,
                geometry: 3,
                cs: 5,
                english: 4,
                pe: 0,
            );
        }
    );

    // Идём к Паше за вдохновением, убеждаемся что выносливость увеличилась,
    // а знания по некоторым предметам уменьшились
    replay_game(game_ui, "7↓2r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 18,
                money: 50,
                brain: 5,
                stamina: 3,
                charisma: 3,
            );
            assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 5,
                geometry: 3,
                cs: 3,
                english: 4,
                pe: 0,
            );
        }
    );

    // И ещё раз
    replay_game(game_ui, "7↓2r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 18,
                money: 50,
                brain: 5,
                stamina: 4,
                charisma: 3,
            );
            assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 5,
                geometry: 3,
                cs: 3,
                english: 4,
                pe: 0,
            );
        }
    );

    // И ещё раз
    replay_game(game_ui, "7↓2r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_characteristics!(
                state,
                health: 18,
                money: 50,
                brain: 5,
                stamina: 5,
                charisma: 3,
            );
            assert_subject_knowledge!(
                state,
                algebra: 2,
                calculus: 3,
                geometry: 3,
                cs: 3,
                english: 4,
                pe: 0,
            );
        }
    );
}
