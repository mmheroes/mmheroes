use crate::logic::actions::illegal_action;
use crate::logic::entry_point::GameEnd;
use crate::logic::scene_router::RouterResult;
use crate::logic::{
    scene_router, Action, CauseOfDeath, CharismaLevel, Duration, GameScreen, GameState,
    HealthLevel, InternalGameState, Location, Time,
};

pub(in crate::logic) async fn decrease_health(
    g: &mut InternalGameState<'_>,
    delta: HealthLevel,
    state: &mut GameState,
    cause_of_death: CauseOfDeath,
) -> RouterResult {
    if state.player.health <= delta {
        state.player.cause_of_death = Some(cause_of_death);
        Err(game_end(g, state).await)
    } else {
        state.player.health -= delta;
        Ok(())
    }
}

pub(in crate::logic) async fn hour_pass(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> RouterResult {
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
        midnight(g, state).await
    } else {
        Ok(())
    }
}

pub(in crate::logic) async fn midnight(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> RouterResult {
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
