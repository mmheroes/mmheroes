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
                LOCATION_CHANGE_SMALL_HEALTH_PENALTY,
                CauseOfDeath::CouldntLeaveTheComputer,
            );
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoToMausoleum => {
            state.set_location(Location::Mausoleum);
            misc::decrease_health(
                state,
                LOCATION_CHANGE_SMALL_HEALTH_PENALTY,
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
        Action::PlayMMHEROES => play_mmheroes(g, state).await,
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
    let found_program =
        player.is_god_mode() || (g.rng.random(player.brain) > 6 && !solved_all_problems);
    g.set_screen_and_wait_for_any_key(GameScreen::SurfInternet { found_program })
        .await;
    if found_program {
        state
            .player
            .status_for_subject_mut(Subject::ComputerScience)
            .more_problems_solved(1);
    } else if state.player.brain < 5 && g.rng.roll_dice(3) {
        state.player.brain += 1;
    }
    misc::hour_pass(g, state, None).await
}

#[derive(Debug, Copy, Clone)]
pub enum PlayMmheroesScene {
    /// "Неожиданно ты осознаешь, что началась зачетная неделя…"
    Ding,

    /// "СТОП! ЧТО-ТО ТАКОЕ ТЫ УЖЕ ВИДЕЛ!!!"
    Wait,

    /// "Не каждый способен пережить такое потрясение…"
    NotEveryoneCanSurviveThis,
}

async fn play_mmheroes(g: &mut InternalGameState<'_>, state: &mut GameState) {
    use PlayMmheroesScene::*;
    state.add_recursion_level();
    g.set_screen_and_wait_for_any_key(GameScreen::PlayMmheroes(Ding))
        .await;
    g.set_screen_and_wait_for_any_key(GameScreen::PlayMmheroes(Wait))
        .await;
    if state.player.stamina + state.player.brain - (state.recursion() as i16 * 5) < 8 {
        misc::decrease_health(state, 100, CauseOfDeath::SplitPersonality);
    }

    misc::hour_pass(g, state, None).await;

    if state.player.health <= 0 {
        return;
    }

    g.set_screen_and_wait_for_any_key(GameScreen::PlayMmheroes(
        NotEveryoneCanSurviveThis,
    ))
    .await;

    state
        .timetable
        .randomize_from_day(state.current_day_index(), &mut g.rng);
}
