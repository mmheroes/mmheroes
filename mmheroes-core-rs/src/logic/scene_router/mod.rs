pub(in crate::logic) mod computer_class;
pub(in crate::logic) mod dorm;
pub(in crate::logic) mod mausoleum;
pub(in crate::logic) mod pdmi;
pub(in crate::logic) mod punk;
pub mod train;

use super::*;

pub(super) fn run(game: &mut InternalGameState, state: GameState) -> ActionVec {
    // TODO: assert that no exam is in progress
    let location = state.location;

    let add_classmates = |available_actions: &mut ActionVec| {
        for classmate_info in state.classmates.filter_by_location(location) {
            available_actions
                .push(Action::InteractWithClassmate(classmate_info.classmate()));
        }
    };

    let available_actions = match location {
        Location::PDMI => {
            let mut available_actions = ActionVec::from([
                Action::GoToProfessor,
                Action::LookAtBulletinBoard,
                Action::RestInCafePDMI,
                Action::GoToPUNKFromPDMI,
            ]);
            add_classmates(&mut available_actions);
            available_actions.push(Action::IAmDone);
            available_actions
        }
        Location::PUNK => {
            let mut available_actions = ActionVec::from([
                Action::GoToProfessor,
                Action::LookAtBaobab,
                Action::GoFromPunkToDorm,
                Action::GoToPDMI,
                Action::GoToMausoleum,
            ]);
            if state.current_time < Time::computer_class_closing() {
                available_actions.push(Action::GoToComputerClass);
            }
            if state.current_time.is_cafe_open() {
                available_actions.push(Action::GoToCafePUNK);
            }
            add_classmates(&mut available_actions);
            if state.player.is_employed_at_terkom() {
                available_actions.push(Action::GoToWork);
            }
            available_actions.push(Action::IAmDone);
            available_actions
        }
        Location::Mausoleum => {
            let mut available_actions = ActionVec::from([
                Action::GoFromMausoleumToPunk,
                Action::GoToPDMI,
                Action::GoFromMausoleumToDorm,
                Action::Rest,
            ]);
            for classmate_info in state.classmates.filter_by_location(location) {
                available_actions
                    .push(Action::InteractWithClassmate(classmate_info.classmate()));
            }
            available_actions.push(Action::IAmDone);
            available_actions
        }
        Location::Dorm => ActionVec::from([
            Action::Study,
            Action::ViewTimetable,
            Action::Rest,
            Action::GoToBed,
            Action::GoFromDormToPunk,
            Action::GoToPDMI,
            Action::GoToMausoleum,
            Action::IAmDone,
            Action::WhatToDo,
        ]),
        Location::ComputerClass => {
            if state.current_time > Time::computer_class_closing() {
                todo!("Класс закрывается. Пошли домой!")
            }
            let mut available_actions = ActionVec::new();
            if location.is_exam_here_now(
                Subject::ComputerScience,
                state.current_day(),
                state.current_time,
            ) {
                available_actions.push(Action::Exam(Subject::ComputerScience));
            }
            available_actions.push(Action::GoFromPunkToDorm);
            available_actions.push(Action::LeaveComputerClass);
            available_actions.push(Action::GoToPDMI);
            available_actions.push(Action::GoToMausoleum);
            if state.player.has_internet() {
                available_actions.push(Action::SurfInternet);
            }
            if state.player.has_mmheroes_floppy() {
                available_actions.push(Action::PlayMMHEROES);
            }
            for classmate_info in state.classmates.filter_by_location(location) {
                available_actions
                    .push(Action::InteractWithClassmate(classmate_info.classmate()));
            }
            available_actions.push(Action::IAmDone);
            available_actions
        }
    };
    game.set_screen(GameScreen::SceneRouter(state));
    available_actions
}

pub(super) fn handle_action(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    match state.location() {
        Location::PUNK => punk::handle_action(game, state, action),
        Location::PDMI => pdmi::handle_action(game, state, action),
        Location::ComputerClass => computer_class::handle_action(game, state, action),
        Location::Dorm => dorm::handle_action(game, state, action),
        Location::Mausoleum => mausoleum::handle_action(game, state, action),
    }
}

pub(super) fn i_am_done(game: &mut InternalGameState, state: GameState) -> ActionVec {
    game.set_screen(GameScreen::IAmDone(state));
    ActionVec::from([Action::NoIAmNotDone, Action::IAmCertainlyDone])
}

pub(super) fn handle_i_am_done(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::NoIAmNotDone => run(game, state),
        Action::IAmCertainlyDone => game_end(game, state),
        _ => illegal_action!(action),
    }
}

pub(super) fn game_end(game: &mut InternalGameState, state: GameState) -> ActionVec {
    game.set_screen(GameScreen::GameEnd(state));
    wait_for_any_key()
}

pub(super) fn wanna_try_again(game: &mut InternalGameState) -> ActionVec {
    game.set_screen(GameScreen::WannaTryAgain);
    // Хочешь попробовать снова? Да или нет.
    ActionVec::from([Action::WantToTryAgain, Action::DontWantToTryAgain])
}

pub(super) fn handle_wanna_try_again(
    game: &mut InternalGameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::WantToTryAgain => entry_point::start_game(game),
        Action::DontWantToTryAgain => {
            game.set_screen(GameScreen::Disclaimer);
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
