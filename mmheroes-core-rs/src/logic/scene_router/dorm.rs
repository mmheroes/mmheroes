use super::*;
use crate::logic::actions::HelpAction;

pub(in crate::logic) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) -> RouterResult {
    assert_eq!(state.location, Location::Dorm);
    let available_actions = match action {
        Action::Study => choose_subject_to_study(g, state.clone()),
        Action::ViewTimetable => {
            timetable::show(g, state).await;
            return RouterResult::ReturnToRouter;
        }
        Action::Rest => rest(g, state.clone()),
        Action::GoToBed => try_to_sleep(g, state.clone()),
        Action::GoFromDormToPunk => {
            state.location = Location::PUNK;
            g.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state.clone(),
                CauseOfDeath::OnTheWayToPUNK,
                |g, state| legacy::scene_router_run(g, state),
            )
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state.clone()),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            g.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state.clone(),
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| legacy::scene_router_run(g, state),
            )
        }
        Action::WhatToDo => {
            handle_what_to_do(g, state.clone(), HelpAction::WhatToDoAtAll)
        }
        _ => illegal_action!(action),
    };

    // LEGACY
    g.set_available_actions_from_vec(available_actions);
    loop {
        let action = g.wait_for_action().await;
        let new_actions = g.perform_action(action);
        g.set_available_actions_from_vec(new_actions);
    }
}

pub(in crate::logic) fn choose_subject_to_study(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
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
    game.set_screen(GameScreen::Study(state));
    available_actions
}

pub(in crate::logic) fn choose_use_lecture_notes(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::DoStudy {
            subject,
            lecture_notes_available,
        } => {
            assert_eq!(
                state.player.status_for_subject(subject).has_lecture_notes(),
                lecture_notes_available
            );
            if lecture_notes_available {
                game.set_screen(GameScreen::PromptUseLectureNotes(state));
                ActionVec::from([
                    Action::UseLectureNotes(subject),
                    Action::DontUseLectureNotes(subject),
                ])
            } else {
                study(game, state, subject, false)
            }
        }
        Action::DontStudy => legacy::scene_router_run(game, &state),
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) fn study(
    game: &mut InternalGameState,
    mut state: GameState,
    subject: Subject,
    use_lecture_notes: bool,
) -> ActionVec {
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
        return legacy::scene_router_run(game, &state);
    }
    let health = state.player.health;
    let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
    *knowledge += if state.current_time.is_optimal_study_time() {
        brain_or_stamina
    } else {
        brain_or_stamina * 2 / 3
    };
    *knowledge -= game.rng.random(brain_or_stamina / 2);
    *knowledge += game.rng.random(health.0 / 18);
    if use_lecture_notes {
        *knowledge += 10
    }
    assert!(*knowledge >= BrainLevel(0));
    assert!(state.player.stamina >= StaminaLevel(0));
    let mut health_penalty = 10 - game.rng.random(state.player.stamina.0);
    if health_penalty < 0 || use_lecture_notes {
        health_penalty = 0;
    }
    if state.current_time.is_suboptimal_study_time() {
        health_penalty += 12;
    }

    game.decrease_health(
        HealthLevel(health_penalty),
        state,
        CauseOfDeath::Overstudied,
        |game, state| {
            if state
                .player
                .status_for_subject(subject)
                .knowledge
                .is_lethal()
            {
                game.decrease_health(
                    HealthLevel(10),
                    state.clone(),
                    CauseOfDeath::StudiedTooWell,
                    |game, state| game.hour_pass(state.clone()),
                )
            } else {
                game.hour_pass(state.clone())
            }
        },
    )
}

pub(in crate::logic) fn rest(
    game: &mut InternalGameState,
    mut state: GameState,
) -> ActionVec {
    state.player.health += game.rng.random_in_range(7..15);
    game.hour_pass(state)
}

pub(in crate::logic) fn try_to_sleep(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    if state.current_time > Time(3) && state.current_time < Time(20) {
        game.set_screen(GameScreen::Sleep(state));
        wait_for_any_key()
    } else {
        go_to_sleep(game, state)
    }
}

pub(in crate::logic) fn go_to_sleep(
    _game: &mut InternalGameState,
    _state: GameState,
) -> ActionVec {
    todo!()
}

pub(in crate::logic) fn handle_sleeping(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    // TODO: Реализовать что-то помимо неудавшегося сна
    assert_matches!(&*game.screen(), GameScreen::Sleep(_));
    assert_eq!(action, Action::AnyKey);
    legacy::scene_router_run(game, &state)
}

pub(in crate::logic) fn handle_what_to_do(
    game: &mut InternalGameState,
    state: GameState,
    action: HelpAction,
) -> ActionVec {
    use GameScreen::*;
    assert_eq!(state.location(), Location::Dorm);
    game.set_screen(match action {
        HelpAction::WhatToDoAtAll => WhatToDo(state),
        HelpAction::AboutScreen => AboutScreen(state),
        HelpAction::WhereToGoAndWhy => WhereToGoAndWhy(state),
        HelpAction::AboutProfessors => AboutProfessors(state),
        HelpAction::AboutCharacters => AboutCharacters(state),
        HelpAction::AboutThisProgram => AboutThisProgram(state),
        HelpAction::ThanksButNothing => {
            return legacy::scene_router_run(game, &state);
        }
    });
    ActionVec::from([
        Action::Help(HelpAction::WhatToDoAtAll),
        Action::Help(HelpAction::AboutScreen),
        Action::Help(HelpAction::WhereToGoAndWhy),
        Action::Help(HelpAction::AboutProfessors),
        Action::Help(HelpAction::AboutCharacters),
        Action::Help(HelpAction::AboutThisProgram),
        Action::Help(HelpAction::ThanksButNothing),
    ])
}
