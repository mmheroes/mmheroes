use crate::logic::{
    actions, misc, CauseOfDeath, GameScreen, GameState, InternalGameState, Location,
    Subject,
};
use strum::VariantArray;

#[derive(Debug, Clone)]
pub enum MishaInteraction {
    /// "Слушай, хватит мучаться! Прервись! Давай в клоподавку сыграем!"
    PromptBugSquasher(GameState),

    /// "Ты сыграл с Мишей партию в клоподавку."
    PlayedBugSquasherWithMisha,

    /// "Зря, очень зря!" (когда отказался играть в клоподавку)
    TooBad,

    /// "Слушай, а ведь в ТЕРКОМе есть столик для тенниса. Сыграем?"
    PromptTennis(GameState),

    /// "Ты сыграл с Мишей в теннис."
    PlayedTennisWithMisha,

    /// "Ничего, я на тебя не в обиде." (когда отказался играть в теннис)
    NoWorries,

    RandomReply(GameState, MishaReply),
}

use MishaInteraction::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum MishaReply {
    /// "Эх, жаль, негде сыграть в клоподавку!"
    TooBadNowhereToPlayBugSquasher,

    /// "Всегда следи за здоровьем!"
    AlwaysPayAttentionToHealth,

    /// "Мозги влияют на подготовку и сдачу зачетов."
    BrainLevelAffectsExamSuccess,

    /// "Чем больше выносливость, тем меньше здоровья ты тратишь."
    TheMoreStaminaTheLessHealthYouSpend,

    /// "Чем больше твоя харизма, тем лучше у тебя отношения с людьми."
    TheMoreCharismaTheBetterRelationshipsWithPeople,

    /// "Важность конкретного качества сильно зависит от стиля игры."
    ImportanceOfCharacteristicAffectsGameStyle,

    /// "Харизма помогает получить что угодно от кого угодно."
    CharismaHelpsGetAnything,

    /// "Чем больше харизма, тем чаще к тебе пристают."
    TheMoreCharismaTheMoreYouAreApproached,

    /// "Чем меньше выносливость, тем больнее учиться."
    TheLessStaminaTheMorePainfulStudyingIs,

    /// "Чем больше мозги, тем легче готовиться."
    TheMoreBrainTheMoreEasyToPrepare,

    /// "Сидение в Inet\'e иногда развивает мозги."
    InternetSometimesImprovesBrain,

    /// "Если тебе надоело умирать - попробуй другую стратегию."
    IfTiredOfDyingTryAnotherStrategy,

    /// "Хочешь халявы - набирай харизму."
    WantFreebieGetCharisma,

    /// "Хочешь добиться всего сам - развивай мозги."
    WantAchieveEverythingYourselfImproveBrain,

    /// "В "Мавзолее" важно знать меру..."
    InMausoleumKnowingWhenToStopIsImportant,

    /// "От раздвоения личности спасают харизма и выносливость."
    CharismaAndStaminaSaveFromPersonalityDisorder,

    /// "От любого общения с NiL ты тупеешь!"
    YouGetStupidFromInteractingWithNil,

    /// "Гриша может помочь с трудоустройством."
    GrishaCanHelpWithEmployment,

    /// "Перемещения студентов предсказуемы."
    NpcMovementsArePredictable,
}

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    match exam_in_progress {
        Some(subject) if subject != Subject::ComputerScience => {
            maybe_play_bug_squasher_with_misha(g, state, exam_in_progress).await;
            return;
        }
        _ => (),
    }
    if state.location() == Location::PUNK
        && exam_in_progress.is_none()
        && state.player().is_employed_at_terkom()
        && state.player().charisma() > g.rng.random(8)
    {
        maybe_play_tennis_with_misha(g, state, exam_in_progress).await;
        return;
    }

    let reply = g.rng.random_variant();
    g.set_screen_and_wait_for_any_key(GameScreen::MishaInteraction(RandomReply(
        state.clone(),
        reply,
    )))
    .await;
}

async fn maybe_play_bug_squasher_with_misha(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    match g
        .set_screen_and_wait_for_action(GameScreen::MishaInteraction(PromptBugSquasher(
            state.clone(),
        )))
        .await
    {
        actions::BugSquasherAction::LetsGo => {
            g.set_screen_and_wait_for_any_key(GameScreen::MishaInteraction(
                PlayedBugSquasherWithMisha,
            ))
            .await;
            state.player.charisma += 1;
            misc::hour_pass(g, state, exam_in_progress).await;
        }
        actions::BugSquasherAction::NoIWontPlay => {
            g.set_screen_and_wait_for_any_key(GameScreen::MishaInteraction(TooBad))
                .await;
            state.player.charisma -= g.rng.random(2);
        }
    }
}

async fn maybe_play_tennis_with_misha(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    match g
        .set_screen_and_wait_for_action(GameScreen::MishaInteraction(PromptTennis(
            state.clone(),
        )))
        .await
    {
        actions::TennisAction::Sure => {
            g.set_screen_and_wait_for_any_key(GameScreen::MishaInteraction(
                PlayedTennisWithMisha,
            ))
            .await;
            state.player.charisma += 1;
            if state.player.charisma < g.rng.random(10) {
                misc::decrease_health(
                    state,
                    g.rng.random_in_range(3..6),
                    CauseOfDeath::ExhaustedByMisha,
                )
            }
            misc::hour_pass(g, state, exam_in_progress).await;
        }
        actions::TennisAction::SorryMaybeLater => {
            g.set_screen_and_wait_for_any_key(GameScreen::MishaInteraction(NoWorries))
                .await
        }
    }
}
