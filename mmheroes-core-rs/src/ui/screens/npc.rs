use crate::logic::diamond::DiamondInteraction;
use crate::logic::npc::{
    grisha::GrishaInteraction, kolya::KolyaInteraction, kuzmenko::KuzmenkoInteraction,
    pasha::PashaInteraction, sasha::SashaInteraction,
};
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
            scene_router::display_short_today_timetable(
                r,
                11,
                state.current_day(),
                state.player(),
            );
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

    match interaction {
        PromptEmploymentAtTerkom
        | ProxyAddress
        | WantFreebie { .. }
        | FreebieComeToMe { .. }
        | FreebieExists { .. }
        | LetsOrganizeFreebieLoversClub { .. }
        | NoNeedToStudyToGetDiploma { .. }
        | YouStudiedDidItHelp { .. }
        | ThirdYearStudentsDontAttendLectures { .. }
        | TakeExampleFromKolya { .. }
        | HateLevTolstoy { .. }
        | DontGoToPDMI { .. }
        | NamesOfFreebieLovers { .. }
        | SitHereAndChill { .. }
        | NoNeedToTakeLectureNotes { .. }
        | CantBeExpelledInFourthYear { .. }
        | MechanicsHaveFreebie { .. } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::White, Color::Black);
        }
        CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => (),
    }

    let (reply, drink_beer, hour_pass) = match interaction {
        PromptEmploymentAtTerkom => {
            writeln_colored!(
                YellowBright,
                r,
                "\"А ты не хочешь устроиться в ТЕРКОМ? Может, кое-чего подзаработаешь...\""
            );
            writeln!(r);
            return dialog(r, available_actions);
        }
        CongratulationsYouAreNowEmployed => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Поздравляю, теперь ты можешь идти в \"контору\"!\""
            );
            return wait_for_any_key(r);
        }
        AsYouWantButDontOverstudy => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Как хочешь. Только смотри, не заучись там ...\""
            );
            return wait_for_any_key(r);
        }
        ProxyAddress => {
            writeln_colored!(
                White,
                r,
                "\"Кстати, я тут знаю один качественно работающий прокси-сервер...\""
            );
            writeln!(r);
            writeln_colored!(White, r, "Ты записываешь адрес. Вдруг пригодится?");
            return wait_for_any_key(r);
        }
        WantFreebie {
            drink_beer,
            hour_pass,
        } => ("Хочу халявы!", drink_beer, hour_pass),
        FreebieComeToMe {
            drink_beer,
            hour_pass,
        } => ("Прийди же, о халява!", drink_beer, hour_pass),
        FreebieExists {
            drink_beer,
            hour_pass,
        } => ("Халява есть - ее не может не быть.", drink_beer, hour_pass),
        LetsOrganizeFreebieLoversClub {
            drink_beer,
            hour_pass,
        } => (
            "Давай организуем клуб любетелей халявы!",
            drink_beer,
            hour_pass,
        ),
        NoNeedToStudyToGetDiploma {
            drink_beer,
            hour_pass,
        } => (
            "Чтобы получить диплом, учиться совершенно необязательно!",
            drink_beer,
            hour_pass,
        ),
        YouStudiedDidItHelp {
            drink_beer,
            hour_pass,
        } => (
            "Ну вот, ты готовился... Помогло это тебе?",
            drink_beer,
            hour_pass,
        ),
        ThirdYearStudentsDontAttendLectures {
            drink_beer,
            hour_pass,
        } => (
            "На третьем курсе на лекции уже никто не ходит. Почти никто.",
            drink_beer,
            hour_pass,
        ),
        TakeExampleFromKolya {
            drink_beer,
            hour_pass,
        } => ("Вот, бери пример с Коли.", drink_beer, hour_pass),
        HateLevTolstoy {
            drink_beer,
            hour_pass,
        } => (
            "Ненавижу Льва Толстого! Вчера \"Войну и мир\" <йк> ксерил...",
            drink_beer,
            hour_pass,
        ),
        DontGoToPDMI {
            drink_beer,
            hour_pass,
        } => ("А в ПОМИ лучше вообще не ездить!", drink_beer, hour_pass),
        NamesOfFreebieLovers {
            drink_beer,
            hour_pass,
        } => (
            "Имена главных халявчиков и алкоголиков висят на баобабе.",
            drink_beer,
            hour_pass,
        ),
        SitHereAndChill {
            drink_beer,
            hour_pass,
        } => (
            "Правильно, лучше посидим здесь и оттянемся!",
            drink_beer,
            hour_pass,
        ),
        NoNeedToTakeLectureNotes {
            drink_beer,
            hour_pass,
        } => (
            "Конспектировать ничего не надо. В мире есть ксероксы!",
            drink_beer,
            hour_pass,
        ),
        CantBeExpelledInFourthYear {
            drink_beer,
            hour_pass,
        } => (
            "А с четвертого курса вылететь уже почти невозможно.",
            drink_beer,
            hour_pass,
        ),
        MechanicsHaveFreebie {
            drink_beer,
            hour_pass,
        } => ("Вот у механиков - у них халява!", drink_beer, hour_pass),
    };

    write_colored!(White, r, "Гриша:");
    writeln_colored!(YellowBright, r, "\"{}\"", reply);
    if drink_beer {
        writeln_colored!(White, r, "И еще по пиву...");
    }
    if hour_pass {
        writeln_colored!(White, r, "И еще один час прошел в бесплодных разговорах...");
    }

    wait_for_any_key(r)
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
            scene_router::display_short_today_timetable(
                r,
                11,
                state.current_day(),
                state.player(),
            );
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
