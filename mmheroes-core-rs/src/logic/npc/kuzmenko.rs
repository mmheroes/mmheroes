use crate::logic::{
    timetable, CharismaLevel, Duration, GameScreen, GameState, InternalGameState,
    Location, Subject, Time,
};
use crate::random;
use strum::VariantArray;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KuzmenkoInteraction {
    /// "Вы знаете, Климова можно найти в компьютерном классе 24-го мая с 10 по 11ч.."
    AdditionalComputerScienceExam {
        day_index: u8,
    },

    RandomReply(KuzmenkoReply),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum KuzmenkoReply {
    /// "... отформатировать дискету так, чтобы 1ый сектор был 5ым ..."
    FormatFloppy,

    /// "А Вы нигде не видели литературы по фильтрам в Windows?"
    FiltersInWindows,

    /// "... написать визуализацию байта на ассемблере за 11 байт ..."
    ByteVisualization,

    /// "У вас Олег Плисс ведет какие-нибудь занятия?"
    OlegPliss,

    /// "Bill Gates = must die = кабысдох (рус.)."
    BillGatesMustDie,

    /// "Вы читали журнал "Монитор"? Хотя вряд ли..."
    MonitorJournal,

    /// "Я слышал, что mmHeroes написана на BP 7.0."
    MmheroesBP7,

    /// "Записывайтесь на мой семинар по языку Си!"
    CSeminar,

    /// "На третьем курсе я буду вести у вас спецвычпрактикум."
    ThirdYear,

    /// "Интересно, когда они снова наладят STAR?"
    STAR,

    /// "Получите себе ящик rambler'e или на mail.ru !"
    GetYourselvesAnEmail,

    /// "А разве Терехов-старший ничего не рассказывает про IBM PC?"
    TerekhovSenior,
}

use KuzmenkoInteraction::*;

fn additional_exam_day_index(rng: &mut random::Rng, state: &mut GameState) -> Option<u8> {
    let tomorrow = state.current_day_index() + 1;
    let saturday = 5;
    if tomorrow > saturday {
        return None;
    }
    let mut additional_exam_day_idx: Option<u8> = None;
    for i in (tomorrow..=saturday).rev() {
        let day = state.timetable.day_mut(i);
        let has_enough_charisma = state.player.charisma > rng.random(CharismaLevel(18));
        let can_add_exam = day.exam(Subject::ComputerScience).is_none();
        if has_enough_charisma && can_add_exam {
            let exam_start_time = Time(rng.random_in_range(10..15));
            let exam_end_time = exam_start_time + Duration(rng.random_in_range(1..3));
            let additional_exam = timetable::Exam::new(
                Subject::ComputerScience,
                exam_start_time,
                exam_end_time,
                Location::ComputerClass,
            );
            day.add_exam(additional_exam);
            additional_exam_day_idx = Some(i);
            break;
        }
    }
    additional_exam_day_idx
}

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let new_screen = match additional_exam_day_index(&mut g.rng, state) {
        Some(additional_exam_day_idx)
            if state.additional_computer_science_exams() < 2 =>
        {
            // Баг в оригинальной реализации:
            // экран AdditionalComputerScienceExam показывается максимум дважды,
            // но в последующие разы экзамен тоже может добавиться в расписание,
            // просто Кузьменко об этом не скажет.
            state.add_additional_computer_science_exam();
            GameScreen::KuzmenkoInteraction(
                state.clone(),
                AdditionalComputerScienceExam {
                    day_index: additional_exam_day_idx,
                },
            )
        }
        _ => GameScreen::KuzmenkoInteraction(
            state.clone(),
            RandomReply(g.rng.random_variant()),
        ),
    };
    g.set_screen_and_wait_for_any_key(new_screen).await;
}
