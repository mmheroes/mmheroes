use crate::logic::scene_router::exams::{BenefitsOfRunning, ExamIntro};
use crate::logic::{Action, GameState, Subject, SUBJECTS};
use crate::ui::dialog::dialog;
use crate::ui::renderer::{Renderer, RendererRequestConsumer};
use crate::ui::screens::scene_router;
use crate::ui::{classmate_name, professor_name, Color, WaitingState};

pub(in crate::ui) fn display_exam_intro(
    r: &mut Renderer<impl RendererRequestConsumer>,
    intro: ExamIntro,
) -> WaitingState {
    r.clear_screen();
    match intro {
        ExamIntro::AlgebraPunkBigCrowdedRoom => {
            writeln_colored!(
                Green,
                r,
                "\
Болшая, рассчитанная на поток аудитория кажется забитой народом.
Здесь присутствуют не только твои одногруппники,
но и какие-то не очень знакомые тебе люди
(кажется, прикладники со второго курса).
За столом около доски сидит М. А. Всемирнов
и принимает зачет у студентов.
Ты решаешь не терять времени даром и присоединиться к остальным."
            );
        }
        ExamIntro::AlgebraPunkWrongRoom => {
            writeln_colored!(
                Green,
                r,
                "\
Ты заходишь в небольшую аудиторию, забитую народом.
Около доски сидит весьма своеобразный преподаватель.
Сие своебразие проявляется, в первую очередь, значком
с надписью: \"НЕ СТРЕЛЯЕЙТЕ В ПРЕПОДА - ОБУЧАЕТ КАК УМЕЕТ\".
\"А вы к кому? Максим Александрович в аудитории напротив!\"\
Похоже, ты не туда попал. Ты извиняешься и идешь к Всемирнову."
            )
        }
        ExamIntro::AlgebraPdmi => {
            writeln_colored!(
                Red,
                r,
                "\
Маленький кабинет в ПОМИ заполнен людьми.
И, как ни странно, почти все они хотят одного и того же.
Похоже, ты тоже хочешь именно этого -
РАЗДЕЛАТЬСЯ НАКОНЕЦ С ЗАЧЕТОМ ПО АЛГЕБРЕ!"
            )
        }
        ExamIntro::Calculus => {
            writeln_colored!(
                CyanBright,
                r,
                "\
В обычной \"групповой\" аудитории сидят около 15 человек.
В центре их внимания находится Е.С. Дубцов,
принимающий зачет по матанализу.
Ты получаешь задание и садишься за свободную парту."
            );
        }
        ExamIntro::GeometryPunk => {
            writeln_colored!(
                BlueBright,
                r,
                "\
Небольшая, полупустая аудитория.
И доска, и стены, и, похоже, даже пол
исписаны различными геометрическими утверждениями.
В центре всего этого хаоса находится
(или, скорее, постоянно перемещается)
Подкорытов-младший.
Ты радуешься, что смог застать его на факультете!"
            )
        }
        ExamIntro::GeometryPdmi => {
            writeln_colored!(
                White,
                r,
                "\
В небольшом ПОМИшном кабинете собралось человек 10 студентов.
Кроме них, в комнате ты видишь Подкорытова-младшего,
а также - полного седоволосого лысеющего господина,
издающего характерные пыхтящие звуки.
Ты надеешься, что все это скоро кончится..."
            )
        }
        ExamIntro::ComputerScience => {
            writeln_colored!(
                White,
                r,
                "Климов А.А. сидит и тоскует по халявному Inet'у."
            );
        }
        ExamIntro::English => {
            writeln_colored!(
                YellowBright,
                r,
                "\
На третьем этаже учебного корпуса Мат-Меха
в одной из аудиторий, закрепленных за кафедрой иностранных языков,
расположилась Н.П. Влащенко.
Стены кабинета выглядят как-то странно.
Рядом с небольшой доской висит изображение Эйфелевой башни,
чуть дальше - странное изображение,
обладающее непостижимым метафизическим смыслом.
Похоже, сейчас ты будешь сдавать зачет по английскому."
            );
        }
        ExamIntro::PhysicalEducation(lecture) => {
            r.set_color(Color::WhiteBright, Color::Black);
            if let Some(lecture_topic) = lecture {
                let topic_text = match lecture_topic {
                    BenefitsOfRunning::NationalEconomy => "для народного хозяйства",
                    BenefitsOfRunning::PersonalLife => "для личной жизни",
                    BenefitsOfRunning::ScientificResearch => "для научной работы",
                    BenefitsOfRunning::BuildingCommunism => {
                        "для коммунистического строительства"
                    }
                    BenefitsOfRunning::StudyAndEntertainment => "для учебы и досуга",
                    BenefitsOfRunning::EscapingFromInspectors => {
                        "для спасения от контроллеров"
                    }
                };
                writeln!(
                    r,
                    "\
Альбинский проводит лекцию о пользе бега
{}.

Похоже, он, как всегда, немного увлекся.
Немного в нашем случае - 1 час.
",
                    topic_text
                );
            }
            writeln!(
                r,
                "\
Альбинский просит тебя замерить пульс.
Назвав первое пришедшее в замученную математикой голову число,
ты отправляешься мотать круги в парке,
в котором, вообще-то, \"запрещены спортивные мероприятия\"."
            )
        }
    }
    writeln!(r, "...");
    r.flush();
    WaitingState::PressAnyKey
}

pub(in crate::ui) fn display_exam_scene(
    r: &mut Renderer<impl RendererRequestConsumer>,
    state: &GameState,
    available_actions: &[Action],
    subject: Subject,
) -> WaitingState {
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    scene_router::display_short_today_timetable(
        r,
        11,
        state.current_day(),
        state.player(),
    );
    let problems_done = state.player().status_for_subject(subject).problems_done();
    let problems_required = SUBJECTS[subject].required_problems();
    if problems_done >= problems_required {
        r.move_cursor_to(6, 0);
        write_colored!(Green, r, "У вас все зачтено, можете быть свободны.");
    }
    r.move_cursor_to(7, 0);
    writeln_colored!(
        YellowBright,
        r,
        "Сейчас тебя истязает {}.",
        professor_name(subject)
    );

    let mut num_classmates_here = state.classmates().filter_by_exam(subject).count();
    if num_classmates_here > 0 {
        write_colored!(
            White,
            r,
            "Кроме тебя, здесь еще {} ",
            if num_classmates_here == 1 {
                "сидит"
            } else {
                "сидят"
            }
        );
        for classmate_info in state.classmates().filter_by_exam(subject) {
            write!(r, "{}", classmate_name(classmate_info.classmate()));
            num_classmates_here -= 1;

            // TODO: Сделать перенос строки не так топорно, добавить тесты
            if r.get_cursor_position().1 > 70 {
                writeln!(r);
            }
            if num_classmates_here == 0 {
                writeln!(r, ".")
            } else if num_classmates_here == 1 {
                write!(r, " и ")
            } else {
                write!(r, ", ")
            }
        }
    }

    r.move_cursor_to(6, 0);
    if problems_done == 0 {
        writeln_colored!(White, r, "У тебя еще ничего не зачтено.")
    } else if problems_done < problems_required {
        write_colored!(White, r, "Зачтено ");
        write_colored!(WhiteBright, r, "{}", problems_done);
        write_colored!(White, r, " задач из ");
        writeln_colored!(WhiteBright, r, "{}", problems_required);
    } else {
        writeln_colored!(Green, r, "У тебя уже все зачтено.")
    }
    r.move_cursor_to(11, 0);
    dialog(r, available_actions)
}
