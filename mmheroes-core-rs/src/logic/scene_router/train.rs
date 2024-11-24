use super::*;
use crate::logic::actions::TrainTicketAction;
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
    BoughtRoundtripTicket,
}

#[derive(Debug, Clone)]
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

    let health_penalty = HealthLevel(g.rng.random(10));
    state.set_location(Location::PDMI);
    let caught_by_inspectors =
        go_by_train(g, state, health_penalty, &GameScreen::TrainToPDMI).await;
    misc::hour_pass(g, state).await;
    if caught_by_inspectors {
        misc::hour_pass(g, state).await;
    }
}

pub(super) async fn go_by_train(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    health_penalty: HealthLevel,
    make_screen: &dyn Fn(GameState, TrainScene) -> GameScreen,
) -> bool {
    if state.player.money < Money::roundtrip_train_ticket_cost() {
        let caught_by_inspectors = inspectors(&mut g.rng, state);
        g.set_screen_and_wait_for_any_key(make_screen(
            state.clone(),
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            },
        ))
        .await;
        misc::decrease_health(state, health_penalty, CauseOfDeath::CorpseFoundInTheTrain);
        if caught_by_inspectors {
            misc::decrease_health(
                state,
                HealthLevel(10),
                CauseOfDeath::KilledByInspectors,
            );
        }
        caught_by_inspectors
    } else {
        let caught_by_inspectors = match g
            .set_screen_and_wait_for_action::<TrainTicketAction>(make_screen(
                state.clone(),
                PromptToBuyTickets,
            ))
            .await
        {
            TrainTicketAction::GatecrashTrain => {
                let caught_by_inspectors = inspectors(&mut g.rng, state);
                // Здоровье не уменьшается если поймали контролёры!
                g.set_screen_and_wait_for_any_key(make_screen(
                    state.clone(),
                    GatecrashByChoice {
                        caught_by_inspectors,
                    },
                ))
                .await;
                caught_by_inspectors
            }
            TrainTicketAction::BuyRoundtripTrainTicket => {
                g.set_screen_and_wait_for_any_key(make_screen(
                    state.clone(),
                    BoughtRoundtripTicket,
                ))
                .await;
                state.player.money -= Money::roundtrip_train_ticket_cost();
                state.player.set_has_roundtrip_train_ticket();
                false
            }
        };
        misc::decrease_health(state, health_penalty, CauseOfDeath::CorpseFoundInTheTrain);
        caught_by_inspectors
    }
}

pub(in crate::logic) fn inspectors(rng: &mut Rng, state: &GameState) -> bool {
    state.player.charisma < CharismaLevel(rng.random(10))
}
