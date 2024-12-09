use crate::logic::{
    actions, misc, BrainLevel, CauseOfDeath, CharismaLevel, GameScreen, GameState,
    InternalGameState, Money, StaminaLevel, Subject,
};

#[derive(Debug, Clone)]
pub enum NilInteraction {
    /// "Маладой чилавек, вы мне не паможите решить задачу?
    /// А то я сигодня ни в зуб нагой ..."
    WillYouHelpMe(GameState),

    RefusedToHelp,

    /// "Ой, спасибо! Вот вам x руб. за это..."
    ThanksHereIsYourMoney(Money),

    /// "У тебя ничего не вышло."
    DidntWorkOut,
}

use crate::logic::nil::NilInteraction::{
    DidntWorkOut, RefusedToHelp, ThanksHereIsYourMoney,
};
use NilInteraction::WillYouHelpMe;

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    let subject =
        exam_in_progress.expect("Взаимодействие с NiL возможно только на зачёте");
    assert!(subject.is_math());

    match g
        .set_screen_and_wait_for_action(GameScreen::NilInteraction(WillYouHelpMe(
            state.clone(),
        )))
        .await
    {
        actions::NilAction::YesOfCourse => {
            let player = &mut state.player;
            let subject_knowledge = player.status_for_subject(subject).knowledge();
            if subject_knowledge > subject.mental_load() {
                let reward = Money(subject_knowledge.0);
                g.set_screen_and_wait_for_any_key(GameScreen::NilInteraction(
                    ThanksHereIsYourMoney(reward),
                ))
                .await;
                player.money += subject_knowledge.0;

                // Тут что-то странное, но так сделано в оригинале.
                // При этом может возникнуть отрицательный уровень знания.
                player.status_for_subject_mut(subject).knowledge -=
                    subject.single_problem_mental_factor() as i16
                        + g.rng.random(subject.health_penalty());
                misc::decrease_health(
                    state,
                    subject.health_penalty(),
                    CauseOfDeath::Altruism,
                );
                misc::hour_pass(g, state, exam_in_progress).await;
            } else {
                g.set_screen_and_wait_for_any_key(GameScreen::NilInteraction(
                    DidntWorkOut,
                ))
                .await;
                misc::hour_pass(g, state, exam_in_progress).await;
                misc::decrease_health(
                    state,
                    subject.health_penalty(),
                    CauseOfDeath::Altruism,
                );
            }
        }
        actions::NilAction::MaybeNextTime => {
            g.set_screen_and_wait_for_any_key(GameScreen::NilInteraction(RefusedToHelp))
                .await;
            // Баг в оригинальной реализации: если какая-то из этих характеристик
            // опускается до нуля, смерть не наступает.
            let player = &mut state.player;
            player.brain -= g.rng.random(BrainLevel(2));
            player.charisma -= g.rng.random(CharismaLevel(2));
            player.stamina -= g.rng.random(StaminaLevel(2));
        }
    }
}
