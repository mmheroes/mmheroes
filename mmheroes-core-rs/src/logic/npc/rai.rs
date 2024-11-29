use super::*;
use crate::logic::actions::RaiAction;

#[derive(Debug, Clone)]
pub enum RaiInteraction {
    /// "RAI не реагирует на твои позывы."
    Ignores(GameState),

    /// "Ты мне поможешь?"
    PromptWillYouHelpMe(GameState),

    /// "Ах, так! Получай! Получай!"
    TakeIt,

    /// "Ты помог RAI."
    YouHelped,

    /// "Ничего не вышло."
    Fail,
}

use RaiInteraction::*;

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    match exam_in_progress {
        Some(subject) if !subject.is_math() => {
            rai_ignores_you(g, state).await;
        }
        None => {
            rai_ignores_you(g, state).await;
        }
        Some(subject) => {
            match g
                .set_screen_and_wait_for_action::<RaiAction>(GameScreen::RaiInteraction(
                    PromptWillYouHelpMe(state.clone()),
                ))
                .await
            {
                RaiAction::YesOfCourse => {
                    if g.rng
                        .random(state.player.status_for_subject(subject).knowledge)
                        > g.rng.random(subject.mental_load())
                    {
                        g.set_screen_and_wait_for_any_key(GameScreen::RaiInteraction(
                            YouHelped,
                        ))
                        .await;
                        state.player.brain += 1;
                    } else {
                        g.set_screen_and_wait_for_any_key(GameScreen::RaiInteraction(
                            Fail,
                        ))
                        .await;
                    }
                    misc::hour_pass(g, state).await;
                }
                RaiAction::NoSorry => {
                    g.set_screen_and_wait_for_any_key(GameScreen::RaiInteraction(TakeIt))
                        .await;
                    misc::decrease_health(
                        state,
                        HealthLevel(10),
                        CauseOfDeath::KilledByRai,
                    )
                }
            }
        }
    };
}

async fn rai_ignores_you(g: &mut InternalGameState<'_>, state: &GameState) {
    g.set_screen_and_wait_for_any_key(GameScreen::RaiInteraction(Ignores(state.clone())))
        .await
}
