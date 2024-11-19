use crate::logic::actions::{illegal_action, ActionVec};
use crate::logic::{
    misc, Action, CauseOfDeath, GameScreen, GameState, HealthLevel, InternalGameState,
    Money, Player, Time,
};
use Terkom::AgainNoFreeComputers;

#[derive(Debug, Clone, Copy)]
pub enum Terkom {
    SorryNoFreeComputers { hiccup: u8 },
    AgainNoFreeComputers { hiccup: u8 },
    Prompt,
    YouEarnedByWorking { income: Money, hiccup: u8 },
    YouEarnedBySurfingInternet { income: Money, hiccup: u8 },
    MMHEROES { hiccup: u8 },
    Leaving { hiccup: u8 },
    EndOfWorkDay { hiccup: u8 },
}

use Terkom::*;

pub(super) async fn work(g: &mut InternalGameState<'_>, state: &mut GameState) {
    if !state.terkom_has_places() {
        g.set_screen_and_wait_for_any_key(GameScreen::Terkom(AgainNoFreeComputers {
            hiccup: hiccup(state),
        }))
        .await;
        return;
    }

    if g.rng.random(3) > 0 {
        g.set_screen_and_wait_for_any_key(GameScreen::Terkom(SorryNoFreeComputers {
            hiccup: hiccup(state),
        }))
        .await;
        state.set_terkom_has_places(false);
        return;
    }

    loop {
        let mut available_actions = ActionVec::new();
        available_actions.push(Action::EarnAtTerkom);
        if state.player.has_mmheroes_floppy() {
            available_actions.push(Action::PlayMMHEROES)
        }
        if state.player.has_internet() {
            available_actions.push(Action::SurfInternetAtTerkom)
        }
        available_actions.push(Action::ExitTerkom);
        g.set_screen_and_action_vec(GameScreen::Terkom(Prompt), available_actions);
        match g.wait_for_action().await {
            Action::EarnAtTerkom => {
                let income = earned_money(&mut g.rng, state.player());
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    YouEarnedByWorking {
                        income,
                        hiccup: hiccup(state),
                    },
                ))
                .await;
                state.player.money += income;
                misc::decrease_health(
                    state,
                    HealthLevel(income.0 * 2),
                    CauseOfDeath::Burnout,
                );
                misc::hour_pass(g, state).await;
            }
            Action::PlayMMHEROES => {
                // Сперва нужно реализовать взаимодействие с Diamond
                todo!("Поиграть в MMHEROES")
            }
            Action::SurfInternetAtTerkom => {
                let income = earned_money(&mut g.rng, state.player());
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    YouEarnedBySurfingInternet {
                        income,
                        hiccup: hiccup(state),
                    },
                ))
                .await;
                state.player.money += income;
                misc::hour_pass(g, state).await;
            }
            Action::ExitTerkom => {
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(Leaving {
                    hiccup: hiccup(state),
                }))
                .await;
                break;
            }
            action => illegal_action!(action),
        }

        if state.current_time() >= Time::terkom_closing_time() {
            g.set_screen_and_wait_for_any_key(GameScreen::Terkom(EndOfWorkDay {
                hiccup: hiccup(state),
            }))
            .await;
            break;
        }
    }
}

fn earned_money(rng: &mut crate::random::Rng, player: &Player) -> Money {
    let mut income = rng.random(player.brain().0 + player.charisma().0);
    income = rng.random(income);
    income += 1;
    while income > 4 {
        income = rng.random_in_range(2..(income - 1));
    }
    Money(income)
}

/// Икота сотрудника ТЕРКОМа. Чем меньше число, тем чаще он икает.
/// Чем ближе конец дня, тем сильнее икота.
fn hiccup(state: &GameState) -> u8 {
    5 - [10, 14, 16, 18]
        .into_iter()
        .position(|t| t > state.current_time().0)
        .unwrap_or(4) as u8
}
