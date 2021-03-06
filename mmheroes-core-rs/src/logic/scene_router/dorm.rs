use super::*;

pub(super) fn handle_action(
    game: &mut Game,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    match action {
        Action::Study => choose_subject_to_study(game, state),
        Action::ViewTimetable => game.view_timetable(state),
        Action::Rest => rest(game, state),
        Action::GoToBed => try_to_sleep(game, state),
        Action::GoFromDormToPunk => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
                run,
            )
        }
        Action::GoToPDMI => train::go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                run,
            )
        }
        Action::IAmDone => scene_router::i_am_done(game, state),
        Action::WhatToDo => handle_what_to_do(game, state, Action::WhatToDoAtAll),
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) fn choose_subject_to_study(
    game: &mut Game,
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
    game.screen = GameScreen::Study(state);
    available_actions
}

pub(in crate::logic) fn choose_use_lecture_notes(
    game: &mut Game,
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
                game.screen = GameScreen::PromptUseLectureNotes(state);
                ActionVec::from([
                    Action::UseLectureNotes(subject),
                    Action::DontUseLectureNotes(subject),
                ])
            } else {
                study(game, state, subject, false)
            }
        }
        Action::DontStudy => scene_router::run(game, state),
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) fn study(
    game: &mut Game,
    mut state: GameState,
    subject: Subject,
    use_lecture_notes: bool,
) -> ActionVec {
    // ???????????????????? "???????????????????????? ???????????????? => ?? ???????????? ???????? ????????????????"
    // ???????????? ???????? ????????????????
    assert!(
        !use_lecture_notes
            || state.player.status_for_subject(subject).has_lecture_notes(),
        "???????????? ?????????????????????????????? ????????????????????, ?????? ?????? ?????? ???? ?????????? ???????? ??????"
    );
    let brain_or_stamina = if subject == Subject::PhysicalEducation {
        state.player.stamina.0
    } else {
        state.player.brain.0
    };
    if brain_or_stamina <= 0 {
        return scene_router::run(game, state);
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
                    state,
                    CauseOfDeath::StudiedTooWell,
                    |game, state| game.hour_pass(state),
                )
            } else {
                game.hour_pass(state)
            }
        },
    )
}

fn rest(game: &mut Game, mut state: GameState) -> ActionVec {
    state.player.health += game.rng.random_in_range(7..15);
    game.hour_pass(state)
}

fn try_to_sleep(game: &mut Game, state: GameState) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    if state.current_time > Time(3) && state.current_time < Time(20) {
        game.screen = GameScreen::Sleep(state);
        wait_for_any_key()
    } else {
        go_to_sleep(game, state)
    }
}

pub(in crate::logic) fn go_to_sleep(_game: &mut Game, _state: GameState) -> ActionVec {
    todo!()
}

pub(in crate::logic) fn handle_sleeping(
    game: &mut Game,
    state: GameState,
    action: Action,
) -> ActionVec {
    // TODO: ?????????????????????? ??????-???? ???????????? ???????????????????????? ??????
    assert_matches!(game.screen, GameScreen::Sleep(_));
    assert_eq!(action, Action::AnyKey);
    scene_router::run(game, state)
}

pub(in crate::logic) fn handle_what_to_do(
    game: &mut Game,
    state: GameState,
    action: Action,
) -> ActionVec {
    use GameScreen::*;
    assert_eq!(state.location(), Location::Dorm);
    game.screen = match action {
        Action::WhatToDoAtAll => WhatToDo(state),
        Action::AboutScreen => AboutScreen(state),
        Action::WhereToGoAndWhy => WhereToGoAndWhy(state),
        Action::AboutProfessors => AboutProfessors(state),
        Action::AboutCharacters => AboutCharacters(state),
        Action::AboutThisProgram => AboutThisProgram(state),
        Action::ThanksButNothing => {
            return scene_router::run(game, state);
        }
        _ => illegal_action!(action),
    };
    ActionVec::from([
        Action::WhatToDoAtAll,
        Action::AboutScreen,
        Action::WhereToGoAndWhy,
        Action::AboutProfessors,
        Action::AboutCharacters,
        Action::AboutThisProgram,
        Action::ThanksButNothing,
    ])
}
