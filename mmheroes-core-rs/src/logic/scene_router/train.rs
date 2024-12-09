use super::*;
use crate::random::Rng;
use TrainScene::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrainScene {
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
    BoughtRoundTripTicket,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum BaltiyskiyRailwayStationScene {
    /// "Ты в Питере, на Балтийском вокзале. Куда направляемся?"
    Prompt(GameState),

    /// "Тебя заловили контролеры!"
    CaughtByInspectors,
}

pub(super) async fn go_to_pdmi(g: &mut InternalGameState<'_>, state: &mut GameState) {
    assert_ne!(state.location(), Location::PDMI);
    if state.current_time() > Time(20) {
        g.set_screen_and_wait_for_any_key(GameScreen::TrainToPDMI(
            state.clone(),
            NoPointToGoToPDMI,
        ))
        .await;
        return;
    }

    let health_penalty = g.rng.random(10);
    state.set_location(Location::PDMI);
    let caught_by_inspectors =
        go_by_train(g, state, health_penalty, false, &GameScreen::TrainToPDMI).await;
    misc::hour_pass(g, state, None).await;
    if caught_by_inspectors {
        misc::hour_pass(g, state, None).await;
    }
}

pub(super) async fn go_from_pdmi(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let health_penalty = g.rng.random(10);
    go_by_train(g, state, health_penalty, true, &GameScreen::TrainFromPDMI).await;

    state.set_location(Location::PUNK);

    // Баг в оригинальной реализации: безбилетная поездка из ПОМИ в ПУНК
    // занимает два часа даже если не поймали контролёры.
    if !state.player.has_train_ticket() {
        misc::hour_pass(g, state, None).await;
    }

    misc::hour_pass(g, state, None).await;
    state.player.set_has_train_ticket(false);
}

/// Возвращаемое значение — поймали ли контролёры.
pub(super) async fn go_by_train(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    health_penalty: HealthLevel,
    back_from_pdmi: bool,
    make_screen: &dyn Fn(GameState, TrainScene) -> GameScreen,
) -> bool {
    let ticket_cost = if back_from_pdmi {
        Money::one_way_train_ticket_cost()
    } else {
        Money::roundtrip_train_ticket_cost()
    };
    let no_money_for_ticket = state.player.money < ticket_cost;
    let caught_by_inspectors = if state.player.has_train_ticket() {
        false
    } else if no_money_for_ticket {
        let caught_by_inspectors = inspectors(&mut g.rng, state);
        g.set_screen_and_wait_for_any_key(make_screen(
            state.clone(),
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            },
        ))
        .await;
        caught_by_inspectors
    } else {
        let available_actions = if back_from_pdmi {
            [
                Action::TrainFromPDMIBuyTicket,
                Action::TrainFromPDMIGatecrash,
            ]
        } else {
            [Action::TrainToPDMIGatecrash, Action::TrainToPDMIBuyTicket]
        };
        match g
            .set_screen_and_wait_for_action_vec(
                make_screen(state.clone(), PromptToBuyTickets),
                available_actions,
            )
            .await
        {
            Action::TrainToPDMIGatecrash | Action::TrainFromPDMIGatecrash => {
                let caught_by_inspectors = inspectors(&mut g.rng, state);
                g.set_screen_and_wait_for_any_key(make_screen(
                    state.clone(),
                    GatecrashByChoice {
                        caught_by_inspectors,
                    },
                ))
                .await;
                caught_by_inspectors
            }
            Action::TrainToPDMIBuyTicket | Action::TrainFromPDMIBuyTicket => {
                if !back_from_pdmi {
                    g.set_screen_and_wait_for_any_key(make_screen(
                        state.clone(),
                        BoughtRoundTripTicket,
                    ))
                    .await;
                }
                state.player.money -= ticket_cost;
                state.player.set_has_train_ticket(true);
                false
            }
            action => illegal_action!(action),
        }
    };
    misc::decrease_health(state, health_penalty, CauseOfDeath::CorpseFoundInTheTrain);
    if caught_by_inspectors && (no_money_for_ticket || back_from_pdmi) {
        // Баг в оригинальной реализации:
        // здоровье не уменьшается, если контролёры поймали на пути в ПОМИ и при этом
        // были деньги на билет.
        misc::decrease_health(state, 10, CauseOfDeath::KilledByInspectors);
    }
    caught_by_inspectors
}

pub(in crate::logic) fn inspectors(rng: &mut Rng, state: &GameState) -> bool {
    state.player.charisma < rng.random(10)
}
