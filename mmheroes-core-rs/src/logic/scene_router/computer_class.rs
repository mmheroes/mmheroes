use super::*;

pub(super) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    assert_eq!(state.location(), Location::ComputerClass);
    match action {
        Action::Exam(subject) => exams::enter_exam(g, state, subject).await,
        Action::GoFromPunkToDorm => {
            state.set_location(Location::Dorm);
        }
        Action::LeaveComputerClass => {
            state.set_location(Location::PUNK);
            misc::decrease_health(
                state,
                HealthLevel::location_change_small_penalty(),
                CauseOfDeath::CouldntLeaveTheComputer,
            );
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoToMausoleum => {
            state.set_location(Location::Mausoleum);
            misc::decrease_health(
                state,
                HealthLevel::location_change_small_penalty(),
                CauseOfDeath::OnTheWayToMausoleum,
            );
        }
        Action::SurfInternet => surf_internet(g, state).await,
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::ComputerClass)
            );
            interact_with_classmate(g, state, classmate, None).await
        }
        Action::PlayMMHEROES => todo!("Поиграть в MMHEROES"),
        _ => illegal_action!(action),
    }
}

async fn surf_internet(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let player = &state.player;
    let solved_all_problems = player
        .status_for_subject(Subject::ComputerScience)
        .solved_all_problems();
    // В GOD-режиме можно нафармить сколь угодно много решённых задач.
    // Наверное, баг в оригинальной реализации. А может и нет.
    let found_program = player.is_god_mode()
        || (g.rng.random(player.brain) > BrainLevel(6) && !solved_all_problems);
    g.set_screen_and_wait_for_any_key(GameScreen::SurfInternet { found_program })
        .await;
    if found_program {
        state
            .player
            .status_for_subject_mut(Subject::ComputerScience)
            .more_problems_solved(1);
    } else if state.player.brain < BrainLevel(5) && g.rng.roll_dice(3) {
        state.player.brain += 1;
    }
    misc::hour_pass(g, state).await
}
