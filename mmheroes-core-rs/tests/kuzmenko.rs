use assert_matches::assert_matches;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{GameMode, GameScreen, Time};

mod common;
use common::*;
use mmheroes_core::logic::kuzmenko::KuzmenkoInteraction;
use mmheroes_core::logic::Subject::ComputerScience;

macro_rules! assert_cs_exams {
    ($state:expr, day $day:literal from $from:literal to $to:literal) => {
        let maybe_exam = $state.timetable().day($day).exam(ComputerScience);
        assert_matches!(
            maybe_exam,
            Some(exam) if exam.from().0 == $from && exam.to().0 == $to,
            "Actual day {} exam: {maybe_exam:?}", $day
        );
    };
    ($state:expr, day $day:literal none) => {
        let maybe_exam = $state.timetable().day($day).exam(ComputerScience);
        assert_matches!(
            maybe_exam,
            None,
            "Actual day {} exam: {maybe_exam:?}", $day
        );
    };
}

#[test]
fn kuzmenko_cs_exam_every_day() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Ждём пока не появится Кузьменко
    replay_game(game_ui, "2↓r2↓r4↓r5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 none);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 none);
        }
    );

    // Подходим к Кузьменко, он говорит про журнал «Монитор»
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::MonitorJournal
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 none);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 none);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::OlegPliss
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 none);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 none);
        }
    );

    // Снова подходим к Кузьменко, он сообщает том что 27-го мая принимает Климов
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::AdditionalComputerScienceExam { day_index: 5 }
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 none);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::FormatFloppy
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 none);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко, он сообщает том что 24-го мая принимает Климов
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::AdditionalComputerScienceExam { day_index: 2 }
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::OlegPliss
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::GetYourselvesAnEmail
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 none);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко. Он НЕ сообщает о том, что 23 мая можно сдать зачёт,
    // но новый зачёт появляется в расписании.
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::BillGatesMustDie
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко. Больше новых зачётов не появляется.
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::MmheroesBP7
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::ThirdYear
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::FiltersInWindows
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::FormatFloppy
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::FiltersInWindows
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::STAR
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::STAR
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::CSeminar
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::MmheroesBP7
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );

    // Снова подходим к Кузьменко
    replay_game(game_ui, "r2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KuzmenkoInteraction(
            state,
            KuzmenkoInteraction::TerekhovSenior
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_cs_exams!(state, day 0 none);
            assert_cs_exams!(state, day 1 from 10 to 12);
            assert_cs_exams!(state, day 2 from 13 to 15);
            assert_cs_exams!(state, day 3 from 13 to 15);
            assert_cs_exams!(state, day 4 from 15 to 17);
            assert_cs_exams!(state, day 5 from 13 to 15);
        }
    );
}
