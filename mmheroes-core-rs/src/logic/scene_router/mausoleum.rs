use super::*;

pub(super) fn handle_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    match action {
        Action::GoFromMausoleumToPunk => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
                run_sync,
            )
        }
        Action::GoToPDMI => train::go_to_pdmi(game, state),
        Action::GoFromMausoleumToDorm => {
            state.location = Location::Dorm;
            run_sync(game, state)
        }
        Action::Rest => {
            let money = state.player.money;
            game.set_screen(GameScreen::RestInMausoleum(state));
            let mut available_actions = ActionVec::new();
            if money >= Money::cola_cost() {
                available_actions.push(Action::OrderCola);
            }
            if money >= Money::soup_cost() {
                available_actions.push(Action::OrderSoup);
            }
            if money >= Money::beer_cost() {
                available_actions.push(Action::OrderBeer);
            }
            available_actions.push(Action::RestByOurselvesInMausoleum);
            available_actions.push(Action::NoRestIsNoGood);
            available_actions
        }
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::Mausoleum)
            );
            npc::interact_with_classmate(game, state, classmate)
        }
        Action::IAmDone => scene_router::i_am_done(game, state),
        _ => illegal_action!(action),
    }
}

pub(in crate::logic) fn handle_rest(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    let player = &mut state.player;
    match action {
        Action::OrderCola => {
            assert!(player.money >= Money::cola_cost());
            player.money -= Money::cola_cost();
            player.health += game.rng.random(player.charisma.0) + 3;
        }
        Action::OrderSoup => {
            assert!(player.money >= Money::soup_cost());
            player.money -= Money::soup_cost();
            player.health += game.rng.random(player.charisma.0) + 5;
        }
        Action::OrderBeer => {
            assert!(player.money >= Money::beer_cost());
            player.money -= Money::beer_cost();
            if game.rng.roll_dice(3) {
                player.brain -= 1;
            }
            if game.rng.roll_dice(3) {
                player.charisma += 1;
            }
            if game.rng.roll_dice(3) {
                player.stamina += 2;
            }
            player.health += game.rng.random(player.charisma.0);
            if player.brain <= BrainLevel(0) {
                player.health = HealthLevel(0);
                player.cause_of_death = Some(CauseOfDeath::BeerAlcoholism);
                return scene_router::game_end(game, state);
            }
        }
        Action::RestByOurselvesInMausoleum => {
            player.health += game.rng.random(player.charisma.0);
        }
        Action::NoRestIsNoGood => return scene_router::run_sync(game, state),
        _ => illegal_action!(action),
    }

    game.hour_pass(state)
}
