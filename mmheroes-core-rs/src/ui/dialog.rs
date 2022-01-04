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
        Action::RandomStudent => "Случайный студент",
        Action::CleverStudent => "Шибко умный",
        Action::ImpudentStudent => "Шибко наглый",
        Action::SociableStudent => "Шибко общительный",
        Action::GodMode => "GOD-режим",
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
        Action::GoToMausoleum => "Пойти в мавзолей",
        Action::GoToCafePUNK => "Сходить в кафе",
        Action::SurfInternet => "Провести 1 час в Inet'е",
        Action::PlayMMHEROES => "Поиграть в MMHEROES",
        Action::GoToProfessor => "Идти к преподу",
        Action::GoToWork => "Пойти в ТЕРКОМ, поработать",
        Action::LookAtBaobab => "Посмотреть на баобаб",
        Action::OrderCola => "Стакан колы за 4 р.",
        Action::OrderSoup => "Суп, 6 р. все удовольствие",
        Action::OrderBeer => "0,5 пива за 8 р.",
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
        Action::WhatToDoAtAll => " А что вообще делать? ",
        Action::AboutScreen => " Об экране            ",
        Action::WhereToGoAndWhy => " Куда и зачем ходить? ",
        Action::AboutProfessors => " О преподавателях     ",
        Action::AboutCharacters => " О персонажах         ",
        Action::AboutThisProgram => " Об этой программе    ",
        Action::ThanksButNothing => " Спасибо, ничего      ",
        Action::AnyKey => panic!("Action {:?} cannot be used in a dialog", action),
    };
    set_color(r, Color::CyanBright);
    write!(r, "{}", option_name)
}
