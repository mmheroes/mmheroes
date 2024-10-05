use super::super::*;

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
    LetsHaveABreakHere { drink_beer: bool, hour_pass: bool },

    /// "Конспектировать ничего не надо. В мире есть ксероксы!"
    NoNeedToTakeLectureNotes { drink_beer: bool, hour_pass: bool },

    /// "А с четвертого курса вылететь уже почти невозможно."
    CantBeExpelledInFourthYear { drink_beer: bool, hour_pass: bool },

    /// "Вот у механиков - у них халява!"
    MechanicsHaveFreebie { drink_beer: bool, hour_pass: bool },
}

use GrishaInteraction::*;

pub(in crate::logic) fn interact(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    let player = &state.player;
    let has_enough_charisma = player.charisma > game.rng.random(CharismaLevel(20));
    let (actions, interaction) = if !player.is_employed_at_terkom() && has_enough_charisma
    {
        (
            ActionVec::from([
                Action::AcceptEmploymentAtTerkom,
                Action::DeclineEmploymentAtTerkom,
            ]),
            PromptEmploymentAtTerkom,
        )
    } else if !player.has_internet() && has_enough_charisma {
        (wait_for_any_key(), ProxyAddress)
    } else {
        let drink_beer = game.rng.random(3) > 0;
        let hour_pass = game.rng.roll_dice(3);
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
            LetsHaveABreakHere {
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
        (wait_for_any_key(), *game.rng.random_element(&replies[..]))
    };
    game.set_screen(GameScreen::GrishaInteraction(state, interaction));
    actions
}

pub(in crate::logic) fn proceed(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: GrishaInteraction,
) -> ActionVec {
    assert_matches!(&*game.screen(), GameScreen::GrishaInteraction(_, _));
    let player = &mut state.player;
    match action {
        Action::AnyKey => match interaction {
            PromptEmploymentAtTerkom => unreachable!(),
            CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => {
                scene_router::run(game, state)
            }
            ProxyAddress => {
                assert!(!player.has_internet());
                player.set_has_internet();
                scene_router::run(game, state)
            }
            WantFreebie {
                drink_beer,
                hour_pass,
            }
            | FreebieComeToMe {
                drink_beer,
                hour_pass,
            }
            | FreebieExists {
                drink_beer,
                hour_pass,
            }
            | LetsOrganizeFreebieLoversClub {
                drink_beer,
                hour_pass,
            }
            | NoNeedToStudyToGetDiploma {
                drink_beer,
                hour_pass,
            }
            | YouStudiedDidItHelp {
                drink_beer,
                hour_pass,
            }
            | ThirdYearStudentsDontAttendLectures {
                drink_beer,
                hour_pass,
            }
            | TakeExampleFromKolya {
                drink_beer,
                hour_pass,
            }
            | HateLevTolstoy {
                drink_beer,
                hour_pass,
            }
            | DontGoToPDMI {
                drink_beer,
                hour_pass,
            }
            | NamesOfFreebieLovers {
                drink_beer,
                hour_pass,
            }
            | LetsHaveABreakHere {
                drink_beer,
                hour_pass,
            }
            | NoNeedToTakeLectureNotes {
                drink_beer,
                hour_pass,
            }
            | CantBeExpelledInFourthYear {
                drink_beer,
                hour_pass,
            }
            | MechanicsHaveFreebie {
                drink_beer,
                hour_pass,
            } => {
                if drink_beer {
                    player.brain -= game.rng.random(2);
                    if player.brain <= BrainLevel(0) {
                        player.health = HealthLevel(0);
                        player.cause_of_death = Some(CauseOfDeath::DrankTooMuchBeer);
                        return scene_router::game_end(game, state);
                    }
                    player.charisma += game.rng.random(2);
                }
                if hour_pass {
                    return game.hour_pass(state);
                }

                scene_router::run(game, state)
            }
        },
        Action::AcceptEmploymentAtTerkom => {
            assert_eq!(interaction, PromptEmploymentAtTerkom);
            assert!(!player.is_employed_at_terkom());
            player.set_employed_at_terkom();
            game.set_screen(GameScreen::GrishaInteraction(
                state,
                CongratulationsYouAreNowEmployed,
            ));
            wait_for_any_key()
        }
        Action::DeclineEmploymentAtTerkom => {
            assert_eq!(interaction, PromptEmploymentAtTerkom);
            assert!(!player.is_employed_at_terkom());
            game.set_screen(GameScreen::GrishaInteraction(
                state,
                AsYouWantButDontOverstudy,
            ));
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
