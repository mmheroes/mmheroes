use crate::logic::*;
use crate::ui::*;

pub(in crate::ui) fn display_dialog(
    r: &mut Renderer<impl RendererRequestConsumer>,
    start: (Line, Column),
    current_choice: Option<u8>,
    actions: &[Action],
) {
    for (i, action) in actions.iter().enumerate() {
        r.move_cursor_to(start.0 + i as Line, start.1);
        display_action(r, *action, false)
    }
    if let Some(current_choice) = current_choice {
        r.move_cursor_to(start.0 + current_choice, start.1);
        display_action(r, actions[current_choice as usize], true);
    }
    r.flush();
}

pub(in crate::ui) fn dialog(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
) -> WaitingState {
    let start = r.get_cursor_position();
    let current_choice = 0;
    display_dialog(r, start, Some(current_choice), available_actions);
    WaitingState::Dialog {
        current_choice,
        start,
    }
}

fn display_action<RequestConsumer: RendererRequestConsumer>(
    r: &mut Renderer<RequestConsumer>,
    action: Action,
    chosen: bool,
) {
    let set_color = |r: &mut Renderer<RequestConsumer>, color: Color| {
        if chosen {
            r.set_color(Color::Black, Color::White);
        } else {
            r.set_color(color, Color::Black)
        }
    };
    let option_name = match action {
        Action::Yes => "Да",
        Action::No => "Нет",
        Action::InteractWithClassmate(classmate) => {
            set_color(r, Color::YellowBright);
            write!(r, "{}", classmate_name(classmate));
            return;
        }
        Action::Exam(subject) => {
            if subject == Subject::ComputerScience {
                set_color(r, Color::YellowBright);
                write!(r, "{}", professor_name(subject));
                return;
            } else {
                professor_name(subject)
            }
        }
        Action::DontGoToProfessor => "Ни к кому",
        Action::SelectPlayStyle(actions::PlayStyle::RandomStudent) => "Случайный студент",
        Action::SelectPlayStyle(actions::PlayStyle::CleverStudent) => "Шибко умный",
        Action::SelectPlayStyle(actions::PlayStyle::ImpudentStudent) => "Шибко наглый",
        Action::SelectPlayStyle(actions::PlayStyle::SociableStudent) => {
            "Шибко общительный"
        }
        Action::SelectPlayStyle(actions::PlayStyle::GodMode) => "GOD-режим",
        Action::Study => "Готовиться",
        Action::DoStudy {
            subject,
            lecture_notes_available,
        } => {
            set_color(r, Color::CyanBright);
            write!(r, "{}", subject_name(subject));
            if lecture_notes_available {
                write!(r, " (к)")
            }
            return;
        }
        Action::DontStudy => "Ни к чему",
        Action::UseLectureNotes(_) => "Воспользуюсь конспектом",
        Action::DontUseLectureNotes(_) => "Буду учиться, как умею",
        Action::RequestLectureNotesFromSasha(subject) => subject_name(subject),
        Action::DontNeedAnythingFromSasha => "Ничего не надо",
        Action::ViewTimetable => "Посмотреть расписание",
        Action::Rest => "Отдыхать",
        Action::GoToBed => "Лечь спать",
        Action::GoFromPunkToDorm => "Пойти в общагу",
        Action::GoFromDormToPunk => "Пойти на факультет",
        Action::GoFromMausoleumToDorm => "Идти в общагу",
        Action::RestByOurselvesInMausoleum => "Расслабляться будем своими силами.",
        Action::NoRestIsNoGood => "Нет, отдыхать - это я зря сказал.",
        Action::GoFromMausoleumToPunk => "Идти в ПУНК",
        Action::GoToComputerClass => "Пойти в компьютерный класс",
        Action::LeaveComputerClass => "Покинуть класс",
        Action::GoToPDMI => "Поехать в ПОМИ",
        Action::GoToPUNKFromPDMI => "Поехать в ПУНК",
        Action::GatecrashTrain => "Ехать зайцем",
        Action::BuyRoundtripTrainTicket => {
            set_color(r, Color::CyanBright);
            return write!(
                r,
                "Честно заплатить {} руб. за билет в оба конца",
                Money::roundtrip_train_ticket_cost()
            );
        }
        Action::GoToMausoleum => "Пойти в мавзолей",
        Action::GoToCafePUNK => "Сходить в кафе",
        Action::SurfInternet => "Провести 1 час в Inet'е",
        Action::PlayMMHEROES => "Поиграть в MMHEROES",
        Action::GoToProfessor => "Идти к преподу",
        Action::GoToWork => "Пойти в ТЕРКОМ, поработать",
        Action::LookAtBaobab => "Посмотреть на баобаб",
        Action::LookAtBulletinBoard => "Посмотреть на доску объявлений",
        Action::OrderCola => {
            set_color(r, Color::CyanBright);
            write!(r, "Стакан колы за {} р.", Money::cola_cost());
            return;
        }
        Action::OrderSoup => {
            set_color(r, Color::CyanBright);
            write!(r, "Суп, {} р. все удовольствие", Money::soup_cost());
            return;
        }
        Action::OrderBeer => {
            set_color(r, Color::CyanBright);
            write!(r, "0,5 пива за {} р.", Money::beer_cost());
            return;
        }
        Action::OrderTea => {
            set_color(r, Color::CyanBright);
            write!(r, "Чай за {} р.", Money::tea_cost());
            return;
        }
        Action::OrderCake => {
            set_color(r, Color::CyanBright);
            write!(r, "Кекс за {} р.", Money::cake_cost());
            return;
        }
        Action::OrderTeaWithCake => {
            set_color(r, Color::CyanBright);
            write!(r, "Чай и выпечку, {} р.", Money::tea_with_cake_cost());
            return;
        }
        Action::RestInCafePUNK => "Просто посижу с приятелями.",
        Action::ShouldntHaveComeToCafePUNK => "Я вообще зря сюда зашел.",
        Action::RestInCafePDMI => "Пойти в кафе",
        Action::AcceptEmploymentAtTerkom => "Да, мне бы не помешало.",
        Action::DeclineEmploymentAtTerkom => "Нет, я лучше поучусь уще чуток.",
        Action::IAmDone => {
            set_color(r, Color::BlueBright);
            write!(r, "С меня хватит!");
            return;
        }
        Action::NoIAmNotDone => "Нет, не хочу!",
        Action::IAmCertainlyDone => "Я же сказал: с меня хватит!",
        Action::WantToTryAgain => "ДА!!! ДА!!! ДА!!!",
        Action::DontWantToTryAgain => "Нет... Нет... Не-э-эт...",
        Action::WhatToDo => {
            set_color(r, Color::BlueBright);
            write!(r, "ЧТО ДЕЛАТЬ ???");
            return;
        }
        Action::Help(actions::HelpAction::WhatToDoAtAll) => " А что вообще делать? ",
        Action::Help(actions::HelpAction::AboutScreen) => " Об экране            ",
        Action::Help(actions::HelpAction::WhereToGoAndWhy) => " Куда и зачем ходить? ",
        Action::Help(actions::HelpAction::AboutProfessors) => " О преподавателях     ",
        Action::Help(actions::HelpAction::AboutCharacters) => " О персонажах         ",
        Action::Help(actions::HelpAction::AboutThisProgram) => " Об этой программе    ",
        Action::Help(actions::HelpAction::ThanksButNothing) => " Спасибо, ничего      ",
        Action::AnyKey => panic!("Action {:?} cannot be used in a dialog", action),
    };
    set_color(r, Color::CyanBright);
    write!(r, "{}", option_name)
}
