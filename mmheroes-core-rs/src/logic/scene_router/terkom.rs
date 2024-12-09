use crate::logic::actions::{illegal_action, ActionVec};
use crate::logic::{
    misc, Action, CauseOfDeath, GameScreen, GameState, InternalGameState, Money, Player,
    Time,
};
use Terkom::AgainNoFreeComputers;

#[derive(Debug, Clone, Copy)]
pub enum Terkom {
    SorryNoFreeComputers { hiccup: u8 },
    AgainNoFreeComputers { hiccup: u8 },
    Prompt,
    YouEarnedByWorking { income: Money, hiccup: u8 },
    YouEarnedBySurfingInternet { income: Money, hiccup: u8 },
    MmheroesPhase1 { hiccup: u8 },
    MmheroesPhase2 { hiccup: u8 },
    Leaving { hiccup: u8 },
    EndOfWorkDay { hiccup: u8 },
}

use Terkom::*;

pub(super) async fn work(g: &mut InternalGameState<'_>, state: &mut GameState) {
    if !state.terkom_has_places() {
        g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
            state.clone(),
            AgainNoFreeComputers {
                hiccup: hiccup(state),
            },
        ))
        .await;
        return;
    }

    if g.rng.random(3) > 0 {
        g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
            state.clone(),
            SorryNoFreeComputers {
                hiccup: hiccup(state),
            },
        ))
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
        g.set_screen_and_action_vec(
            GameScreen::Terkom(state.clone(), Prompt),
            available_actions,
        );
        let hiccup = hiccup(state);
        match g.wait_for_action().await {
            Action::EarnAtTerkom => {
                let income = earned_money(&mut g.rng, state.player());
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    state.clone(),
                    YouEarnedByWorking { income, hiccup },
                ))
                .await;
                state.player.money += income;
                misc::decrease_health(state, income.0 * 2, CauseOfDeath::Burnout);
                misc::hour_pass(g, state, None).await;
            }
            Action::PlayMMHEROES => {
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    state.clone(),
                    MmheroesPhase1 { hiccup },
                ))
                .await;
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    state.clone(),
                    MmheroesPhase2 { hiccup },
                ))
                .await;
            }
            Action::SurfInternetAtTerkom => {
                let income = earned_money(&mut g.rng, state.player());
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    state.clone(),
                    YouEarnedBySurfingInternet { income, hiccup },
                ))
                .await;
                state.player.money += income;
                misc::hour_pass(g, state, None).await;
            }
            Action::ExitTerkom => {
                g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                    state.clone(),
                    Leaving { hiccup },
                ))
                .await;
                break;
            }
            action => illegal_action!(action),
        }

        if state.current_time() >= Time::terkom_closing_time() {
            g.set_screen_and_wait_for_any_key(GameScreen::Terkom(
                state.clone(),
                EndOfWorkDay { hiccup },
            ))
            .await;
            break;
        }
    }
}

fn earned_money(rng: &mut crate::random::Rng, player: &Player) -> Money {
    let mut income = rng.random(player.brain() + player.charisma());
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
