use super::*;
use crate::logic::actions::{HelpAction, UseLectureNotesAction};

pub(super) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    assert_eq!(state.location(), Location::Dorm);
    match action {
        Action::Study => study(g, state).await,
        Action::ViewTimetable => {
            timetable::show(g, state).await;
        }
        Action::Rest => rest(g, state).await,
        Action::GoToBed => sleep(g, state).await,
        Action::GoFromDormToPunk => {
            state.set_location(Location::PUNK);
            misc::decrease_health(
                state,
                HealthLevel::location_change_large_penalty(),
                CauseOfDeath::OnTheWayToPUNK,
            );
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoToMausoleum => {
            state.set_location(Location::Mausoleum);
            misc::decrease_health(
                state,
                HealthLevel::location_change_large_penalty(),
                CauseOfDeath::OnTheWayToMausoleum,
            );
        }
        Action::WhatToDo => {
            show_help(g, state).await;
        }
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) fn subjects_to_study(state: &GameState) -> ActionVec {
    let mut available_actions = Subject::all_subjects()
        .map(|subject| Action::DoStudy {
            subject,
            lecture_notes_available: state
                .player
                .status_for_subject(subject)
                .has_lecture_notes(),
        })
        .collect::<ActionVec>();
    available_actions.push(Action::DontStudy);
    available_actions
}

async fn study(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let available_subjects = subjects_to_study(state);
    g.set_screen_and_action_vec(GameScreen::Study(state.clone()), available_subjects);
    let subject_to_study = match g.wait_for_action().await {
        Action::DoStudy { subject, .. } => subject,
        Action::DontStudy {} => return,
        action => illegal_action!(action),
    };
    let lecture_notes_available = state
        .player
        .status_for_subject(subject_to_study)
        .has_lecture_notes();
    let use_lecture_notes = if lecture_notes_available {
        match g
            .set_screen_and_wait_for_action::<UseLectureNotesAction>(
                GameScreen::PromptUseLectureNotes(state.clone()),
            )
            .await
        {
            UseLectureNotesAction::Yes => true,
            UseLectureNotesAction::No => false,
        }
    } else {
        false
    };
    study_subject(g, state, subject_to_study, use_lecture_notes).await
}

async fn study_subject(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
    use_lecture_notes: bool,
) {
    // Импликация "использовать конспект => у игрока есть конспект"
    // должна быть истинной
    assert!(
        !use_lecture_notes
            || state.player.status_for_subject(subject).has_lecture_notes(),
        "Нельзя воспользоваться конспектом, так как его на самом деле нет"
    );
    let brain_or_stamina = if subject == Subject::PhysicalEducation {
        state.player.stamina.0
    } else {
        state.player.brain.0
    };
    if brain_or_stamina <= 0 {
        return;
    }
    let health = state.player.health;
    let current_time = state.current_time();
    let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
    *knowledge += if current_time.is_optimal_study_time() {
        brain_or_stamina
    } else {
        brain_or_stamina * 2 / 3
    };
    *knowledge -= g.rng.random(brain_or_stamina / 2);
    *knowledge += g.rng.random(health.0 / 18);
    if use_lecture_notes {
        *knowledge += 10
    }
    assert!(*knowledge >= BrainLevel(0));
    assert!(state.player.stamina >= StaminaLevel(0));
    let mut health_penalty = 10 - g.rng.random(state.player.stamina.0);
    if health_penalty < 0 || use_lecture_notes {
        health_penalty = 0;
    }
    if current_time.is_suboptimal_study_time() {
        health_penalty += 12;
    }
    misc::decrease_health(
        state,
        HealthLevel(health_penalty),
        CauseOfDeath::Overstudied,
    );
    if state
        .player
        .status_for_subject(subject)
        .knowledge
        .is_lethal()
    {
        misc::decrease_health(state, HealthLevel(10), CauseOfDeath::StudiedTooWell);
    }
    misc::hour_pass(g, state, None).await
}

async fn rest(g: &mut InternalGameState<'_>, state: &mut GameState) {
    state.player.health += g.rng.random_in_range(7..15);
    misc::hour_pass(g, state, None).await
}

pub(in crate::logic) async fn sleep(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    if state.current_time() > Time(3) && state.current_time() < Time(20) {
        g.set_screen_and_wait_for_any_key(GameScreen::Sleep(state.clone()))
            .await;
    } else {
        todo!("Реализовать что-то помимо неудавшегося сна")
    }
}

async fn show_help(g: &mut InternalGameState<'_>, state: &GameState) {
    let mut help_action = HelpAction::WhatToDoAtAll;
    loop {
        let help_screen = match help_action {
            HelpAction::WhatToDoAtAll => GameScreen::WhatToDo(state.clone()),
            HelpAction::AboutScreen => GameScreen::AboutScreen(state.clone()),
            HelpAction::WhereToGoAndWhy => GameScreen::WhereToGoAndWhy(state.clone()),
            HelpAction::AboutProfessors => GameScreen::AboutProfessors(state.clone()),
            HelpAction::AboutCharacters => GameScreen::AboutCharacters(state.clone()),
            HelpAction::AboutThisProgram => GameScreen::AboutThisProgram(state.clone()),
            HelpAction::ThanksButNothing => return,
        };
        help_action = g
            .set_screen_and_wait_for_action::<HelpAction>(help_screen)
            .await;
    }
}
