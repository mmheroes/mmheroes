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
        Action::YesOrNo(actions::YesOrNoAction::Yes) => "Да",
        Action::YesOrNo(actions::YesOrNoAction::No) => "Нет",
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
        Action::UseLectureNotes(actions::UseLectureNotesAction::Yes) => {
            "Воспользуюсь конспектом"
        }
        Action::UseLectureNotes(actions::UseLectureNotesAction::No) => {
            "Буду учиться, как умею"
        }
        Action::RequestLectureNotesFromSasha(subject) => subject_name(subject),
        Action::DontNeedAnythingFromSasha => "Ничего не надо",
        Action::ViewTimetable => "Посмотреть расписание",
        Action::Rest => "Отдыхать",
        Action::GoToBed => "Лечь спать",
        Action::InvitationFromNeighbor(actions::InvitationFromNeighborAction::Accept) => {
            "\"Угу, я сейчас!!!\""
        }
        Action::InvitationFromNeighbor(actions::InvitationFromNeighborAction::Deny) => {
            "\"Не, извини, мне готовиться надо...\""
        }
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
        Action::TrainToPDMIGatecrash => "Ехать зайцем",
        Action::TrainToPDMIBuyTicket => {
            set_color(r, Color::CyanBright);
            return write!(
                r,
                "Честно заплатить {} руб. за билет в оба конца",
                Money::roundtrip_train_ticket_cost()
            );
        }
        Action::TrainFromPDMIGatecrash => "Нет, не будем",
        Action::TrainFromPDMIBuyTicket => "Да, будем",
        Action::GoToMausoleum => "Пойти в мавзолей",
        Action::GoToCafePUNK => "Сходить в кафе",
        Action::SurfInternet => "Провести 1 час в Inet'е",
        Action::PlayMMHEROES => "Поиграть в MMHEROES",
        Action::EarnAtTerkom => "Сидеть и зарабатывать деньги",
        Action::SurfInternetAtTerkom => "Посидеть часок в Inet'e",
        Action::ExitTerkom => "Выйти отсюда на \"свежий воздух\"",
        Action::GoToProfessor => "Идти к преподу",
        Action::SufferMore => "Мучаться дальше",
        Action::ExitExam => "Бросить это дело",
        Action::ContinueSufferingWithExamInTrain(
            actions::ContinueSufferingWithExamInTrainAction::WantToSufferMore,
        ) => "Да, я хочу еще помучаться",
        Action::ContinueSufferingWithExamInTrain(
            actions::ContinueSufferingWithExamInTrainAction::NoThanks,
        ) => "Ну уж нет, спасибо!",
        Action::BaltiyskiyRailwayStation(
            actions::BaltiyskiyRailwayStationAction::GoToPUNK,
        ) => "Домой, в ПУНК!",
        Action::BaltiyskiyRailwayStation(
            actions::BaltiyskiyRailwayStationAction::GoToPDMI,
        ) => "Хочу в ПОМИ!",
        Action::Rai(actions::RaiAction::YesOfCourse) => "\"Да, конечно\"",
        Action::Rai(actions::RaiAction::NoSorry) => "\"Нет, извини...\"",
        Action::Nil(actions::NilAction::YesOfCourse) => "\"Да, конечно\"",
        Action::Nil(actions::NilAction::MaybeNextTime) => "\"Извини, в другой раз\"",
        Action::BugSquasher(actions::BugSquasherAction::LetsGo) => "\"Давай!\"",
        Action::BugSquasher(actions::BugSquasherAction::NoIWontPlay) => {
            "\"Нет, не буду я в клоподавку ...\""
        }
        Action::Tennis(actions::TennisAction::Sure) => "\"Обязательно!\"",
        Action::Tennis(actions::TennisAction::SorryMaybeLater) => "\"Извини, потом.\"",
        Action::HelpFromAndrew(
            actions::HelpFromAndrewAction::YesAmIWorseThanEveryoneElse,
        ) => "Да, чем я хуже других?",
        Action::HelpFromAndrew(actions::HelpFromAndrewAction::IWillDoItMyself) => {
            "Нет, я уж как-нибудь сам..."
        }
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
            write!(r, "Чай за {} р.", Money::drink_cost());
            return;
        }
        Action::OrderCake => {
            set_color(r, Color::CyanBright);
            write!(r, "Кекс за {} р.", Money::pastry_cost());
            return;
        }
        Action::OrderTeaWithCake => {
            set_color(r, Color::CyanBright);
            write!(r, "Чай и выпечку, {} р.", Money::drink_with_pastry_cost());
            return;
        }
        Action::RestInCafePUNK => "Просто посижу с приятелями.",
        Action::ShouldntHaveComeToCafePUNK => "Я вообще зря сюда зашел.",
        Action::GoToCafePDMI => "Пойти в кафе",
        Action::OrderCoffee => {
            set_color(r, Color::CyanBright);
            write!(r, "Кофе за {} р.", Money::drink_cost());
            return;
        }
        Action::OrderPastry => {
            set_color(r, Color::CyanBright);
            write!(r, "Корж за {} р.", Money::pastry_cost());
            return;
        }
        Action::OrderCoffeeWithPastry => {
            set_color(r, Color::CyanBright);
            write!(r, "Кофе и выпечку, {} р.", Money::drink_with_pastry_cost());
            return;
        }
        Action::RestInCafePDMI => "Ничего, просто просидеть здесь часок.",
        Action::LeaveCafePDMI => "Совсем ничего. Бывает.",
        Action::MmheroesFloppy(actions::MmheroesFloppyAction::WantToTestNewMMHEROES) => {
            "ДА, КОНЕЧНО, ОЧЕНЬ ХОЧУ!"
        }
        Action::MmheroesFloppy(
            actions::MmheroesFloppyAction::DontWantToTestNewMMHEROES,
        ) => "Нет, у меня нет на это времени...",
        Action::TerkomEmployment(actions::TerkomEmploymentAction::Accept) => {
            "Да, мне бы не помешало."
        }
        Action::TerkomEmployment(actions::TerkomEmploymentAction::Decline) => {
            "Нет, я лучше поучусь уще чуток."
        }
        Action::NpcApproach(actions::NpcApproachAction::Ignore) => {
            "Пытаться игнорировать"
        }
        Action::NpcApproach(actions::NpcApproachAction::TalkToClassmate(classmate)) => {
            classmate_name(classmate)
        }
        Action::IAmDone => {
            set_color(r, Color::BlueBright);
            write!(r, "С меня хватит!");
            return;
        }
        Action::GameEnd(actions::GameEndAction::NoIAmNotDone) => "Нет, не хочу!",
        Action::GameEnd(actions::GameEndAction::IAmCertainlyDone) => {
            "Я же сказал: с меня хватит!"
        }
        Action::TryAgain(actions::TryAgainAction::WantToTryAgain) => "ДА!!! ДА!!! ДА!!!",
        Action::TryAgain(actions::TryAgainAction::DontWantToTryAgain) => {
            "Нет... Нет... Не-э-эт..."
        }
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
