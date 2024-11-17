mod cafe;
pub(in crate::logic) mod computer_class;
pub(in crate::logic) mod dorm;
pub mod exams;
pub(in crate::logic) mod mausoleum;
pub(in crate::logic) mod pdmi;
pub(in crate::logic) mod punk;
pub mod terkom;
pub mod train;

use super::*;
use crate::logic::actions::GameEndAction;

pub(super) fn available_actions(state: &GameState) -> ActionVec {
    assert!(state.exam_in_progress().is_none());
    let location = state.location();

    let add_classmates = |available_actions: &mut ActionVec| {
        available_actions.extend(state.classmates.filter_by_location(location).map(
            |classmate_info| Action::InteractWithClassmate(classmate_info.classmate()),
        ));
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
            if state.current_time() < Time::computer_class_closing() {
                available_actions.push(Action::GoToComputerClass);
            }
            if state.current_time().is_cafe_open() {
                available_actions.push(Action::GoToCafePUNK);
            }
            add_classmates(&mut available_actions);
            if state.player.is_employed_at_terkom()
                && state.current_time() < Time::terkom_closing_time()
            {
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
            if state.current_time() > Time::computer_class_closing() {
                todo!("Класс закрывается. Пошли домой!")
            }
            let mut available_actions = ActionVec::new();
            if location.is_exam_here_now(
                Subject::ComputerScience,
                state.current_day(),
                state.current_time(),
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
        if router_action == Action::IAmDone {
            match i_am_done(g, &state).await {
                None => continue,
                Some(game_end) => return game_end,
            }
        }
        handle_router_action(g, &mut state, router_action).await;
        if state.player.cause_of_death.is_some() {
            return misc::game_end(g, &state).await;
        }
    }
}

async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    use Location::*;
    match state.location() {
        PUNK => punk::handle_router_action(g, state, action).await,
        PDMI => pdmi::handle_router_action(g, state, action).await,
        ComputerClass => computer_class::handle_router_action(g, state, action).await,
        Dorm => dorm::handle_router_action(g, state, action).await,
        Mausoleum => mausoleum::handle_router_action(g, state, action).await,
    }
}

async fn i_am_done(
    g: &mut InternalGameState<'_>,
    state: &GameState,
) -> Option<entry_point::GameEnd> {
    match g
        .set_screen_and_wait_for_action::<GameEndAction>(GameScreen::IAmDone(
            state.clone(),
        ))
        .await
    {
        GameEndAction::NoIAmNotDone => None,
        GameEndAction::IAmCertainlyDone => Some(misc::game_end(g, state).await),
    }
}
