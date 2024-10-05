use super::super::*;
use crate::random::Rng;

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

/// Возвращает `Some`, если Коля может помочь решить задачи по алгебре,
/// иначе — `None`.
fn kolya_maybe_solve_algebra_problems(
    rng: &mut Rng,
    player: &mut Player,
) -> Option<ActionVec> {
    use Subject::AlgebraAndNumberTheory;
    let has_enough_charisma = player.charisma > rng.random(CharismaLevel(10));
    let algebra = player.status_for_subject(AlgebraAndNumberTheory);
    let problems_done = algebra.problems_done();
    let required_problems = SUBJECTS[AlgebraAndNumberTheory].required_problems;
    let has_at_least_2_remaining_problems = problems_done + 2 <= required_problems;
    if has_enough_charisma && has_at_least_2_remaining_problems {
        Some(wait_for_any_key())
    } else {
        None
    }
}

pub(in crate::logic) fn interact(
    game: &mut InternalGameState,
    mut state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    let player = &mut state.player;
    let (available_actions, interaction) = if let Some(available_actions) =
        kolya_maybe_solve_algebra_problems(&mut game.rng, player)
    {
        (available_actions, SolvedAlgebraProblemsForFree)
    } else if player.money < Money::oat_tincture_cost() {
        // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        (wait_for_any_key(), BrakeFluidNoMoney)
    } else {
        // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
        // "Заказать Коле настойку овса?"
        (
            ActionVec::from([Action::Yes, Action::No]),
            PromptOatTincture,
        )
    };
    game.set_screen(GameScreen::KolyaInteraction(state, interaction));
    available_actions
}

pub(in crate::logic) fn proceed(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: KolyaInteraction,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    assert_matches!(&*game.screen(), GameScreen::KolyaInteraction(_, _));
    let player = &mut state.player;
    match action {
        Action::AnyKey => {
            let algebra_status =
                player.status_for_subject_mut(Subject::AlgebraAndNumberTheory);
            match interaction {
                SolvedAlgebraProblemsForFree => {
                    algebra_status.more_problems_solved(2);
                    return game.hour_pass(state);
                }
                PromptOatTincture => unreachable!(),
                SolvedAlgebraProblemsForOatTincture => {
                    algebra_status.more_problems_solved(2);
                    player.money -= Money::oat_tincture_cost();
                    return game.hour_pass(state);
                }
                BrakeFluidNoMoney => {
                    player.brain -= 1;
                    if player.brain <= BrainLevel(0) {
                        player.health = HealthLevel(0);
                        player.cause_of_death = Some(CauseOfDeath::DrankTooMuch);
                        return scene_router::game_end(game, state);
                    }
                }
                BrakeFluidBecauseRefused => {
                    state.player.brain -= 1;
                    // Забавно, что в этой ветке можно бесконечно пить тормозную
                    // жидкость и никогда не спиться. Баг в оригинальной реализации.
                }
                Altruism => {
                    player.money -= Money::oat_tincture_cost();
                }
            }
            scene_router::run_sync(game, state)
        }
        Action::Yes => {
            assert_eq!(interaction, PromptOatTincture);
            if let Some(num_actions) =
                kolya_maybe_solve_algebra_problems(&mut game.rng, player)
            {
                // "Коля решил тебе ещё 2 задачи по алгебре!"
                game.set_screen(GameScreen::KolyaInteraction(
                    state,
                    SolvedAlgebraProblemsForOatTincture,
                ));
                num_actions
            } else {
                // "Твой альтруизм навсегда останется в памяти потомков."
                game.set_screen(GameScreen::KolyaInteraction(state, Altruism));
                wait_for_any_key()
            }
        }
        Action::No => {
            assert_eq!(interaction, PromptOatTincture);
            // "Зря, ой, зря ..."
            // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
            game.set_screen(GameScreen::KolyaInteraction(
                state,
                BrakeFluidBecauseRefused,
            ));
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
