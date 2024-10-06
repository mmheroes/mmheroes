use super::*;
use crate::logic::actions::HelpAction;

pub(in crate::logic) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) -> RouterResult {
    assert_eq!(state.location, Location::Dorm);
    let available_actions = match action {
        Action::Study => return study(g, state).await,
        Action::ViewTimetable => {
            timetable::show(g, state).await;
            return Ok(());
        }
        Action::Rest => return rest(g, state).await,
        Action::GoToBed => return sleep(g, state).await,
        Action::GoFromDormToPunk => {
            state.location = Location::PUNK;
            return misc::decrease_health(
                g,
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
            )
            .await;
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state.clone()),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            return misc::decrease_health(
                g,
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
            )
            .await;
        }
        Action::WhatToDo => {
            show_help(g, state).await;
            return Ok(());
        }
        _ => illegal_action!(action),
    };

    // LEGACY
    g.set_available_actions_from_vec(available_actions);
    loop {
        let action = g.wait_for_action().await;
        if action == Action::IAmDone {
            return i_am_done(g, state).await;
        }
        let new_actions = g.perform_action(action);
        g.set_available_actions_from_vec(new_actions);
    }
}

pub(in crate::logic) fn subjects_to_study(state: &GameState) -> ActionVec {
    let mut available_actions = SUBJECTS
        .iter()
        .map(|(subject, _)| Action::DoStudy {
            subject: *subject,
            lecture_notes_available: state
                .player
                .status_for_subject(*subject)
                .has_lecture_notes(),
        })
        .collect::<ActionVec>();
    available_actions.push(Action::DontStudy);
    available_actions
}

async fn study(g: &mut InternalGameState<'_>, state: &mut GameState) -> RouterResult {
    let available_subjects = subjects_to_study(state);
    g.set_screen_and_action_vec(GameScreen::Study(state.clone()), available_subjects);
    let subject_to_study = match g.wait_for_action().await {
        Action::DoStudy { subject, .. } => subject,
        Action::DontStudy {} => return Ok(()),
        action => illegal_action!(action),
    };
    let lecture_notes_available = state
        .player
        .status_for_subject(subject_to_study)
        .has_lecture_notes();
    let use_lecture_notes = if lecture_notes_available {
        g.set_screen_and_available_actions(
            GameScreen::PromptUseLectureNotes(state.clone()),
            [
                Action::UseLectureNotes(subject_to_study),
                Action::DontUseLectureNotes(subject_to_study),
            ],
        );
        match g.wait_for_action().await {
            Action::UseLectureNotes(_) => true,
            Action::DontUseLectureNotes(_) => false,
            action => illegal_action!(action),
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
) -> RouterResult {
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
        return Ok(());
    }
    let health = state.player.health;
    let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
    *knowledge += if state.current_time.is_optimal_study_time() {
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
    if state.current_time.is_suboptimal_study_time() {
        health_penalty += 12;
    }
    misc::decrease_health(
        g,
        HealthLevel(health_penalty),
        state,
        CauseOfDeath::Overstudied,
    )
    .await?;
    if state
        .player
        .status_for_subject(subject)
        .knowledge
        .is_lethal()
    {
        misc::decrease_health(g, HealthLevel(10), state, CauseOfDeath::StudiedTooWell)
            .await?;
    }
    misc::hour_pass(g, state).await
}

async fn rest(g: &mut InternalGameState<'_>, state: &mut GameState) -> RouterResult {
    state.player.health += g.rng.random_in_range(7..15);
    misc::hour_pass(g, state).await
}

pub(in crate::logic) async fn sleep(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> RouterResult {
    if state.current_time > Time(3) && state.current_time < Time(20) {
        g.set_screen_and_wait_for_any_key(GameScreen::Sleep(state.clone()))
            .await;
        Ok(())
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
        g.set_screen_and_available_actions(
            help_screen,
            [
                Action::Help(HelpAction::WhatToDoAtAll),
                Action::Help(HelpAction::AboutScreen),
                Action::Help(HelpAction::WhereToGoAndWhy),
                Action::Help(HelpAction::AboutProfessors),
                Action::Help(HelpAction::AboutCharacters),
                Action::Help(HelpAction::AboutThisProgram),
                Action::Help(HelpAction::ThanksButNothing),
            ],
        );
        help_action = match g.wait_for_action().await {
            Action::Help(help_action) => help_action,
            action => illegal_action!(action),
        }
    }
}
