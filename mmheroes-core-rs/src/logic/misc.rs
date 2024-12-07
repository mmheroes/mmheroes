use super::*;
use crate::logic::entry_point::GameEnd;

pub(in crate::logic) fn decrease_health(
    state: &mut GameState,
    delta: HealthLevel,
    cause_of_death: CauseOfDeath,
) {
    state.player.health -= delta;
    if state.player.health <= 0 {
        state.player.cause_of_death = Some(cause_of_death);
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
    exam_in_progress: Option<Subject>,
) {
    state.set_terkom_has_places(true);
    g.run_classmate_routines(state);
    state.adjust_time(Duration(1));

    if state.location() == Location::PDMI
        && matches!(exam_in_progress, Some(Subject::GeometryAndTopology))
    {
        decrease_health(state, HealthLevel(6), CauseOfDeath::DjugIsDeadly);
        state.player.set_knows_djug(true);
    }

    // TODO: Написать на это тест
    if state.player.charisma <= CharismaLevel(0) {
        state.player.health = HealthLevel(0);
        state.player.cause_of_death = Some(CauseOfDeath::Paranoia)
    }

    if state.current_time() == Time(24) {
        state.midnight();
        state.next_day();
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
        Location::Dorm => sleep::sleep(g, state).await,
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
        .set_screen_and_wait_for_action(GameScreen::WannaTryAgain)
        .await
    {
        actions::TryAgainAction::WantToTryAgain => GameEnd::Restart,
        actions::TryAgainAction::DontWantToTryAgain => {
            g.set_screen_and_wait_for_any_key(GameScreen::Disclaimer)
                .await;
            g.set_screen_and_action_vec(GameScreen::Terminal, ActionVec::new());
            GameEnd::Exit
        }
    }
}
