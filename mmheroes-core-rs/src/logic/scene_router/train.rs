use super::*;
use crate::random::Rng;
use TrainToPDMI::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrainToPDMI {
    /// "Здравый смысл подсказывает тебе, что в такое время ты там никого уже не найдешь.
    /// Не будем зря тратить здоровье на поездку в ПОМИ."
    NoPointToGoToPDMI,

    /// Денег у тебя нет, пришлось ехать зайцем...
    GatecrashBecauseNoMoney { caught_by_inspectors: bool },

    /// "Ехать зайцем" или "Честно заплатить 10 руб. за билет в оба конца"
    PromptToBuyTickets,

    /// Выбрали ехать зайцем, даже несмотря на то что деньги на билет были.
    GatecrashByChoice { caught_by_inspectors: bool },

    /// Купили билет
    BoughtRoundtripTicket,
}

// FIXME: Здесь всё неправильно, нужно переделать.
pub(super) async fn go_to_pdmi(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) -> RouterResult {
    assert_ne!(state.location, Location::PDMI);
    if state.current_time > Time(20) {
        g.set_screen_and_wait_for_any_key(GameScreen::TrainToPDMI(
            state.clone(),
            NoPointToGoToPDMI,
        ))
        .await;
        return Ok(());
    }
    let health_penalty = HealthLevel(g.rng.random(10));
    // FIXME: Игра должна закончится после wait_for_any_key, а не сразу!!!
    misc::decrease_health(
        g,
        health_penalty,
        state,
        CauseOfDeath::CorpseFoundInTheTrain,
    )
    .await?;
    state.location = Location::PDMI;
    if state.player.money < Money::roundtrip_train_ticket_cost() {
        no_money(g, state).await
    } else {
        g.set_screen_and_available_actions(
            GameScreen::TrainToPDMI(state.clone(), PromptToBuyTickets),
            [Action::GatecrashTrain, Action::BuyRoundtripTrainTicket],
        );
        match g.wait_for_action().await {
            Action::GatecrashTrain => {
                let caught_by_inspectors = inspectors(&mut g.rng, state);
                g.set_screen_and_wait_for_any_key(GameScreen::TrainToPDMI(
                    state.clone(),
                    GatecrashByChoice {
                        caught_by_inspectors,
                    },
                ))
                .await;
                if caught_by_inspectors {
                    todo!("Если поймали контролёры, должно пройти два часа!")
                }
                misc::hour_pass(g, state).await
            }
            Action::BuyRoundtripTrainTicket => {
                g.set_screen_and_wait_for_any_key(GameScreen::TrainToPDMI(
                    state.clone(),
                    BoughtRoundtripTicket,
                ))
                .await;
                state.player.money -= Money::roundtrip_train_ticket_cost();
                state.player.set_has_roundtrip_train_ticket();
                misc::hour_pass(g, state).await
            }
            action => illegal_action!(action),
        }
    }
}

pub(in crate::logic) fn inspectors(rng: &mut Rng, state: &GameState) -> bool {
    state.player.charisma < CharismaLevel(rng.random(10))
}

async fn no_money(g: &mut InternalGameState<'_>, state: &mut GameState) -> RouterResult {
    let caught_by_inspectors = inspectors(&mut g.rng, state);
    let health_penalty = HealthLevel(10);
    if caught_by_inspectors {
        // FIXME: Игра должна закончится после wait_for_any_key, а не сразу!!!
        misc::decrease_health(g, health_penalty, state, CauseOfDeath::KilledByInspectors)
            .await?;
    }
    g.set_screen_and_wait_for_any_key(GameScreen::TrainToPDMI(
        state.clone(),
        GatecrashBecauseNoMoney {
            caught_by_inspectors,
        },
    ))
    .await;
    // TODO: Должен пройти час!
    Ok(())
}
