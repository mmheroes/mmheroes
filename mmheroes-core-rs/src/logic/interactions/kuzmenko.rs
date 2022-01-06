use super::super::*;

pub(in crate::logic) fn interact(game: &mut Game, mut state: GameState) -> ActionVec {
    use npc::KuzmenkoInteraction::*;
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

    game.screen = match additional_exam_day_idx {
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

    wait_for_any_key()
}
