use super::*;

pub(in crate::logic) async fn handle_router_action(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    action: Action,
) {
    assert_eq!(state.location, Location::Mausoleum);
    match action {
        Action::GoFromMausoleumToPunk => {
            state.location = Location::PUNK;
            misc::decrease_health(
                state,
                HealthLevel::location_change_large_penalty(),
                CauseOfDeath::OnTheWayToPUNK,
            );
        }
        Action::GoToPDMI => train::go_to_pdmi(g, state).await,
        Action::GoFromMausoleumToDorm => {
            state.location = Location::Dorm;
        }
        Action::Rest => rest(g, state).await,
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::Mausoleum)
            );
            interact_with_classmate(g, state, classmate).await
        }
        _ => illegal_action!(action),
    }
}

async fn rest(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let money = state.player.money;
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
    g.set_screen_and_action_vec(
        GameScreen::RestInMausoleum(state.clone()),
        available_actions,
    );
    let player = &mut state.player;
    match g.wait_for_action().await {
        Action::OrderCola => {
            player.money -= Money::cola_cost();
            player.health += g.rng.random(player.charisma.0) + 3;
        }
        Action::OrderSoup => {
            player.money -= Money::soup_cost();
            player.health += g.rng.random(player.charisma.0) + 5;
        }
        Action::OrderBeer => {
            player.money -= Money::beer_cost();
            if g.rng.roll_dice(3) {
                player.brain -= 1;
            }
            if g.rng.roll_dice(3) {
                player.charisma += 1;
            }
            if g.rng.roll_dice(2) {
                player.stamina += 1;
            }
            player.health += g.rng.random(player.charisma.0);
            if player.brain <= BrainLevel(0) {
                player.health = HealthLevel(0);
                player.cause_of_death = Some(CauseOfDeath::BeerAlcoholism);
            }
        }
        Action::RestByOurselvesInMausoleum => {
            player.health += g.rng.random(player.charisma.0);
        }
        Action::NoRestIsNoGood => return,
        action => illegal_action!(action),
    }
    misc::hour_pass(g, state).await
}
