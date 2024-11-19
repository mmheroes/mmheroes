use crate::logic::actions::TerkomEmploymentAction;
use crate::logic::{
    misc, BrainLevel, CauseOfDeath, CharismaLevel, GameScreen, GameState,
    InternalGameState, Location,
};
use strum::VariantArray;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GrishaInteraction {
    /// "А ты не хочешь устроиться в ТЕРКОМ? Может, кое-чего подзаработаешь..."
    /// (да или нет)
    PromptEmploymentAtTerkom,

    /// "Поздравляю, теперь ты можешь идти в "контору"!"
    CongratulationsYouAreNowEmployed,

    /// "Как хочешь. Только смотри, не заучись там ..."
    AsYouWantButDontOverstudy,

    /// "Кстати, я тут знаю один качественно работающий прокси-сервер..."
    ProxyAddress,

    RandomReply {
        reply: GrishaReply,
        drink_beer: bool,
        hour_pass: bool,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum GrishaReply {
    /// "Хочу халявы!"
    WantFreebie,

    /// "Прийди же, о халява!"
    FreebieComeToMe,

    /// "Халява есть - ее не может не быть."
    FreebieExists,

    /// "Давай организуем клуб любетелей халявы!"
    LetsOrganizeFreebieLoversClub,

    /// "Чтобы получить диплом, учиться совершенно необязательно!"
    NoNeedToStudyToGetDiploma,

    /// "Ну вот, ты готовился... Помогло это тебе?"
    YouStudiedDidItHelp,

    /// "На третьем курсе на лекции уже никто не ходит. Почти никто."
    ThirdYearStudentsDontAttendLectures,

    /// "Вот, бери пример с Коли."
    TakeExampleFromKolya,

    /// "Ненавижу Льва Толстого! Вчера "Войну и мир" <йк> ксерил..."
    HateLevTolstoy,

    /// "А в ПОМИ лучше вообще не ездить!"
    DontGoToPDMI,

    /// "Имена главных халявчиков и алкоголиков висят на баобабе."
    NamesOfFreebieLovers,

    /// "Правильно, лучше посидим здесь и оттянемся!"
    SitHereAndChill,

    /// "Конспектировать ничего не надо. В мире есть ксероксы!"
    NoNeedToTakeLectureNotes,

    /// "А с четвертого курса вылететь уже почти невозможно."
    CantBeExpelledInFourthYear,

    /// "Вот у механиков - у них халява!"
    MechanicsHaveFreebie,
}

use GrishaInteraction::*;

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    assert_eq!(state.location(), Location::Mausoleum);
    let mut has_enough_charisma =
        || state.player.charisma > g.rng.random(CharismaLevel(20));
    if !state.player.is_employed_at_terkom() && has_enough_charisma() {
        match g
            .set_screen_and_wait_for_action::<TerkomEmploymentAction>(
                GameScreen::GrishaInteraction(PromptEmploymentAtTerkom),
            )
            .await
        {
            TerkomEmploymentAction::Accept => {
                g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
                    CongratulationsYouAreNowEmployed,
                ))
                .await;
                state.player.set_employed_at_terkom();
            }
            TerkomEmploymentAction::Decline => {
                g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
                    AsYouWantButDontOverstudy,
                ))
                .await;
            }
        }
    } else if !state.player.has_internet() && has_enough_charisma() {
        g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(ProxyAddress))
            .await;
        state.player.set_has_internet();
    } else {
        let drink_beer = g.rng.random(3) > 0;
        let hour_pass = g.rng.roll_dice(3);
        let reply = g.rng.random_variant();
        g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(RandomReply {
            reply,
            drink_beer,
            hour_pass,
        }))
        .await;
        if drink_beer {
            misc::decrease_brain(
                state,
                BrainLevel(g.rng.random(2)),
                CauseOfDeath::DrankTooMuchBeer,
            );
            state.player.charisma += g.rng.random(2);
        }
        if hour_pass {
            misc::hour_pass(g, state).await;
        }
    }
}
