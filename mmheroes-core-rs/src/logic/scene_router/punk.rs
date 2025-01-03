use super::*;

pub(super) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    assert_eq!(state.location(), Location::PUNK);
    match action {
        Action::GoToProfessor => exams::go_to_professor(g, state).await,
        Action::LookAtBaobab => {
            g.set_screen_and_wait_for_any_key(GameScreen::HighScores(state.clone()))
                .await;
        }
        Action::GoFromPunkToDorm => {
            state.set_location(Location::Dorm);
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoToMausoleum => {
            state.set_location(Location::Mausoleum);
            misc::decrease_health(
                state,
                LOCATION_CHANGE_LARGE_HEALTH_PENALTY,
                CauseOfDeath::OnTheWayToMausoleum,
            )
        }
        Action::GoToComputerClass => {
            assert!(state.current_time() < Time::computer_class_closing());
            state.set_location(Location::ComputerClass);
            misc::decrease_health(
                state,
                LOCATION_CHANGE_SMALL_HEALTH_PENALTY,
                CauseOfDeath::FellFromStairs,
            )
        }
        Action::GoToCafePUNK => {
            assert!(state.current_time().is_cafe_open());
            go_to_cafe(g, state).await
        }
        Action::InteractWithClassmate(classmate) => {
            assert_eq!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PUNK)
            );
            interact_with_classmate(g, state, classmate, None).await
        }
        Action::GoToWork => {
            assert!(state.player.is_employed_at_terkom());
            terkom::work(g, state).await;
        }
        _ => illegal_action!(action),
    }
}

async fn go_to_cafe(g: &mut InternalGameState<'_>, state: &mut GameState) {
    cafe::go(
        g,
        state,
        &[
            (Action::OrderTea, Money::drink_cost(), 2),
            (Action::OrderCake, Money::pastry_cost(), 4),
            (Action::OrderTeaWithCake, Money::drink_with_pastry_cost(), 7),
        ],
        Action::RestInCafePUNK,
        Action::ShouldntHaveComeToCafePUNK,
    )
    .await
}
