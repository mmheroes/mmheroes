use assert_matches::assert_matches;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Action, GameMode, GameScreen, Subject, Time};

mod common;
use common::*;
use mmheroes_core::logic::sasha::SashaInteraction;
use Subject::{AlgebraAndNumberTheory, Calculus, GeometryAndTopology};

#[test]
fn sasha() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Ждём когда на факультет приходит Саша
    replay_game(game_ui, "2↓r2↓r");

    // Идём на факультет, обращаемся к Саше
    replay_game(game_ui, "4↓r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(_, SashaInteraction::ChooseSubject)
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::RequestLectureNotesFromSasha(AlgebraAndNumberTheory),
            Action::RequestLectureNotesFromSasha(Calculus),
            Action::RequestLectureNotesFromSasha(GeometryAndTopology),
            Action::DontNeedAnythingFromSasha,
        ]
    );

    // Просим конспект по алгебре, Саша отказывает
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(state, SashaInteraction::SorryGaveToSomeoneElse) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(!state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(!state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    // Снова обращаемся к Саше
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(_, SashaInteraction::ChooseSubject)
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::RequestLectureNotesFromSasha(AlgebraAndNumberTheory),
            Action::RequestLectureNotesFromSasha(Calculus),
            Action::RequestLectureNotesFromSasha(GeometryAndTopology),
            Action::DontNeedAnythingFromSasha,
        ]
    );

    // Просим конспект по матанализу, Саша отказывает
    replay_game(game_ui, "↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(state, SashaInteraction::SorryGaveToSomeoneElse) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(!state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(!state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    // Снова обращаемся к Саше
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(_, SashaInteraction::ChooseSubject)
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::RequestLectureNotesFromSasha(AlgebraAndNumberTheory),
            Action::RequestLectureNotesFromSasha(Calculus),
            Action::RequestLectureNotesFromSasha(GeometryAndTopology),
            Action::DontNeedAnythingFromSasha,
        ]
    );

    // Просим конспект по геометрии, Саша соглашается
    replay_game(game_ui, "2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(state, SashaInteraction::YesIHaveTheLectureNotes) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert!(!state.player().status_for_subject(AlgebraAndNumberTheory).has_lecture_notes());
            assert!(!state.player().status_for_subject(Calculus).has_lecture_notes());
            assert!(state.player().status_for_subject(GeometryAndTopology).has_lecture_notes());
        }
    );

    // Снова обращаемся к Саше
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(_, SashaInteraction::ChooseSubject)
    );
    assert_eq!(
        state.observable_state().available_actions(),
        [
            Action::RequestLectureNotesFromSasha(AlgebraAndNumberTheory),
            Action::RequestLectureNotesFromSasha(Calculus),
            Action::DontNeedAnythingFromSasha,
        ]
    );

    // Уходим
    replay_game(game_ui, "↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SashaInteraction(_, SashaInteraction::SuitYourself)
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(_)
    );

    // Проверяем, что конспекты никогда не появляются у Саши снова.
    for i in 0..200 {
        // Снова обращаемся к Саше
        replay_game(game_ui, "2↑r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SashaInteraction(_, SashaInteraction::ChooseSubject)
        );

        // Выбираем алгебру или матанализ
        if i % 2 == 0 {
            replay_game(game_ui, "r");
        } else {
            replay_game(game_ui, "↓r");
        }
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SashaInteraction(_, SashaInteraction::SorryGaveToSomeoneElse)
        );
        replay_game(game_ui, "r");
    }
}
