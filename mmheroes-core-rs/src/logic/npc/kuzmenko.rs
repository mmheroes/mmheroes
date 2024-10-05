use super::super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KuzmenkoInteraction {
    /// "Вы знаете, Климова можно найти в компьютерном классе 24-го мая с 10 по 11ч.."
    AdditionalComputerScienceExam { day_index: usize },

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

pub(in crate::logic) fn interact(
    game: &mut InternalGameState,
    mut state: GameState,
) -> ActionVec {
    let tomorrow = state.current_day_index + 1;
    let saturday = 5;
    let mut additional_exam_day_idx: Option<usize> = None;
    if tomorrow <= saturday {
        for i in (tomorrow..=saturday).rev() {
            let day = &mut state.timetable.days_mut()[i];
            let has_enough_charisma =
                state.player.charisma > game.rng.random(CharismaLevel(18));
            let can_add_exam = day.exam(Subject::ComputerScience).is_none();
            if has_enough_charisma && can_add_exam {
                let exam_start_time = Time(10u8 + game.rng.random(5));
                let exam_end_time = exam_start_time + Duration(1i8 + game.rng.random(2));
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
    }

    let new_screen = match additional_exam_day_idx {
        // Проверка на `state.additional_computer_science_exams() < 2` должна быть
        // раньше — до модификации расписания.
        // Иначе получается, что при достаточной харизме Кузьменко может
        // добавить экзамены по информатике на каждый день.
        // Баг в оригинальной реализации. Возможно, стоит исправить, но
        // пока не буду.
        Some(additional_exam_day_idx)
            if state.additional_computer_science_exams() < 2 =>
        {
            state.add_additional_computer_science_exam();
            GameScreen::KuzmenkoInteraction(
                state,
                AdditionalComputerScienceExam {
                    day_index: additional_exam_day_idx,
                },
            )
        }
        _ => {
            let replies = [
                FormatFloppy,
                FiltersInWindows,
                ByteVisualization,
                OlegPliss,
                BillGatesMustDie,
                MonitorJournal,
                MmheroesBP7,
                CSeminar,
                ThirdYear,
                STAR,
                GetYourselvesAnEmail,
                TerekhovSenior,
            ];
            GameScreen::KuzmenkoInteraction(state, *game.rng.random_element(&replies))
        }
    };
    game.set_screen(new_screen);

    wait_for_any_key()
}
