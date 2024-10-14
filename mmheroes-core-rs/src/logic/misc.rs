use crate::logic::actions::illegal_action;
use crate::logic::entry_point::GameEnd;
use crate::logic::{
    scene_router, Action, BrainLevel, CauseOfDeath, CharismaLevel, Duration, GameScreen,
    GameState, HealthLevel, InternalGameState, Location, Time,
};

pub(in crate::logic) fn decrease_health(
    state: &mut GameState,
    delta: HealthLevel,
    cause_of_death: CauseOfDeath,
) {
    if state.player.health <= delta {
        state.player.cause_of_death = Some(cause_of_death);
    } else {
        state.player.health -= delta;
    }
}

pub(in crate::logic) fn decrease_brain(
    state: &mut GameState,
    delta: BrainLevel,
    cause_of_death: CauseOfDeath,
) {
    state.player.brain -= delta;
    if state.player.brain <= BrainLevel(0) {
        state.player.health = HealthLevel(0);
        state.player.cause_of_death = Some(cause_of_death);
    }
}

pub(in crate::logic) async fn hour_pass(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    // TODO: Lot of stuff going on here

    // TODO: Поменять эти строки местами и не забыть отредактировать метод
    // Time::is_between_9_and_19()!
    g.run_classmate_routines(state);
    state.current_time += Duration(1);

    if state.player.charisma <= CharismaLevel(0) {
        state.player.health = HealthLevel(0);
    }

    if state.current_time.is_midnight() {
        state.current_day_index += 1;
        state.current_time = Time(0);
        midnight(g, state).await;
    }
}

pub(in crate::logic) async fn midnight(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    match state.location {
        Location::PUNK => todo!("sub_1E907"),
        Location::PDMI => todo!("sub_1E7F8"),
        Location::ComputerClass => {
            unreachable!("Компьютерный класс уже должен быть закрыт в полночь!")
        }
        Location::Dorm => scene_router::dorm::sleep(g, state).await,
        Location::Mausoleum => todo!("sub_1E993"),
    }
}

pub(in crate::logic) async fn game_end(
    g: &mut InternalGameState<'_>,
    state: &GameState,
) -> GameEnd {
    g.set_screen(GameScreen::GameEnd(state.clone()));
    g.wait_for_any_key().await;
    // Хочешь попробовать снова? Да или нет.
    g.set_screen_and_available_actions(
        GameScreen::WannaTryAgain,
        [Action::WantToTryAgain, Action::DontWantToTryAgain],
    );
    match g.wait_for_action().await {
        Action::WantToTryAgain => GameEnd::Restart,
        Action::DontWantToTryAgain => {
            g.set_screen_and_wait_for_any_key(GameScreen::Disclaimer)
                .await;
            g.set_screen_and_available_actions(GameScreen::Terminal, []);
            GameEnd::Exit
        }
        action => illegal_action!(action),
    }
}
