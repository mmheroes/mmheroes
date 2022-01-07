use super::*;

pub(super) fn handle_action(
    game: &mut Game,
    state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::PDMI);
    match action {
        Action::GoToProfessor => actions::go_to_professor(game, state),
        Action::LookAtBulletinBoard => {
            game.screen = GameScreen::HighScores(state);
            wait_for_any_key()
        }
        Action::RestInCafePDMI => todo!("Пойти в кафе"),
        Action::GoToPUNKFromPDMI => todo!("Поехать в ПУНК"),
        Action::IAmDone => scene_router::i_am_done(game, state),
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PDMI)
            );
            npc::interact_with_classmate(game, state, classmate)
        }
        _ => illegal_action!(action),
    }
}
