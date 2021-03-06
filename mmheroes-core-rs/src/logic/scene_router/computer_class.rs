use super::*;

pub(super) fn handle_action(
    game: &mut Game,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::ComputerClass);
    match action {
        Action::Exam(Subject::ComputerScience) => todo!(),
        Action::GoFromPunkToDorm => {
            state.location = Location::Dorm;
            run(game, state)
        }
        Action::LeaveComputerClass => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::CouldntLeaveTheComputer,
                run,
            )
        }
        Action::GoToPDMI => train::go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                run,
            )
        }
        Action::SurfInternet => surf_internet(game, state),
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::ComputerClass)
            );
            npc::interact_with_classmate(game, state, classmate)
        }
        Action::PlayMMHEROES => todo!(),
        Action::IAmDone => scene_router::game_end(game, state),
        _ => illegal_action!(action),
    }
}

fn surf_internet(game: &mut Game, state: GameState) -> ActionVec {
    let player = &state.player;
    let cs_problems_done = player
        .status_for_subject(Subject::ComputerScience)
        .problems_done();
    let cs_problems_required = SUBJECTS[Subject::ComputerScience].required_problems;
    let found_program = player.is_god_mode()
        || (game.rng.random(player.brain) > BrainLevel(6)
            && cs_problems_done < cs_problems_required);
    game.screen = GameScreen::SurfInternet(state, found_program);
    wait_for_any_key()
}

pub(in crate::logic) fn proceed_with_internet(
    game: &mut Game,
    mut state: GameState,
    action: Action,
    found_program: bool,
) -> ActionVec {
    assert_eq!(action, Action::AnyKey);
    if found_program {
        state
            .player
            .status_for_subject_mut(Subject::ComputerScience)
            .more_problems_solved(1);
    } else if state.player.brain < BrainLevel(5) && game.rng.roll_dice(3) {
        state.player.brain += 1;
    }
    game.hour_pass(state)
}
