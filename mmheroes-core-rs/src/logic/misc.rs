use crate::logic::actions::{ActionVec, TryAgainAction};
use crate::logic::entry_point::GameEnd;
use crate::logic::{
    scene_router, BrainLevel, CauseOfDeath, CharismaLevel, GameScreen, GameState,
    HealthLevel, InternalGameState, Location,
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
    state.set_terkom_has_places(true);
    g.run_classmate_routines(state);
    state.next_hour();

    // TODO: Если сдаём экзамен по геометрии в ПОМИ, DJuG уменьшает здоровье.

    // TODO: Написать на это тест
    if state.player.charisma <= CharismaLevel(0) {
        state.player.health = HealthLevel(0);
        state.player.cause_of_death = Some(CauseOfDeath::Paranoia)
    }

    if state.current_time().is_midnight() {
        state.next_day();
        state.midnight();
        midnight(g, state).await;
    }
}

pub(in crate::logic) async fn midnight(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    match state.location() {
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
    g.set_screen_and_wait_for_any_key(GameScreen::GameEnd(state.clone()))
        .await;
    // Хочешь попробовать снова? Да или нет.
    match g
        .set_screen_and_wait_for_action::<TryAgainAction>(GameScreen::WannaTryAgain)
        .await
    {
        TryAgainAction::WantToTryAgain => GameEnd::Restart,
        TryAgainAction::DontWantToTryAgain => {
            g.set_screen_and_wait_for_any_key(GameScreen::Disclaimer)
                .await;
            g.set_screen_and_action_vec(GameScreen::Terminal, ActionVec::new());
            GameEnd::Exit
        }
    }
}
