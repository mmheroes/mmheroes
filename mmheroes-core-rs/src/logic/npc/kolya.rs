use crate::logic::actions::YesOrNoAction;
use crate::logic::Subject::AlgebraAndNumberTheory;
use crate::logic::{
    misc, BrainLevel, CauseOfDeath, CharismaLevel, GameScreen, GameState,
    InternalGameState, Location, Money, Player,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KolyaInteraction {
    /// "Коля решил тебе ещё 2 задачи по алгебре!"
    /// (не пришлось заказывать настойку овса)
    SolvedAlgebraProblemsForFree,

    /// "Заказать Коле настойку овса?"
    /// (да или нет)
    PromptOatTincture,

    /// "Коля решил тебе ещё 2 задачи по алгебре!"
    /// (пришлось заказать настойку овса для этого)
    SolvedAlgebraProblemsForOatTincture,

    /// "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
    /// (так как нет денег на настойку овса)
    BrakeFluidNoMoney,

    /// "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
    /// (отказался заказывать настойку овса)
    BrakeFluidBecauseRefused,

    /// "Твой альтруизм навсегда останется в памяти потомков."
    /// (заказал Коле настойку овса, но решать задачи он не стал)
    Altruism,
}

use KolyaInteraction::*;

fn can_solve_algebra_problems(rng: &mut crate::random::Rng, player: &mut Player) -> bool {
    if player.charisma <= rng.random(CharismaLevel(10)) {
        return false;
    }
    player
        .status_for_subject(AlgebraAndNumberTheory)
        .problems_remaining()
        >= 2
}

async fn solve_algebra_problems(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    interaction: KolyaInteraction,
) {
    g.set_screen_and_wait_for_any_key(GameScreen::KolyaInteraction(
        state.clone(),
        interaction,
    ))
    .await;
    state
        .player
        .status_for_subject_mut(AlgebraAndNumberTheory)
        .more_problems_solved(2);
    misc::hour_pass(g, state, None).await
}

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    assert_eq!(state.location(), Location::Mausoleum);
    if can_solve_algebra_problems(&mut g.rng, &mut state.player) {
        return solve_algebra_problems(g, state, SolvedAlgebraProblemsForFree).await;
    }

    if state.player.money < Money::oat_tincture_cost() {
        // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        g.set_screen_and_wait_for_any_key(GameScreen::KolyaInteraction(
            state.clone(),
            BrakeFluidNoMoney,
        ))
        .await;
        misc::decrease_brain(state, BrainLevel(1), CauseOfDeath::DrankTooMuch);
    } else {
        // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
        // "Заказать Коле настойку овса?"
        match g
            .set_screen_and_wait_for_action::<YesOrNoAction>(
                GameScreen::KolyaInteraction(state.clone(), PromptOatTincture),
            )
            .await
        {
            YesOrNoAction::Yes => {
                if can_solve_algebra_problems(&mut g.rng, &mut state.player) {
                    solve_algebra_problems(g, state, SolvedAlgebraProblemsForOatTincture)
                        .await;
                } else {
                    g.set_screen_and_wait_for_any_key(GameScreen::KolyaInteraction(
                        state.clone(),
                        Altruism,
                    ))
                    .await;
                }
                state.player.money -= Money::oat_tincture_cost();
            }
            YesOrNoAction::No => {
                g.set_screen_and_wait_for_any_key(GameScreen::KolyaInteraction(
                    state.clone(),
                    BrakeFluidBecauseRefused,
                ))
                .await;
                // В этой ветке мозг может стать отрицательным и смерть не наступит.
                // Баг в оригинальной реализации.
                state.player.brain -= 1;
            }
        }
    }
}
