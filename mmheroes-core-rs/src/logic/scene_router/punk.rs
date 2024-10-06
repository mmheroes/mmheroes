use super::*;

pub(in crate::logic) fn handle_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
    match action {
        Action::GoToProfessor => actions::go_to_professor(game, state),
        Action::LookAtBaobab => {
            game.set_screen(GameScreen::HighScores(state));
            wait_for_any_key()
        }
        Action::GoFromPunkToDorm => {
            state.location = Location::Dorm;
            legacy::scene_router_run(game, &state)
        }
        Action::GoToPDMI => train::go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| legacy::scene_router_run(g, state),
            )
        }
        Action::GoToComputerClass => {
            assert!(state.current_time < Time::computer_class_closing());
            state.location = Location::ComputerClass;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::FellFromStairs,
                |g, state| legacy::scene_router_run(g, state),
            )
        }
        Action::GoToCafePUNK => {
            // TODO: Логику можно переиспользовать в кафе ПОМИ
            assert!(state.current_time.is_cafe_open());
            let mut available_actions = ActionVec::new();
            let available_money = state.player.money;
            if available_money >= Money::tea_cost() {
                available_actions.push(Action::OrderTea);
            }
            if available_money >= Money::cake_cost() {
                available_actions.push(Action::OrderCake);
            }
            if available_money >= Money::tea_with_cake_cost() {
                available_actions.push(Action::OrderTeaWithCake);
            }
            available_actions.push(Action::RestInCafePUNK);
            available_actions.push(Action::ShouldntHaveComeToCafePUNK);
            game.set_screen(GameScreen::CafePUNK(state));
            available_actions
        }
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PUNK)
            );
            npc::interact_with_classmate(game, state, classmate)
        }
        Action::GoToWork => {
            assert!(state.player.is_employed_at_terkom());
            todo!()
        }
        Action::IAmDone => legacy::i_am_done(game, state),
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    let available_actions = handle_action(g, state.clone(), action);
    g.set_available_actions_from_vec(available_actions);
}

pub(in crate::logic) fn handle_cafe_punk_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    // TODO: Логику можно переиспользовать в кафе ПОМИ
    assert_eq!(state.location, Location::PUNK);
    assert!(state.current_time.is_cafe_open());
    assert_matches!(&*game.screen(), GameScreen::CafePUNK(_));
    let money = &mut state.player.money;
    let health = &mut state.player.health;
    let charisma_dependent_health_gain =
        HealthLevel(game.rng.random(state.player.charisma.0));
    match action {
        Action::OrderTea => {
            *money -= Money::tea_cost();
            *health += charisma_dependent_health_gain + 2;
        }
        Action::OrderCake => {
            *money -= Money::cake_cost();
            *health += charisma_dependent_health_gain + 4;
        }
        Action::OrderTeaWithCake => {
            *money -= Money::tea_with_cake_cost();
            *health += charisma_dependent_health_gain + 7;
        }
        Action::RestInCafePUNK => {
            *health += charisma_dependent_health_gain;
        }
        Action::ShouldntHaveComeToCafePUNK => {
            return legacy::scene_router_run(game, &state);
        }
        _ => illegal_action!(action),
    }
    game.hour_pass(state)
}
