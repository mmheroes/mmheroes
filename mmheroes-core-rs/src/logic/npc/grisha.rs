use crate::logic::actions::TerkomEmploymentAction;
use crate::logic::{
    misc, BrainLevel, CauseOfDeath, CharismaLevel, GameScreen, GameState,
    InternalGameState, Location,
};

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

    /// "Хочу халявы!"
    WantFreebie { drink_beer: bool, hour_pass: bool },

    /// "Прийди же, о халява!"
    FreebieComeToMe { drink_beer: bool, hour_pass: bool },

    /// "Халява есть - ее не может не быть."
    FreebieExists { drink_beer: bool, hour_pass: bool },

    /// "Давай организуем клуб любетелей халявы!"
    LetsOrganizeFreebieLoversClub { drink_beer: bool, hour_pass: bool },

    /// "Чтобы получить диплом, учиться совершенно необязательно!"
    NoNeedToStudyToGetDiploma { drink_beer: bool, hour_pass: bool },

    /// "Ну вот, ты готовился... Помогло это тебе?"
    YouStudiedDidItHelp { drink_beer: bool, hour_pass: bool },

    /// "На третьем курсе на лекции уже никто не ходит. Почти никто."
    ThirdYearStudentsDontAttendLectures { drink_beer: bool, hour_pass: bool },

    /// "Вот, бери пример с Коли."
    TakeExampleFromKolya { drink_beer: bool, hour_pass: bool },

    /// "Ненавижу Льва Толстого! Вчера "Войну и мир" <йк> ксерил..."
    HateLevTolstoy { drink_beer: bool, hour_pass: bool },

    /// "А в ПОМИ лучше вообще не ездить!"
    DontGoToPDMI { drink_beer: bool, hour_pass: bool },

    /// "Имена главных халявчиков и алкоголиков висят на баобабе."
    NamesOfFreebieLovers { drink_beer: bool, hour_pass: bool },

    /// "Правильно, лучше посидим здесь и оттянемся!"
    SitHereAndChill { drink_beer: bool, hour_pass: bool },

    /// "Конспектировать ничего не надо. В мире есть ксероксы!"
    NoNeedToTakeLectureNotes { drink_beer: bool, hour_pass: bool },

    /// "А с четвертого курса вылететь уже почти невозможно."
    CantBeExpelledInFourthYear { drink_beer: bool, hour_pass: bool },

    /// "Вот у механиков - у них халява!"
    MechanicsHaveFreebie { drink_beer: bool, hour_pass: bool },
}

use GrishaInteraction::*;

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    assert_eq!(state.location(), Location::Mausoleum);
    let mut has_enough_charisma =
        || state.player.charisma > g.rng.random(CharismaLevel(20));
    if !state.player.is_employed_at_terkom() && has_enough_charisma() {
        match g
            .set_screen_and_wait_for_action::<TerkomEmploymentAction>(
                GameScreen::GrishaInteraction(state.clone(), PromptEmploymentAtTerkom),
            )
            .await
        {
            TerkomEmploymentAction::Accept => {
                g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
                    state.clone(),
                    CongratulationsYouAreNowEmployed,
                ))
                .await;
                state.player.set_employed_at_terkom();
            }
            TerkomEmploymentAction::Decline => {
                g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
                    state.clone(),
                    AsYouWantButDontOverstudy,
                ))
                .await;
            }
        }
    } else if !state.player.has_internet() && has_enough_charisma() {
        g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
            state.clone(),
            ProxyAddress,
        ))
        .await;
        state.player.set_has_internet();
    } else {
        let drink_beer = g.rng.random(3) > 0;
        let hour_pass = g.rng.roll_dice(3);
        let replies = [
            WantFreebie {
                drink_beer,
                hour_pass,
            },
            FreebieComeToMe {
                drink_beer,
                hour_pass,
            },
            FreebieExists {
                drink_beer,
                hour_pass,
            },
            LetsOrganizeFreebieLoversClub {
                drink_beer,
                hour_pass,
            },
            NoNeedToStudyToGetDiploma {
                drink_beer,
                hour_pass,
            },
            YouStudiedDidItHelp {
                drink_beer,
                hour_pass,
            },
            ThirdYearStudentsDontAttendLectures {
                drink_beer,
                hour_pass,
            },
            TakeExampleFromKolya {
                drink_beer,
                hour_pass,
            },
            HateLevTolstoy {
                drink_beer,
                hour_pass,
            },
            DontGoToPDMI {
                drink_beer,
                hour_pass,
            },
            NamesOfFreebieLovers {
                drink_beer,
                hour_pass,
            },
            SitHereAndChill {
                drink_beer,
                hour_pass,
            },
            NoNeedToTakeLectureNotes {
                drink_beer,
                hour_pass,
            },
            CantBeExpelledInFourthYear {
                drink_beer,
                hour_pass,
            },
            MechanicsHaveFreebie {
                drink_beer,
                hour_pass,
            },
        ];
        let reply = *g.rng.random_element(&replies);
        g.set_screen_and_wait_for_any_key(GameScreen::GrishaInteraction(
            state.clone(),
            reply,
        ))
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
