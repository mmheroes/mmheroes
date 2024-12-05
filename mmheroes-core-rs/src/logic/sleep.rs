use crate::logic::{
    timetable, CauseOfDeath, Duration, GameState, HealthLevel, InternalGameState,
    Location, Time,
};
use core::cmp::min;

pub(in crate::logic) async fn sleep(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    assert_eq!(
        state.location(),
        Location::Dorm,
        "Спать можно только в общаге"
    );
    if die_if_time_out(state) {
        return;
    }
    state.player.health = min(state.player.health, HealthLevel(40));
    let health_gain = (state.player.health.0 + g.rng.random_in_range(15..35)).min(50)
        - state.player.health.0;
    assert!(health_gain >= 0, "negative health_gain ({})", health_gain,);
    state.player.health += health_gain;
    let sleep_duration = 7 + g.rng.random(health_gain / 4);
    state.adjust_time(Duration(sleep_duration as i8));

    if state.current_time() >= Time(24) {
        state.set_current_time(state.current_time() % 24);
        state.next_day();
        die_if_time_out(state);
    }

    dreams(g, state).await;

    state.set_current_time(state.current_time().max(Time(5)));

    if state.player.garlic > 0 {
        state.player.garlic -= 1;
        state.player.charisma += g.rng.random(2);
    }
}

fn die_if_time_out(state: &mut GameState) -> bool {
    let last_day_index = timetable::NUM_DAYS as u8;
    assert!(state.current_day_index() <= last_day_index);
    let time_out = state.current_day_index() == last_day_index;
    if time_out {
        state.player.cause_of_death = Some(CauseOfDeath::TimeOut);
    };
    time_out
}

async fn dreams(g: &mut InternalGameState<'_>, state: &mut GameState) {
    // TODO
}
