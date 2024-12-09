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
            g.set_screen_and_wait_for_any_key(GameScreen::HighScores(state.clone()))
                .await;
        }
        Action::GoToCafePDMI => go_to_cafe(g, state).await,
        Action::GoToPUNKFromPDMI => train::go_from_pdmi(g, state).await,
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

async fn go_to_cafe(g: &mut InternalGameState<'_>, state: &mut GameState) {
    cafe::go(
        g,
        state,
        &[
            (Action::OrderCoffee, Money::drink_cost(), 3),
            (Action::OrderPastry, Money::pastry_cost(), 6),
            (
                Action::OrderCoffeeWithPastry,
                Money::drink_with_pastry_cost(),
                10,
            ),
        ],
        Action::RestInCafePDMI,
        Action::LeaveCafePDMI,
    )
    .await
}
