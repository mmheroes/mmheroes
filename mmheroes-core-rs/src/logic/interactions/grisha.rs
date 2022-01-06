use super::super::*;

pub(in crate::logic) fn interact(game: &mut Game, state: GameState) -> ActionVec {
    use npc::GrishaInteraction::*;
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
    game.screen = GameScreen::GrishaInteraction(state, interaction);
    actions
}

pub(in crate::logic) fn proceed(
    game: &mut Game,
    mut state: GameState,
    action: Action,
    interaction: npc::GrishaInteraction,
) -> ActionVec {
    use npc::GrishaInteraction::*;
    assert_matches!(game.screen, GameScreen::GrishaInteraction(_, _));
    let player = &mut state.player;
    match action {
        Action::AnyKey => match interaction {
            PromptEmploymentAtTerkom => unreachable!(),
            CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => {
                game.scene_router(state)
            }
            ProxyAddress => {
                assert!(!player.has_internet());
                player.set_has_internet();
                game.scene_router(state)
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
                        return game.game_end(state);
                    }
                    player.charisma += game.rng.random(2);
                }
                if hour_pass {
                    return game.hour_pass(state);
                }

                game.scene_router(state)
            }
        },
        Action::AcceptEmploymentAtTerkom => {
            assert_eq!(interaction, PromptEmploymentAtTerkom);
            assert!(!player.is_employed_at_terkom());
            player.set_employed_at_terkom();
            game.screen =
                GameScreen::GrishaInteraction(state, CongratulationsYouAreNowEmployed);
            wait_for_any_key()
        }
        Action::DeclineEmploymentAtTerkom => {
            assert_eq!(interaction, PromptEmploymentAtTerkom);
            assert!(!player.is_employed_at_terkom());
            game.screen = GameScreen::GrishaInteraction(state, AsYouWantButDontOverstudy);
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
