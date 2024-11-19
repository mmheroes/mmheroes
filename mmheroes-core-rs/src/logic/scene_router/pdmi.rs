use super::*;

pub(in crate::logic) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    assert_eq!(state.location(), Location::PDMI);
    match action {
        Action::GoToProfessor => exams::go_to_professor(g, state).await,
        Action::LookAtBulletinBoard => {
            g.set_screen_and_wait_for_any_key(GameScreen::HighScores)
                .await;
        }
        Action::RestInCafePDMI => todo!("Пойти в кафе"),
        Action::GoToPUNKFromPDMI => todo!("Поехать в ПУНК"),
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PDMI)
            );
            interact_with_classmate(g, state, classmate, None).await
        }
        _ => illegal_action!(action),
    }
}
