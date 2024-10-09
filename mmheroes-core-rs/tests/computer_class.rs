mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Action, Classmate, GameMode, GameScreen, Subject, Time};

#[test]
fn surf_internet_god_mode() {
    initialize_game!((0, GameMode::God) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::GodMode);

    // Идём в мавзолей, ждём пока не появится Гриша
    replay_game(game_ui, "3↑r2↑2r2↑2r");
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Grisha)));

    // Подходим к Грише, принимаем его предложение устроиться в ТЕРКОМ
    replay_game(game_ui, "2↑3r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert!(!state.player().has_internet());
        }
    );

    // Снова подходим к Грише, получаем у него адрес прокси-сервера
    replay_game(game_ui, "2↑2r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert!(state.player().has_internet());
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(Subject::ComputerScience).problems_done(),
                0
            );
        }
    );

    // Идём в компьютерный класс
    replay_game(game_ui, "r5↓r");

    // Фармим задачки для Климова
    for i in 1..=10 {
        replay_game(game_ui, "4↓2r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10 + i));
                assert_eq!(
                    state.player().status_for_subject(Subject::ComputerScience).problems_done(),
                    i
                );
            }
        );
    }
}
