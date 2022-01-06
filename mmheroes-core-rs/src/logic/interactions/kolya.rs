use super::super::*;
use crate::random::Rng;

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

pub(in crate::logic) fn interact(game: &mut Game, mut state: GameState) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    let player = &mut state.player;
    let (available_actions, interaction) = if let Some(available_actions) =
        kolya_maybe_solve_algebra_problems(&mut game.rng, player)
    {
        (
            available_actions,
            npc::KolyaInteraction::SolvedAlgebraProblemsForFree,
        )
    } else if player.money < Money::oat_tincture_cost() {
        // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        (wait_for_any_key(), npc::KolyaInteraction::BrakeFluidNoMoney)
    } else {
        // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
        // "Заказать Коле настойку овса?"
        (
            ActionVec::from([Action::Yes, Action::No]),
            npc::KolyaInteraction::PromptOatTincture,
        )
    };
    game.screen = GameScreen::KolyaInteraction(state, interaction);
    available_actions
}

pub(in crate::logic) fn proceed(
    game: &mut Game,
    mut state: GameState,
    action: Action,
    interaction: npc::KolyaInteraction,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    assert_matches!(game.screen, GameScreen::KolyaInteraction(_, _));
    let player = &mut state.player;
    match action {
        Action::AnyKey => {
            let algebra_status =
                player.status_for_subject_mut(Subject::AlgebraAndNumberTheory);
            match interaction {
                npc::KolyaInteraction::SolvedAlgebraProblemsForFree => {
                    algebra_status.more_problems_solved(2);
                    return game.hour_pass(state);
                }
                npc::KolyaInteraction::PromptOatTincture => unreachable!(),
                npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture => {
                    algebra_status.more_problems_solved(2);
                    player.money -= Money::oat_tincture_cost();
                    return game.hour_pass(state);
                }
                npc::KolyaInteraction::BrakeFluidNoMoney => {
                    player.brain -= 1;
                    if player.brain <= BrainLevel(0) {
                        player.health = HealthLevel(0);
                        player.cause_of_death = Some(CauseOfDeath::DrankTooMuch);
                        return game.game_end(state);
                    }
                }
                npc::KolyaInteraction::BrakeFluidBecauseRefused => {
                    state.player.brain -= 1;
                    // Забавно, что в этой ветке можно бесконечно пить тормозную
                    // жидкость и никогда не спиться. Баг в оригинальной реализации.
                }
                npc::KolyaInteraction::Altruism => {
                    player.money -= Money::oat_tincture_cost();
                }
            }
            game.scene_router(state)
        }
        Action::Yes => {
            assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
            if let Some(num_actions) =
                kolya_maybe_solve_algebra_problems(&mut game.rng, player)
            {
                // "Коля решил тебе ещё 2 задачи по алгебре!"
                game.screen = GameScreen::KolyaInteraction(
                    state,
                    npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture,
                );
                num_actions
            } else {
                // "Твой альтруизм навсегда останется в памяти потомков."
                game.screen =
                    GameScreen::KolyaInteraction(state, npc::KolyaInteraction::Altruism);
                wait_for_any_key()
            }
        }
        Action::No => {
            assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
            // "Зря, ой, зря ..."
            // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
            game.screen = GameScreen::KolyaInteraction(
                state,
                npc::KolyaInteraction::BrakeFluidBecauseRefused,
            );
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
