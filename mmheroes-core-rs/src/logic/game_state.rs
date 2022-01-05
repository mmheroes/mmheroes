use super::*;
use core::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct GameState {
    pub(in crate::logic) player: Player,
    pub(in crate::logic) current_day_index: usize,
    pub(in crate::logic) current_time: Time,
    pub(in crate::logic) timetable: timetable::Timetable,
    pub(in crate::logic) location: Location,
    pub(in crate::logic) classmates: Classmates,

    /// Младшие 5 бит — количество дополнительных экзаменов по информатике
    /// Старшие 3 бита — флаги о наличии у Саши конспектов по АиТЧ,
    /// матанализу и геометрии.
    bits: u8,
}

impl GameState {
    pub(in crate::logic) fn new(
        player: Player,
        timetable: timetable::Timetable,
        location: Location,
    ) -> GameState {
        GameState {
            player,
            current_day_index: 0,
            current_time: Time(8),
            timetable,
            location,
            classmates: Classmates::new(),
            bits: 0b111_00000,
        }
    }

    pub fn current_day(&self) -> &Day {
        &self.timetable.days()[self.current_day_index]
    }

    pub fn current_time(&self) -> Time {
        self.current_time
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn timetable(&self) -> &timetable::Timetable {
        &self.timetable
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn classmates(&self) -> &Classmates {
        &self.classmates
    }

    pub(in crate::logic) fn additional_computer_science_exams(&self) -> u8 {
        self.bits & 0b000_11111
    }

    pub(in crate::logic) fn add_additional_computer_science_exam(&mut self) {
        let old_count = self.additional_computer_science_exams();
        assert!(
            old_count <= 31,
            "additional_computer_science_exams overflow"
        );
        self.bits &= 0b111_00000;
        self.bits |= old_count + 1;
    }

    pub(in crate::logic) fn sasha_has_lecture_notes(&self, subject: Subject) -> bool {
        assert!(SUBJECTS_WITH_LECTURE_NOTES.contains(&subject));
        (self.bits >> (5 + subject as u8)) & 1 != 0
    }

    pub(in crate::logic) fn set_sasha_has_lecture_notes(
        &mut self,
        subject: Subject,
        value: bool,
    ) {
        assert!(SUBJECTS_WITH_LECTURE_NOTES.contains(&subject));
        if value {
            self.bits |= 1 << (5 + subject as u8)
        } else {
            self.bits &= !(1 << (5 + subject as u8))
        }
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        struct LectureNotesInfoAdapter<'a>(&'a GameState);

        impl Debug for LectureNotesInfoAdapter<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_map()
                    .entries(SUBJECTS_WITH_LECTURE_NOTES.map(|subject| {
                        (subject, self.0.sasha_has_lecture_notes(subject))
                    }))
                    .finish()
            }
        }

        f.debug_struct("GameState")
            .field("player", &self.player)
            .field("current_day_index", &self.current_day_index)
            .field("current_time", &self.current_time)
            .field("timetable", &self.timetable)
            .field("location", &self.location)
            .field("classmates", &self.classmates)
            .field(
                "additional_computer_science_exams",
                &self.additional_computer_science_exams(),
            )
            .field("sasha_has_lecture_notes", &LectureNotesInfoAdapter(self))
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Location {
    PUNK = 1,
    PDMI = 2,
    ComputerClass = 3,
    Dorm = 4,
    Mausoleum = 5,
}

impl Location {
    pub fn is_exam_here_on_day(self, subject: Subject, today: &Day) -> bool {
        today
            .exam(subject)
            .map_or(false, |exam| exam.location() == self)
    }

    pub fn is_exam_here_now(self, subject: Subject, today: &Day, time: Time) -> bool {
        today.exam(subject).map_or(false, |exam| {
            exam.location() == self && time >= exam.from() && time < exam.to()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_bits() {
        let mut rng = random::Rng::new(0);
        let player = Player::new(
            false,
            HealthLevel(10),
            BrainLevel(11),
            StaminaLevel(12),
            CharismaLevel(13),
            |subject| BrainLevel(subject as i16),
        );
        let mut state = GameState::new(player, Timetable::random(&mut rng), Location::Dorm);
        assert_eq!(state.bits, 0b111_00000);
        assert!(state.sasha_has_lecture_notes(Subject::AlgebraAndNumberTheory));
        assert!(state.sasha_has_lecture_notes(Subject::Calculus));
        assert!(state.sasha_has_lecture_notes(Subject::GeometryAndTopology));

        state.set_sasha_has_lecture_notes(Subject::Calculus, false);

        assert_eq!(state.bits, 0b101_00000);
        assert!(state.sasha_has_lecture_notes(Subject::AlgebraAndNumberTheory));
        assert!(!state.sasha_has_lecture_notes(Subject::Calculus));
        assert!(state.sasha_has_lecture_notes(Subject::GeometryAndTopology));

        assert_eq!(state.additional_computer_science_exams(), 0);
        state.add_additional_computer_science_exam();
        assert_eq!(state.bits, 0b101_00001);
        assert_eq!(state.additional_computer_science_exams(), 1);

        state.add_additional_computer_science_exam();
        assert_eq!(state.bits, 0b101_00010);
        assert_eq!(state.additional_computer_science_exams(), 2);
    }
}
