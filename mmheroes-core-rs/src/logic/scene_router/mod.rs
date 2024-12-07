mod cafe;
pub(in crate::logic) mod computer_class;
pub mod dorm;
pub mod exams;
pub(in crate::logic) mod mausoleum;
pub(in crate::logic) mod pdmi;
pub(in crate::logic) mod punk;
pub mod terkom;
pub mod train;

use super::*;
use crate::logic::actions::GameEndAction;

fn add_classmates(available_actions: &mut ActionVec, state: &GameState) {
    let location = state.location();
    available_actions.extend(
        state
            .classmates
            .filter_by_location(location)
            .map(|classmate_info| {
                Action::InteractWithClassmate(classmate_info.classmate())
            }),
    );
}

fn scene_punk(state: &GameState) -> ActionVec {
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
    add_classmates(&mut available_actions, state);
    if state.player.is_employed_at_terkom()
        && state.current_time() < Time::terkom_closing_time()
    {
        available_actions.push(Action::GoToWork);
    }
    available_actions.push(Action::IAmDone);
    available_actions
}

fn scene_pdmi(state: &GameState) -> ActionVec {
    let mut available_actions = ActionVec::from([
        Action::GoToProfessor,
        Action::LookAtBulletinBoard,
        Action::GoToCafePDMI,
        Action::GoToPUNKFromPDMI,
    ]);
    add_classmates(&mut available_actions, state);
    available_actions.push(Action::IAmDone);
    available_actions
}

async fn scene_computer_class(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> Option<ActionVec> {
    if state.current_time() > Time::computer_class_closing() {
        g.set_screen_and_wait_for_any_key(GameScreen::ComputerClassClosing(
            state.clone(),
        ))
        .await;
        state.set_location(Location::Dorm);
        misc::decrease_health(
            state,
            HealthLevel(g.rng.random(5)),
            CauseOfDeath::OnTheWayToDorm,
        );
        return None;
    }
    let mut available_actions = ActionVec::new();
    if Location::ComputerClass.is_exam_here_now(
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
    add_classmates(&mut available_actions, state);
    available_actions.push(Action::IAmDone);
    Some(available_actions)
}

async fn scene_dorm(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> Option<ActionVec> {
    let time_to_sleep =
        Time((23 - core::cmp::max(50 - state.player.health.0, 0) / 12) as u8);
    let current_time = state.current_time();
    if current_time > time_to_sleep || current_time <= Time(3) {
        // В оригинальной реализации есть проверка current_time <= Time(3), но она,
        // кажется, никогда не должна быть истинной.
        assert!(current_time > Time(3), "В такое время уже нужно спать");
        g.set_screen_and_wait_for_any_key(GameScreen::CantStayAwake(state.clone()))
            .await;
        sleep::sleep(g, state).await;
        return None;
    } else if current_time >= Time(18)
        && g.rng.random(10) < 3
        && !state.player.is_invited_to_party()
    {
        state.player.set_invited_to_party(true);
        dorm::invite_from_neighbor(g, state).await;
        return None;
    }
    Some(ActionVec::from([
        Action::Study,
        Action::ViewTimetable,
        Action::Rest,
        Action::GoToBed,
        Action::GoFromDormToPunk,
        Action::GoToPDMI,
        Action::GoToMausoleum,
        Action::IAmDone,
        Action::WhatToDo,
    ]))
}

fn scene_mausoleum(state: &GameState) -> ActionVec {
    let mut available_actions = ActionVec::from([
        Action::GoFromMausoleumToPunk,
        Action::GoToPDMI,
        Action::GoFromMausoleumToDorm,
        Action::Rest,
    ]);
    add_classmates(&mut available_actions, state);
    available_actions.push(Action::IAmDone);
    available_actions
}

pub(super) async fn run(
    g: &mut InternalGameState<'_>,
    mut state: GameState,
) -> entry_point::GameEnd {
    loop {
        let available_actions = match state.location() {
            Location::PUNK => scene_punk(&state),
            Location::PDMI => scene_pdmi(&state),
            Location::ComputerClass => match scene_computer_class(g, &mut state).await {
                Some(available_actions) => available_actions,
                None => continue,
            },
            Location::Dorm => match scene_dorm(g, &mut state).await {
                Some(available_actions) => available_actions,
                None => continue,
            },
            Location::Mausoleum => scene_mausoleum(&state),
        };
        g.set_screen_and_action_vec(
            GameScreen::SceneRouter(state.clone()),
            available_actions,
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
