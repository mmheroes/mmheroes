use crate::logic::diamond::DiamondInteraction;
use crate::logic::grisha::GrishaReply;
use crate::logic::misha::{MishaInteraction, MishaReply};
use crate::logic::npc::{
    grisha::GrishaInteraction, kolya::KolyaInteraction, kuzmenko::KuzmenkoInteraction,
    nil::NilInteraction, pasha::PashaInteraction, sasha::SashaInteraction,
    serj::SerjInteraction,
};
use crate::logic::rai::RaiInteraction;
use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

fn solved_algebra_problems(r: &mut Renderer<impl RendererRequestConsumer>) {
    writeln_colored!(
        WhiteBright,
        r,
        "\"У тебя остались нерешенные задачи по Всемирнову? Давай сюда!\""
    );
    write_colored!(White, r, "Коля решил тебе еще ");
    write_colored!(WhiteBright, r, "{}", 2);
    writeln_colored!(White, r, " задачи по алгебре!");
}

fn brake_fluid(r: &mut Renderer<impl RendererRequestConsumer>) {
    writeln_colored!(
        MagentaBright,
        r,
        "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
    );
}

pub(in crate::ui) fn display_kolya_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    available_actions: &[Action],
    interaction: KolyaInteraction,
) -> WaitingState {
    use KolyaInteraction::*;

    match interaction {
        SolvedAlgebraProblemsForFree | PromptOatTincture | BrakeFluidNoMoney => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            writeln_colored!(
                White,
                r,
                "Коля смотрит на тебя немного окосевшими глазами."
            );
        }
        _ => (),
    }

    match interaction {
        SolvedAlgebraProblemsForFree => {
            solved_algebra_problems(r);
            return wait_for_any_key(r);
        }
        PromptOatTincture | BrakeFluidNoMoney => {
            writeln_colored!(
                WhiteBright,
                r,
                "\"Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!\""
            );
        }
        _ => {}
    }

    match interaction {
        SolvedAlgebraProblemsForFree => unreachable!(),
        PromptOatTincture => {
            writeln_colored!(White, r, "Заказать Коле настойку овса?");
            scene_router::display_short_today_timetable(r, 11, state);
            r.move_cursor_to(14, 0);
            return dialog(r, available_actions);
        }
        SolvedAlgebraProblemsForOatTincture => {
            r.move_cursor_to(18, 0);
            solved_algebra_problems(r);
        }
        BrakeFluidNoMoney => {
            r.move_cursor_to(14, 0);
            brake_fluid(r);
            return wait_for_any_key(r);
        }
        BrakeFluidBecauseRefused => {
            r.move_cursor_to(18, 0);
            writeln_colored!(WhiteBright, r, "\"Зря, ой, зря ...\"");
            brake_fluid(r);
        }
        Altruism => {
            r.move_cursor_to(18, 0);
            writeln_colored!(
                White,
                r,
                "Твой альтруизм навсегда останется в памяти потомков."
            );
        }
    }

    wait_for_any_key(r)
}

pub(in crate::ui) fn display_pasha_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    interaction: PashaInteraction,
) -> WaitingState {
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    match interaction {
        PashaInteraction::Stipend => {
            write_colored!(White, r, "Паша вручает тебе твою стипуху за май: ");
            write_colored!(WhiteBright, r, "{}", Money::stipend());
            write_colored!(White, r, " руб.");
        }
        PashaInteraction::Inspiration => {
            writeln_colored!(Green, r, "Паша воодушевляет тебя на великие дела.");
            writeln_colored!(RedBright, r, "Вместе с этим он немного достает тебя.");
        }
    }
    wait_for_any_key(r)
}

pub(in crate::ui) fn display_grisha_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    available_actions: &[Action],
    interaction: GrishaInteraction,
) -> WaitingState {
    use GrishaInteraction::*;
    use GrishaReply::*;

    match interaction {
        PromptEmploymentAtTerkom | ProxyAddress | RandomReply { .. } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::White, Color::Black);
        }
        CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => (),
    }

    match interaction {
        PromptEmploymentAtTerkom => {
            writeln_colored!(
                YellowBright,
                r,
                "\"А ты не хочешь устроиться в ТЕРКОМ? Может, кое-чего подзаработаешь...\""
            );
            writeln!(r);
            dialog(r, available_actions)
        }
        CongratulationsYouAreNowEmployed => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Поздравляю, теперь ты можешь идти в \"контору\"!\""
            );
            wait_for_any_key(r)
        }
        AsYouWantButDontOverstudy => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Как хочешь. Только смотри, не заучись там ...\""
            );
            wait_for_any_key(r)
        }
        ProxyAddress => {
            writeln_colored!(
                White,
                r,
                "\"Кстати, я тут знаю один качественно работающий прокси-сервер...\""
            );
            writeln!(r);
            writeln_colored!(White, r, "Ты записываешь адрес. Вдруг пригодится?");
            wait_for_any_key(r)
        }
        RandomReply {
            reply,
            drink_beer,
            hour_pass,
        } => {
            let text = match reply {
                WantFreebie => "Хочу халявы!",
                FreebieComeToMe => "Прийди же, о халява!",
                FreebieExists => "Халява есть - ее не может не быть.",
                LetsOrganizeFreebieLoversClub => {
                    "Давай организуем клуб любетелей халявы!"
                }
                NoNeedToStudyToGetDiploma => {
                    "Чтобы получить диплом, учиться совершенно необязательно!"
                }
                YouStudiedDidItHelp => "Ну вот, ты готовился... Помогло это тебе?",
                ThirdYearStudentsDontAttendLectures => {
                    "На третьем курсе на лекции уже никто не ходит. Почти никто."
                }
                TakeExampleFromKolya => "Вот, бери пример с Коли.",
                HateLevTolstoy => {
                    "Ненавижу Льва Толстого! Вчера \"Войну и мир\" <йк> ксерил..."
                }
                DontGoToPDMI => "А в ПОМИ лучше вообще не ездить!",
                NamesOfFreebieLovers => {
                    "Имена главных халявчиков и алкоголиков висят на баобабе."
                }
                SitHereAndChill => "Правильно, лучше посидим здесь и оттянемся!",
                NoNeedToTakeLectureNotes => {
                    "Конспектировать ничего не надо. В мире есть ксероксы!"
                }
                CantBeExpelledInFourthYear => {
                    "А с четвертого курса вылететь уже почти невозможно."
                }
                MechanicsHaveFreebie => "Вот у механиков - у них халява!",
            };
            write_colored!(White, r, "Гриша:");
            writeln_colored!(YellowBright, r, "\"{}\"", text);
            if drink_beer {
                writeln_colored!(White, r, "И еще по пиву...");
            }
            if hour_pass {
                writeln_colored!(
                    White,
                    r,
                    "И еще один час прошел в бесплодных разговорах..."
                );
            }
            wait_for_any_key(r)
        }
    }
}

pub(in crate::ui) fn display_sasha_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    available_actions: &[Action],
    interaction: SashaInteraction,
) -> WaitingState {
    match interaction {
        SashaInteraction::ChooseSubject => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::YellowBright, Color::Black);
            write!(
                r,
                "Ты встретил Сашу! Говорят, у него классные конспекты ..."
            );
            r.move_cursor_to(8, 0);
            write!(r, "Чего тебе надо от Саши?");
            r.move_cursor_to(10, 0);
            dialog(r, available_actions)
        }
        _ => {
            r.move_cursor_to(14, 0);
            match interaction {
                SashaInteraction::SuitYourself => {
                    write_colored!(White, r, "Как знаешь...");
                }
                _ => {
                    write_colored!(White, r, "Саша:");
                    match interaction {
                        SashaInteraction::ChooseSubject
                        | SashaInteraction::SuitYourself => unreachable!(),
                        SashaInteraction::YesIHaveTheLectureNotes => {
                            write_colored!(
                                WhiteBright,
                                r,
                                "\"Да, у меня с собой этот конспект ...\""
                            );
                        }
                        SashaInteraction::SorryGaveToSomeoneElse => {
                            write_colored!(
                                WhiteBright,
                                r,
                                "\"Ох, извини, кто-то другой уже позаимствовал ...\""
                            );
                        }
                    }
                }
            }
            wait_for_any_key(r)
        }
    }
}

pub(in crate::ui) fn display_kuzmenko_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    interaction: KuzmenkoInteraction,
) -> WaitingState {
    use crate::logic::npc::kuzmenko::KuzmenkoInteraction::*;
    use crate::logic::npc::kuzmenko::KuzmenkoReply::*;
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    write_colored!(White, r, "Кузьменко:");
    let reply = match interaction {
        AdditionalComputerScienceExam { day_index } => {
            let exam = state
                .timetable()
                .day(day_index)
                .exam(Subject::ComputerScience)
                .unwrap();
            writeln_colored!(
                WhiteBright,
                r,
                "\"Вы знаете, Климова можно найти в компьютерном классе"
            );
            // Первый день недели — 22-е мая.
            writeln_colored!(
                WhiteBright,
                r,
                "{}-го мая с {} по {}ч..\"",
                22 + day_index,
                exam.from(),
                exam.to()
            );
            return wait_for_any_key(r);
        }
        RandomReply(FormatFloppy) => {
            "... отформатировать дискету так, чтобы 1ый сектор был 5ым ..."
        }
        RandomReply(FiltersInWindows) => {
            "А Вы нигде не видели литературы по фильтрам в Windows?"
        }
        RandomReply(ByteVisualization) => {
            "... написать визуализацию байта на ассемблере за 11 байт ..."
        }
        RandomReply(OlegPliss) => "У вас Олег Плисс ведет какие-нибудь занятия?",
        RandomReply(BillGatesMustDie) => "Bill Gates = must die = кабысдох (рус.).",
        RandomReply(MonitorJournal) => "Вы читали журнал \"Монитор\"? Хотя вряд ли...",
        RandomReply(MmheroesBP7) => "Я слышал, что mmHeroes написана на BP 7.0.",
        RandomReply(CSeminar) => "Записывайтесь на мой семинар по языку Си!",
        RandomReply(ThirdYear) => "На третьем курсе я буду вести у вас спецвычпрактикум.",
        RandomReply(STAR) => "Интересно, когда они снова наладят STAR?",
        RandomReply(GetYourselvesAnEmail) => {
            "Получите себе ящик rambler'e или на mail.ru !"
        }
        RandomReply(TerekhovSenior) => {
            "А разве Терехов-старший ничего не рассказывает про IBM PC?"
        }
    };
    writeln_colored!(WhiteBright, r, "\"{}\"", reply);
    wait_for_any_key(r)
}

pub(crate) fn display_diamond_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    interaction: DiamondInteraction,
    available_actions: &[Action],
    diamond_leaves: bool,
) -> WaitingState {
    use crate::logic::npc::diamond::DiamondInteraction::*;
    use crate::logic::npc::diamond::DiamondReply::*;

    let mut start = || {
        r.clear_screen();
        scene_router::display_header_stats(r, state);
        r.move_cursor_to(7, 0);
        writeln_colored!(
            YellowBright,
            r,
            "Wow! Ты только что встретил автора <Heroes of MAT-MEX == MMHEROES>!"
        );
        writeln!(r);
        write!(r, "Diamond:");
    };

    match interaction {
        WannaTestNewMMHEROES => {
            start();
            writeln!(r, "\"Хочешь по-тестить новую версию Heroes of MAT-MEX?\"");
            scene_router::display_short_today_timetable(r, 11, state);
            r.move_cursor_to(11, 0);
            dialog(r, available_actions)
        }
        HereIsTheFloppy => {
            r.move_cursor_to(15, 0);
            write_colored!(White, r, "\"Ну и ладушки! Вот тебе дискетка...\"");
            wait_for_any_key(r)
        }
        SorryForBothering => {
            r.move_cursor_to(15, 0);
            write_colored!(White, r, "\"Извини, что побеспокоил.\"");
            wait_for_any_key(r)
        }
        Reply(reply) => {
            start();
            let text = match reply {
                KolyaWillHelpWithAlgebra => "Коля поможет с алгеброй.",
                MishaWillTellEveryoneHowGoodYouAre => {
                    "Миша расскажет всем, какой ты хороший."
                }
                PashaIsYourHeadman => "Паша - твой староста.",
                BetterAvoidDJuG => "С DJuGом лучше не сталкиваться.",
                RAIWontLeaveYouAlone => "RAI не отстанет, лучше решить ему чего-нибудь.",
                KolyaIsAlwaysInMausoleum => {
                    "Коля все время сидит в мавзолее и оттягивается."
                }
                WatchYourHealth => "Следи за своим здоровьем!!!",
                IfYouMeetSashaTalkToHim => {
                    "Если встретишь Сашу - ОБЯЗАТЕЛЬНО заговори с ним."
                }
                IfTroubleThinkingTalkWithRAI => {
                    "Если плохо думается, попробуй поговорить с RAI."
                }
                BeSureYouCanDrinkBeforeGoingToKolya => {
                    "Идя к Коле, будь уверен, что можешь пить с ним."
                }
                ExpectSurprisesOnEnglishExam => {
                    "Получая зачет по английскому, будь готов к неожиданностям."
                }
                TalksWithSerj => "Иногда разговоры с Сержем приносят ощутимую пользу.",
                AndrewCanHelpButNotAlways => "Эндрю может помочь, но не всегда...",
                KuzmenkoKnowsAboutKlimov => {
                    "Кузьменко иногда знает о Климове больше, чем сам Климов."
                }
                DontRushWritingBugReports => {
                    "Не спеши слать гневные письма о багах:\
                    \nзагляни на mmheroes.chat.ru,\
                    \nможет быть, все уже в порядке!"
                }
                SerjSometimesAppearsInMausoleum => {
                    "Серж тоже иногда забегает в мавзолей."
                }
                DontOverstudyTopology => {
                    "Не переучи топологию, а то Подкорытов-младший не поймет."
                }
                YouCanGetAJobInTERKOM => "Можешь устроиться в ТЕРКОМ по знакомству.",
                GrishaWorksAtTERKOM => "Гриша работает ( ;*) ) в ТЕРКОМе.",
                YouCanEarnMoneyAtTERKOM => "В ТЕРКОМЕ можно заработать какие-то деньги.",
                GrishaSometimesAppearsInMausoleum => "Гриша иногда бывает в Мавзолее.",
                DontLikeTimetable => {
                    "Не нравится расписание? Подумай о чем-нибудь парадоксальном."
                }
                NiLPaysForHelpBut => "NiL дает деньги за помощь, но...",
                DontKnowWhenLinuxPortWillBeReady => {
                    "Честно, не знаю, когда будет готов порт под Linux..."
                }
                NeedNewFeaturesForMMHEROES => {
                    "Срочно! Нужны новые фишки для \"Зачетной недели\" !"
                }
                SendIdeasAndBugReports => {
                    "Пожелания, идеи, bug report'ы шлите на mmheroes@chat.ru !"
                }
                SendGreetingsToKostyaBulenkov => {
                    "Встретишь Костю Буленкова - передай ему большой привет!"
                }
                ThanksVanyaPavlik => "Большое спасибо Ване Павлику за mmheroes.chat.ru !",
            };
            writeln_colored!(WhiteBright, r, "\"{}\"", text);
            if diamond_leaves {
                writeln_colored!(White, r, "Diamond убегает по своим делам ...");
            }
            wait_for_any_key(r)
        }
    }
}

pub(crate) fn display_serj_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    interaction: SerjInteraction,
    serj_leaves: bool,
) -> WaitingState {
    use crate::logic::npc::serj::{SerjInteraction::*, SerjReply::*};
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    write_colored!(White, r, "Серж: ");
    let reply = match interaction {
        HaveSomeKefir => "На, глотни кефирчику.",
        IKnowWhereToCutInThePark => "Я знаю, где срезать в парке на физ-ре!",
        RandomReply(GuiMmheroes) => {
            "Помнится, когда-то была еще графическая версия mmHeroes..."
        }
        RandomReply(IWasABetaTester) => {
            "Я был бета-тестером первой версии mmHeroes (тогда еще CRWMM19)!"
        }
        RandomReply(HowGreatThatDiamondWroteANewVersion) => {
            "Как здорово, что Diamond написал новую версию!"
        }
        RandomReply(HaveYouAlreadyGotStipendFromPasha) => "Ты уже получил деньги у Паши?",
        RandomReply(TryEasyExamsFirst) => "Попробуй для начала легкие зачеты.",
        RandomReply(HaventYouPassedEnglishExam) => {
            "Ты еще не получил зачет по английскому?"
        }
        RandomReply(WantToRestAnywhereGetMoney) => {
            "Хочешь отдыхать, где угодно? Заимей деньги!"
        }
        RandomReply(MoneyCantBuyHappiness) => {
            "Не в деньгах счастье. Но они действуют успокаивающе."
        }
        RandomReply(AlwaysCrowdedOnVsemirnov) => "На Всемирнове всегда толпа народу.",
        RandomReply(VlaschenkoIsOriginalLady) => "Влащенко - дама весьма оригинальная.",
        RandomReply(WhenWillNewVersionBeReady) => {
            "Интересно, когда будет готова следующая версия?"
        }
        RandomReply(HealthInCafe) => {
            "Здоровье в кафе повышается в зависимости от наличия денег."
        }
        RandomReply(IfOnlyIKnewProxyAddress) => "Если бы я знал адрес хорошего proxy...",
        RandomReply(StarIsKaput) => {
            "STAR временно накрылся. Хорошо бы узнать адрес другого proxy..."
        }
        RandomReply(GrishaKnowsProxyAddress) => {
            "Я подозреваю, что Гриша знает адресок теркомовского proxy."
        }
        RandomReply(DiamondSpendsAllHisFreeTimeOnTheGame) => {
            "А Diamond все свободное время дописывает свою игрушку!"
        }
        RandomReply(NextTermTerekhovJrWillTeachCS) => {
            "В следующем семестре информатику будет вести Терехов-младший."
        }
        RandomReply(DiamondWantsToRewriteItInJava) => {
            "Diamond хочет переписать это все на Java."
        }
        RandomReply(MishaWillTellYouTheStrategy) => {
            "Миша проконсультирует тебя о стратегии."
        }
        RandomReply(TalkWithDiamondHeKnowsALot) => {
            "Поговори с Diamond'ом, он много ценного скажет."
        }
        RandomReply(FightUntilTheEnd) => "Борись до конца!",
        RandomReply(SometimesThereIsFreebieWithDubtsov) => {
            "У Дубцова иногда бывает халява."
        }
    };
    writeln_colored!(WhiteBright, r, "\"{}\"", reply);
    if serj_leaves {
        writeln_colored!(White, r, "Серж уходит куда-то по своим делам ...");
    }
    wait_for_any_key(r)
}

pub(crate) fn display_rai_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    interaction: &RaiInteraction,
) -> WaitingState {
    match interaction {
        RaiInteraction::Ignores(state) => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            writeln_colored!(White, r, "RAI не реагирует на твои позывы.");
        }
        RaiInteraction::PromptWillYouHelpMe(state) => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(9, 0);
            write_colored!(White, r, "RAI:");
            writeln_colored!(WhiteBright, r, "\"Ты мне поможешь?\"");
            scene_router::display_short_today_timetable(r, 11, state);
            r.move_cursor_to(11, 0);
            return dialog(r, available_actions);
        }
        RaiInteraction::TakeIt => {
            r.move_cursor_to(14, 0);
            writeln_colored!(MagentaBright, r, "\"Ах, так! Получай! Получай!\"");
            writeln_colored!(White, r, "RAI делает тебе больно ...");
        }
        RaiInteraction::YouHelped => {
            r.move_cursor_to(14, 0);
            writeln_colored!(Green, r, "Ты помог RAI.");
        }
        RaiInteraction::Fail => {
            r.move_cursor_to(14, 0);
            writeln_colored!(White, r, "Ничего не вышло.");
        }
    }
    wait_for_any_key(r)
}

pub(crate) fn display_nil_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    interaction: &NilInteraction,
) -> WaitingState {
    match interaction {
        NilInteraction::WillYouHelpMe(state) => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            writeln_colored!(
                CyanBright,
                r,
                "\"Маладой чилавек, вы мне не паможите решить задачу?"
            );
            writeln!(r, "А то я сигодня ни в зуб нагой ...\"");
            r.move_cursor_to(10, 0);
            return dialog(r, available_actions);
        }
        NilInteraction::RefusedToHelp => (),
        NilInteraction::ThanksHereIsYourMoney(reward) => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                YellowBright,
                r,
                "\"Ой, спасибо! Вот вам {reward} руб. за это...\"",
            );
        }
        NilInteraction::DidntWorkOut => {
            r.move_cursor_to(13, 0);
            writeln_colored!(MagentaBright, r, "У тебя ничего не вышло.")
        }
    }
    wait_for_any_key(r)
}

pub(crate) fn display_misha_interaction(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    interaction: &MishaInteraction,
) -> WaitingState {
    use MishaInteraction::*;
    use MishaReply::*;
    match interaction {
        PromptBugSquasher(state) | PromptTennis(state) | RandomReply(state, _) => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
        }
        _ => (),
    }
    match interaction {
        PromptBugSquasher(_) => {
            write_colored!(White, r, "Миша : ");
            writeln_colored!(WhiteBright, r, "\"Слушай, хватит мучаться! Прервись!");
            writeln!(r, "Давай в клоподавку сыграем!\"");
            r.move_cursor_to(11, 0);
            dialog(r, available_actions)
        }
        PlayedBugSquasherWithMisha => {
            r.move_cursor_to(14, 0);
            writeln_colored!(Green, r, "Ты сыграл с Мишей партию в клоподавку.");
            wait_for_any_key(r)
        }
        TooBad => {
            r.move_cursor_to(14, 0);
            writeln_colored!(WhiteBright, r, "\"Зря, очень зря!\"");
            wait_for_any_key(r)
        }
        PromptTennis(_) => {
            write_colored!(White, r, "Миша : ");
            writeln_colored!(
                WhiteBright,
                r,
                "\"Слушай, а ведь в ТЕРКОМе есть столик для тенниса. Сыграем?\""
            );
            r.move_cursor_to(11, 0);
            dialog(r, available_actions)
        }
        PlayedTennisWithMisha => {
            r.move_cursor_to(14, 0);
            writeln_colored!(Green, r, "Ты сыграл с Мишей в теннис.");
            wait_for_any_key(r)
        }
        NoWorries => {
            r.move_cursor_to(14, 0);
            writeln_colored!(WhiteBright, r, "\"Ничего, я на тебя не в обиде.\"");
            wait_for_any_key(r)
        }
        RandomReply(_, reply) => {
            write_colored!(White, r, "Миша:");
            let reply_text = match reply {
                TooBadNowhereToPlayBugSquasher => "Эх, жаль, негде сыграть в клоподавку!",
                AlwaysPayAttentionToHealth => "Всегда следи за здоровьем!",
                BrainLevelAffectsExamSuccess => {
                    "Мозги влияют на подготовку и сдачу зачетов."
                }
                TheMoreStaminaTheLessHealthYouSpend => {
                    "Чем больше выносливость, тем меньше здоровья ты тратишь."
                }
                TheMoreCharismaTheBetterRelationshipsWithPeople => {
                    "Чем больше твоя харизма, тем лучше у тебя отношения с людьми."
                }
                ImportanceOfCharacteristicAffectsGameStyle => {
                    "Важность конкретного качества сильно зависит от стиля игры."
                }
                CharismaHelpsGetAnything => {
                    "Харизма помогает получить что угодно от кого угодно."
                }
                TheMoreCharismaTheMoreYouAreApproached => {
                    "Чем больше харизма, тем чаще к тебе пристают."
                }
                TheLessStaminaTheMorePainfulStudyingIs => {
                    "Чем меньше выносливость, тем больнее учиться."
                }
                TheMoreBrainTheMoreEasyToPrepare => {
                    "Чем больше мозги, тем легче готовиться."
                }
                InternetSometimesImprovesBrain => {
                    "Сидение в Inet'e иногда развивает мозги."
                }
                IfTiredOfDyingTryAnotherStrategy => {
                    "Если тебе надоело умирать - попробуй другую стратегию."
                }
                WantFreebieGetCharisma => "Хочешь халявы - набирай харизму.",
                WantAchieveEverythingYourselfImproveBrain => {
                    "Хочешь добиться всего сам - развивай мозги."
                }
                InMausoleumKnowingWhenToStopIsImportant => {
                    "В \"Мавзолее\" важно знать меру..."
                }
                CharismaAndStaminaSaveFromPersonalityDisorder => {
                    "От раздвоения личности спасают харизма и выносливость."
                }
                YouGetStupidFromInteractingWithNil => {
                    "От любого общения с NiL ты тупеешь!"
                }
                GrishaCanHelpWithEmployment => "Гриша может помочь с трудоустройством.",
                NpcMovementsArePredictable => "Перемещения студентов предсказуемы.",
            };
            writeln_colored!(WhiteBright, r, "\"{reply_text}\"");
            wait_for_any_key(r)
        }
    }
}

pub(crate) fn display_djug_interaction<C: RendererRequestConsumer>(
    r: &mut Renderer<C>,
    state: &GameState,
) -> WaitingState {
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    write_colored!(White, r, "DJuG:");
    writeln_colored!(
        WhiteBright,
        r,
        "\"У Вас какой-то школьный метод решения задач...\""
    );
    wait_for_any_key(r)
}
