use crate::logic::actions::{illegal_action, ActionVec};
use crate::logic::{
    misc, Action, GameScreen, GameState, HealthLevel, InternalGameState, Money,
};

pub(super) async fn go(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    menu: &[(Action, Money, HealthLevel)],
    rest_action: Action,
    exit_action: Action,
) {
    let mut available_actions = ActionVec::new();
    let available_money = state.player.money;
    for &(position, cost, _) in menu {
        if available_money >= cost {
            available_actions.push(position);
        }
    }
    available_actions.push(rest_action);
    available_actions.push(exit_action);
    g.set_screen_and_action_vec(GameScreen::Cafe(state.clone()), available_actions);
    let selected_action = g.wait_for_action().await;
    let charisma_dependent_health_gain = g.rng.random(state.player.charisma);
    if let Some(&(_, cost, menu_health_gain)) = menu
        .iter()
        .find(|(action, _, _)| *action == selected_action)
    {
        state.player.money -= cost;
        state.player.health += charisma_dependent_health_gain + menu_health_gain;
    } else if selected_action == rest_action {
        state.player.health += charisma_dependent_health_gain;
    } else if selected_action == exit_action {
        return;
    } else {
        illegal_action!(selected_action);
    }
    misc::hour_pass(g, state, None).await;
}
