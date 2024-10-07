mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Action, GameMode, GameScreen, Subject};

#[test]
fn exam_list_in_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);

    // Идём на факультет. Утром на факультете никого нет.
    // Список экзаменов в первый день:
    // - Алгебра 13:00–15:00
    // - English 14:00-16:00
    replay_game(&mut game_ui, "4↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::GoToProfessor(_));
    assert_eq!(
        state.borrow().available_actions(),
        [Action::DontGoToProfessor]
    );

    // Идём в общагу и отдыхаем до 13:00
    replay_game(&mut game_ui, "r2↓r2↓r2↓r2↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(&mut game_ui, "4↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::GoToProfessor(_));
    assert_eq!(
        state.borrow().available_actions(),
        [
            Action::Exam(Subject::AlgebraAndNumberTheory),
            Action::DontGoToProfessor
        ]
    );

    // Идём в общагу и отдыхаем до 14:00
    replay_game(&mut game_ui, "↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(&mut game_ui, "4↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::GoToProfessor(_));
    assert_eq!(
        state.borrow().available_actions(),
        [
            Action::Exam(Subject::AlgebraAndNumberTheory),
            Action::Exam(Subject::English),
            Action::DontGoToProfessor
        ]
    );

    // Идём в общагу и отдыхаем до 15:00
    replay_game(&mut game_ui, "2↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(&mut game_ui, "4↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::GoToProfessor(_));
    assert_eq!(
        state.borrow().available_actions(),
        [Action::Exam(Subject::English), Action::DontGoToProfessor]
    );

    // Идём в общагу и отдыхаем до 16:00
    replay_game(&mut game_ui, "↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(&mut game_ui, "4↓2r");
    assert_matches!(state.borrow().screen(), GameScreen::GoToProfessor(_));
    assert_eq!(
        state.borrow().available_actions(),
        [Action::DontGoToProfessor]
    );
}
