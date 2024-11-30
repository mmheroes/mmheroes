use super::*;

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    g.set_screen_and_wait_for_any_key(GameScreen::DjugInteraction(state.clone()))
        .await;
    let geometry_knowledge = &mut state
        .player
        .status_for_subject_mut(Subject::GeometryAndTopology)
        .knowledge;
    if *geometry_knowledge > 5 {
        *geometry_knowledge -= g.rng.random(5);
    }
    misc::decrease_health(state, HealthLevel(15), CauseOfDeath::DontTalkToDjug);
}
