use crate::logic::sleep::{
    DjugDream, DjugQuote, DreamScreen, StupidDream, StupidDreamScenario,
    StupidDreamSubject,
};
use crate::ui::renderer::Renderer;
use crate::ui::*;

pub(in crate::ui) fn display_dont_want_to_sleep(
    r: &mut Renderer<impl RendererRequestConsumer>,
) -> WaitingState {
    r.move_cursor_to(21, 0);
    write_colored!(White, r, "Тебя чего-то не тянет по-спать...");
    wait_for_any_key(r)
}

pub(in crate::ui) fn display_dreams(
    r: &mut Renderer<impl RendererRequestConsumer>,
    dream: &DreamScreen,
) -> WaitingState {
    match dream {
        DreamScreen::Stupid(dream) => stupid_dream(r, dream),
        DreamScreen::Djug(dream) => djug_dream(r, dream),
        DreamScreen::SubjectRelated(last_exam) => subject_related_dream(r, *last_exam),
    }
}

fn stupid_dream(
    r: &mut Renderer<impl RendererRequestConsumer>,
    dream: &StupidDream,
) -> WaitingState {
    match dream {
        StupidDream::Phase1(subject, scenario) => {
            r.clear_screen();
            r.set_color(Color::MagentaBright, Color::Black);
            let subject_text = match subject {
                StupidDreamSubject::PinkElephants => {
                    "Розовые слоники с блестящими крылышками"
                }
                StupidDreamSubject::GreenMen => "Зеленые человечки с длинными антеннами",
                StupidDreamSubject::Sheep => "Овечки с ослепительно-белой шерстью",
            };
            writeln!(r, "{subject_text}");
            writeln!(r, "сидят с окосевшими глазами в Мавзолее");
            let scenario_text = match scenario {
                StupidDreamScenario::ComputingDeterminant => {
                    "считают определитель матрицы 10 на 10"
                }
                StupidDreamScenario::ComputingJordanMatrix => {
                    "ищут Жорданову форму матрицы"
                }
                StupidDreamScenario::RaiseMatrixToPower => {
                    "возводят матрицы в 239-ю степень"
                }
                StupidDreamScenario::SolvingLinearSystem => {
                    "решают линейную систему уравнений с параметрами"
                }
                StupidDreamScenario::ProvingIrreducibilityOfPolynomial => {
                    "доказывают неприводимость многочлена 10-й степени над Z"
                }
                StupidDreamScenario::ProvingConvergenceOfIntegral => {
                    "доказывают сходимость неопределенного интеграла с параметрами"
                }
                StupidDreamScenario::ComputingSumOfSeries => {
                    "считают сумму ряда с параметрами"
                }
                StupidDreamScenario::Differentiate => {
                    "дифференцируют, дифференцируют, дифференцирую"
                }
                StupidDreamScenario::TakingIntegrals => "берут интергалы не отдают их",
                StupidDreamScenario::SolvingMathematicalProblems => {
                    "решают задачи по математической болтологии"
                }
            };
            writeln!(r, "и {scenario_text}");
        }
        StupidDream::Phase2 => {
            writeln!(r);
            writeln!(r, "Господи! Ну и присниться же такое!");
            writeln!(r, "За то теперь ты точно знаешь,");
            writeln!(r, "что снится студентам-математикам,");
            writeln!(r, "когда они вне кондиции");
        }
    }
    writeln!(r, "...");
    r.flush();
    WaitingState::PressAnyKey
}

fn djug_dream(
    r: &mut Renderer<impl RendererRequestConsumer>,
    dream: &DjugDream,
) -> WaitingState {
    match dream {
        DjugDream::Phase1 => {
            r.clear_screen();
            r.set_color(Color::MagentaBright, Color::Black);
            writeln!(r, "\"Здравствуйте!\" ...");
        }
        DjugDream::Phase2 => {
            writeln!(r, "Оно большое ...");
        }
        DjugDream::Phase3 => {
            writeln!(r, "Оно пыхтит! ...");
        }
        DjugDream::Phase4 => {
            writeln!(r, "Оно медленно ползет прямо на тебя!!! ...");
        }
        DjugDream::Phase5(quote) => {
            writeln!(r, "Оно говорит человеческим голосом:");
            r.set_color(Color::White, Color::Black);
            match quote {
                DjugQuote::DeathButton => {
                    writeln!(r, "\"Молодой человек. Когда-нибудь Вы вырастете");
                    writeln!(r, "и будете работать на большой машине.");
                    writeln!(r, "Вам надо будет нажать кнопку жизни,");
                    writeln!(r, "а Вы нажмете кнопку смерти ...\"");
                }
                DjugQuote::HowManyDevilsFitANeedleTip => {
                    writeln!(r, "\"Это в средневековье ученые спорили,");
                    writeln!(r, "сколько чертей может поместиться");
                    writeln!(r, "на кончике иглы...\"");
                }
                DjugQuote::DifferentWaysOfSolvingProblems => {
                    writeln!(r, "\"Задачи можно решать по-разному.");
                    writeln!(r, "Можно устно, можно на бумажке,");
                    writeln!(r, "можно - играя в крестики-нолики...");
                    writeln!(r, "А можно - просто списать ответ в конце задачника!\"");
                }
            }
            writeln_colored!(MagentaBright, r, "...");
        }
        DjugDream::Phase6 => {
            writeln!(r);
            writeln!(r, "Уффф... Что-то сегодня опять какие-то гадости снятся.");
            writeln!(r, "Все, пора завязывать с этим. Нельзя так много учиться.");
        }
    }
    r.flush();
    WaitingState::PressAnyKey
}

fn subject_related_dream(
    r: &mut Renderer<impl RendererRequestConsumer>,
    last_exam: Subject,
) -> WaitingState {
    r.clear_screen();
    r.set_color(Color::MagentaBright, Color::Black);
    match last_exam {
        Subject::AlgebraAndNumberTheory => {
            writeln!(r, "Ты слышишь мягкий, ненавязчивый голос:");
            writeln!(r, "\"А Вы действительно правильно выбрали");
            writeln!(r, " себе специальность?\"");
        }
        Subject::Calculus => {
            writeln!(r, "\"Интеграл...\"");
            writeln!(r, "\"Какой интеграл?\"");
            writeln!(r, "\"Да вот же он, мы его только что стерли!\"");
        }
        Subject::GeometryAndTopology => {
            writeln!(r, "\"Вы, конечно, великий парильщик.");
            writeln!(r, " Но эту задачу я Вам засчитаю.\"");
        }
        Subject::ComputerScience => {
            writeln!(r, "\"А что, у нас сегодня разве аудиторное занятие?\"");
        }
        Subject::English => {
            writeln!(r, "\"Well, last time I found a pencil left by one of you.");
            writeln!(r, " I will return it to the owner, if he or she");
            writeln!(r, " can tell me some nice and pleasant words.");
            writeln!(r, " I am a lady, not your computer!\"");
        }
        Subject::PhysicalEducation => {
            writeln!(
                r,
                "\"В следующем семестре вы должны будете написать реферат"
            );
            writeln!(
                r,
                " на тему \"Бег в мировой литературе\". В качестве первоисточника"
            );
            writeln!(r, " можете взять одноименный роман Булгакова.\"");
        }
    }
    writeln!(r);
    writeln!(
        r,
        "Ну все, похоже, заучился - если преподы по ночам снятся..."
    );
    r.flush();
    WaitingState::PressAnyKey
}

pub(in crate::ui) fn display_cant_stay_awake(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
) -> WaitingState {
    r.clear_screen();
    screens::scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    writeln_colored!(White, r, "Тебя неумолимо клонит ко сну ...");
    wait_for_any_key(r)
}
