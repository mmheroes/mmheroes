use super::*;

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

use crate::random::Rng;
use TrainToPDMI::*;

pub(super) fn go_to_pdmi(game: &mut InternalGameState, state: GameState) -> ActionVec {
    assert_ne!(state.location, Location::PDMI);
    if state.current_time > Time(20) {
        game.set_screen(GameScreen::TrainToPDMI(state, NoPointToGoToPDMI));
        return wait_for_any_key();
    }

    let health_penalty = HealthLevel(game.rng.random(10));
    game.decrease_health(
        health_penalty,
        state,
        CauseOfDeath::CorpseFoundInTheTrain,
        |game, mut state| {
            state.location = Location::PDMI;
            if state.player.money < Money::roundtrip_train_ticket_cost() {
                no_money(game, state)
            } else {
                game.set_screen(GameScreen::TrainToPDMI(state, PromptToBuyTickets));
                ActionVec::from([Action::GatecrashTrain, Action::BuyRoundtripTrainTicket])
            }
        },
    )
}

fn inspectors(rng: &mut Rng, state: &GameState) -> bool {
    state.player.charisma < CharismaLevel(rng.random(10))
}

fn no_money(game: &mut InternalGameState, mut state: GameState) -> ActionVec {
    let caught_by_inspectors = inspectors(&mut game.rng, &state);

    let gatecrash_because_no_money = |game: &mut InternalGameState, state: GameState| {
        game.set_screen(GameScreen::TrainToPDMI(
            state,
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            },
        ));
        wait_for_any_key()
    };

    let health_penalty = HealthLevel(10);
    if caught_by_inspectors {
        if state.location != Location::Dorm {
            return game.decrease_health(
                health_penalty,
                state,
                CauseOfDeath::KilledByInspectors,
                gatecrash_because_no_money,
            );
        }
        // При попытке поехать в ПОМИ из общежития здоровье уменьшается, но смерть
        // не наступает, даже если здоровье стало отрицательным.
        // Баг в оригинальной реализации. Возможно, стоит исправить, но пока не буду.
        state.player.health -= health_penalty;
    }

    gatecrash_because_no_money(game, state)
}

pub(in crate::logic) fn proceed_with_train(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: TrainToPDMI,
) -> ActionVec {
    match action {
        Action::AnyKey => match interaction {
            NoPointToGoToPDMI => scene_router::run_sync(game, state),
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            }
            | GatecrashByChoice {
                caught_by_inspectors,
            } => {
                if caught_by_inspectors {
                    todo!("Если поймали контролёры, должно пройти два часа!")
                }
                game.hour_pass(state)
            }
            PromptToBuyTickets => illegal_action!(action),
            BoughtRoundtripTicket => {
                state.player.money -= Money::roundtrip_train_ticket_cost();
                state.player.set_has_roundtrip_train_ticket();
                game.hour_pass(state)
            }
        },
        Action::GatecrashTrain => {
            assert_eq!(interaction, PromptToBuyTickets);
            let caught_by_inspectors = inspectors(&mut game.rng, &state);
            game.set_screen(GameScreen::TrainToPDMI(
                state,
                GatecrashByChoice {
                    caught_by_inspectors,
                },
            ));
            wait_for_any_key()
        }
        Action::BuyRoundtripTrainTicket => {
            assert_eq!(interaction, PromptToBuyTickets);
            game.set_screen(GameScreen::TrainToPDMI(state, BoughtRoundtripTicket));
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
