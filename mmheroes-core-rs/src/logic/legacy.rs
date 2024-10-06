//! # Устаревшие функции
//!
//! Пока мы в процессе переписывания игры в стиле async/await нам всё ещё нужны эти
//! функции. Когда вся игра будет переписана, их можно будет удалить.
//!

#![allow(deprecated)]

use crate::logic::actions::{wait_for_any_key, ActionVec, HelpAction};
use crate::logic::*;

#[deprecated]
pub(in crate::logic) fn start_game(g: &mut InternalGameState) -> ActionVec {
    if entry_point::should_select_game_style(g) {
        g.observable_state.borrow().available_actions.clone()
    } else {
        ding(
            g,
            Action::SelectPlayStyle(actions::PlayStyle::RandomStudent),
        )
    }
}

#[deprecated]
pub(in crate::logic) fn ding(g: &mut InternalGameState, action: Action) -> ActionVec {
    let play_style = match action {
        Action::SelectPlayStyle(play_style) => play_style,
        _ => illegal_action!(action),
    };
    let player = g.initialize_player(play_style);
    g.set_screen(GameScreen::Ding(player));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn view_timetable(
    g: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    g.set_screen(GameScreen::Timetable(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn scene_router_run(
    game: &mut InternalGameState,
    state: &GameState,
) -> ActionVec {
    let available_actions = scene_router::available_actions(state);
    game.set_screen(GameScreen::SceneRouter(state.clone()));
    available_actions
}

#[deprecated]
pub(in crate::logic) fn handle_action_sync(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    use scene_router::*;
    use Location::*;
    match state.location() {
        PUNK => punk::handle_action(game, state, action),
        PDMI => pdmi::handle_action(game, state, action),
        ComputerClass => computer_class::handle_action(game, state, action),
        Dorm => handle_dorm_action(game, state, action),
        Mausoleum => mausoleum::handle_action(game, state, action),
    }
}

#[deprecated]
pub(in crate::logic) fn game_end(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    game.set_screen(GameScreen::GameEnd(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn wanna_try_again(game: &mut InternalGameState) -> ActionVec {
    game.set_screen(GameScreen::WannaTryAgain);
    // Хочешь попробовать снова? Да или нет.
    ActionVec::from([Action::WantToTryAgain, Action::DontWantToTryAgain])
}

#[deprecated]
pub(in crate::logic) fn handle_wanna_try_again(
    game: &mut InternalGameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::WantToTryAgain => start_game(game),
        Action::DontWantToTryAgain => {
            game.set_screen(GameScreen::Disclaimer);
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_dorm_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    match action {
        Action::Study => {
            game.set_screen(GameScreen::Study(state.clone()));
            scene_router::dorm::subjects_to_study(&state)
        }
        Action::ViewTimetable => view_timetable(game, state),
        Action::Rest => rest(game, state),
        Action::GoToBed => try_to_sleep(game, state),
        Action::GoFromDormToPunk => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToPDMI => scene_router::train::go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::WhatToDo => handle_what_to_do(game, state, HelpAction::WhatToDoAtAll),
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_what_to_do(
    game: &mut InternalGameState,
    state: GameState,
    action: HelpAction,
) -> ActionVec {
    use crate::logic::GameScreen::*;
    assert_eq!(state.location(), Location::Dorm);
    game.set_screen(match action {
        HelpAction::WhatToDoAtAll => WhatToDo(state),
        HelpAction::AboutScreen => AboutScreen(state),
        HelpAction::WhereToGoAndWhy => WhereToGoAndWhy(state),
        HelpAction::AboutProfessors => AboutProfessors(state),
        HelpAction::AboutCharacters => AboutCharacters(state),
        HelpAction::AboutThisProgram => AboutThisProgram(state),
        HelpAction::ThanksButNothing => {
            return scene_router_run(game, &state);
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

#[deprecated]
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
        return scene_router_run(game, &state);
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
        Action::DontStudy => scene_router_run(game, &state),
        _ => illegal_action!(action),
    }
}

#[deprecated]
fn rest(game: &mut InternalGameState, mut state: GameState) -> ActionVec {
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
    scene_router_run(game, &state)
}
