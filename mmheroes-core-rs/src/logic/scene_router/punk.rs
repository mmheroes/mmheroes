use super::*;

pub(super) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) -> RouterResult {
    assert_eq!(state.location, Location::PUNK);
    match action {
        Action::GoToProfessor => exams::go_to_professor(g, state).await,
        Action::LookAtBaobab => {
            g.set_screen_and_wait_for_any_key(GameScreen::HighScores(state.clone()))
                .await;
            Ok(())
        }
        Action::GoFromPunkToDorm => {
            state.location = Location::Dorm;
            Ok(())
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            misc::decrease_health(
                g,
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
            )
            .await
        }
        Action::GoToComputerClass => {
            assert!(state.current_time < Time::computer_class_closing());
            state.location = Location::ComputerClass;
            misc::decrease_health(
                g,
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::FellFromStairs,
            )
            .await
        }
        Action::GoToCafePUNK => {
            assert!(state.current_time.is_cafe_open());
            go_to_cafe(g, state).await
        }
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PUNK)
            );
            npc::interact_with_classmate(g, state, classmate).await
        }
        Action::GoToWork => {
            assert!(state.player.is_employed_at_terkom());
            todo!("Пойти в ТЕРКОМ, поработать")
        }
        _ => illegal_action!(action),
    }
}

async fn go_to_cafe(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> RouterResult {
    cafe::go(
        g,
        state,
        &[
            (Action::OrderTea, Money::tea_cost(), HealthLevel(2)),
            (Action::OrderCake, Money::cake_cost(), HealthLevel(4)),
            (
                Action::OrderTeaWithCake,
                Money::tea_with_cake_cost(),
                HealthLevel(7),
            ),
        ],
        Action::RestInCafePUNK,
        Action::ShouldntHaveComeToCafePUNK,
        GameScreen::CafePUNK,
    )
    .await
}
