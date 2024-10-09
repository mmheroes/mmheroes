mod cafe;
pub(in crate::logic) mod computer_class;
pub(in crate::logic) mod dorm;
mod exams;
pub(in crate::logic) mod mausoleum;
pub(in crate::logic) mod pdmi;
pub(in crate::logic) mod punk;
pub mod train;

use super::*;

pub(in crate::logic) type RouterResult = Result<(), entry_point::GameEnd>;

pub(super) fn available_actions(state: &GameState) -> ActionVec {
    // TODO: assert that no exam is in progress
    let location = state.location;

    let add_classmates = |available_actions: &mut ActionVec| {
        for classmate_info in state.classmates.filter_by_location(location) {
            available_actions
                .push(Action::InteractWithClassmate(classmate_info.classmate()));
        }
    };

    match location {
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
    }
}

pub(super) async fn run(
    g: &mut InternalGameState<'_>,
    mut state: GameState,
) -> entry_point::GameEnd {
    loop {
        g.set_screen_and_action_vec(
            GameScreen::SceneRouter(state.clone()),
            available_actions(&state),
        );
        let router_action = g.wait_for_action().await;
        match handle_router_action(g, &mut state, router_action).await {
            Ok(()) => continue,
            Err(game_end) => return game_end,
        }
    }
}

async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) -> RouterResult {
    if action == Action::IAmDone {
        return i_am_done(g, state).await;
    }

    use Location::*;
    match state.location() {
        PUNK => return punk::handle_router_action(g, state, action).await,
        PDMI => return pdmi::handle_router_action(g, state, action).await,
        ComputerClass => {
            return computer_class::handle_router_action(g, state, action).await
        }
        Dorm => return dorm::handle_router_action(g, state, action).await,
        Mausoleum => mausoleum::handle_router_action(g, state, action).await,
    }

    // LEGACY
    loop {
        let action = g.wait_for_action().await;
        if action == Action::IAmDone {
            return i_am_done(g, state).await;
        }
        let new_actions = g.perform_action(action);
        g.set_available_actions_from_vec(new_actions);
    }
}

pub(in crate::logic) async fn i_am_done(
    g: &mut InternalGameState<'_>,
    state: &GameState,
) -> RouterResult {
    g.set_screen_and_available_actions(
        GameScreen::IAmDone(state.clone()),
        [Action::NoIAmNotDone, Action::IAmCertainlyDone],
    );
    match g.wait_for_action().await {
        Action::NoIAmNotDone => Ok(()),
        Action::IAmCertainlyDone => Err(misc::game_end(g, state).await),
        action => illegal_action!(action),
    }
}
