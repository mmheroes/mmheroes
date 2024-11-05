use super::*;
use bitfield_struct::bitfield;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use strum::EnumCount;
use strum::FromRepr;

#[bitfield(u32, debug = false, default = false)]
struct GameStateBits {
    #[bits(3)]
    current_day_index: u8,

    #[bits(5, default = Time(8))]
    current_time: Time,

    #[bits(2)]
    additional_computer_science_exams: u8,

    #[bits(1, default = true)]
    sasha_has_algebra_lecture_notes: bool,

    #[bits(1, default = true)]
    sasha_has_calculus_lecture_notes: bool,

    #[bits(1, default = true)]
    sasha_has_geometry_lecture_notes: bool,

    #[bits(3, default = Location::Dorm)]
    location: Location,

    #[bits(1, default = true)]
    terkom_has_places: bool,

    #[bits(3, default = None, from = exam_in_progress_from_bits, into = exam_in_progress_into_bits)]
    exam_in_progress: Option<Subject>,

    #[bits(12)]
    _padding: u32,
}

const fn exam_in_progress_from_bits(bits: u8) -> Option<Subject> {
    if bits >= Subject::COUNT as u8 {
        None
    } else {
        Some(Subject::from_bits(bits))
    }
}

const fn exam_in_progress_into_bits(subject: Option<Subject>) -> u8 {
    match subject {
        None => Subject::COUNT as u8,
        Some(s) => s.into_bits(),
    }
}

#[derive(Clone)]
pub struct GameState {
    pub(in crate::logic) player: Player,
    pub(in crate::logic) timetable: Timetable,
    pub(in crate::logic) classmates: Classmates,

    bits: GameStateBits,
}

impl GameState {
    pub(in crate::logic) fn new(
        player: Player,
        timetable: Timetable,
        location: Location,
    ) -> GameState {
        GameState {
            player,
            timetable,
            classmates: Classmates::new(),
            bits: GameStateBits::new().with_location(location),
        }
    }

    pub(in crate::logic) fn current_day_index(&self) -> u8 {
        self.bits.current_day_index()
    }

    pub fn current_day(&self) -> &Day {
        self.timetable.day(self.current_day_index())
    }

    pub(in crate::logic) fn next_day(&mut self) {
        self.bits
            .set_current_day_index(self.bits.current_day_index() + 1);
    }

    pub fn current_time(&self) -> Time {
        self.bits.current_time()
    }

    pub(in crate::logic) fn midnight(&mut self) {
        self.bits.set_current_time(Time(0));
    }

    pub(in crate::logic) fn next_hour(&mut self) {
        self.bits
            .set_current_time(self.bits.current_time() + Duration(1));
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn timetable(&self) -> &timetable::Timetable {
        &self.timetable
    }

    pub fn location(&self) -> Location {
        self.bits.location()
    }

    pub fn exam_in_progress(&self) -> Option<Subject> {
        self.bits.exam_in_progress()
    }

    pub(in crate::logic) fn set_exam_in_progress(&mut self, subject: Option<Subject>) {
        self.bits.set_exam_in_progress(subject);
    }

    pub(in crate::logic) fn set_location(&mut self, location: Location) {
        self.bits.set_location(location);
    }

    pub fn classmates(&self) -> &Classmates {
        &self.classmates
    }

    pub(in crate::logic) fn additional_computer_science_exams(&self) -> u8 {
        self.bits.additional_computer_science_exams()
    }

    pub(in crate::logic) fn add_additional_computer_science_exam(&mut self) {
        self.bits.set_additional_computer_science_exams(
            self.bits.additional_computer_science_exams() + 1,
        );
    }

    pub(in crate::logic) fn sasha_has_lecture_notes(&self, subject: Subject) -> bool {
        match subject {
            Subject::AlgebraAndNumberTheory => {
                self.bits.sasha_has_algebra_lecture_notes()
            }
            Subject::Calculus => self.bits.sasha_has_calculus_lecture_notes(),
            Subject::GeometryAndTopology => self.bits.sasha_has_geometry_lecture_notes(),
            _ => panic!("No lecture notes for this subject"),
        }
    }

    pub(in crate::logic) fn set_sasha_has_lecture_notes(
        &mut self,
        subject: Subject,
        value: bool,
    ) {
        match subject {
            Subject::AlgebraAndNumberTheory => {
                self.bits.set_sasha_has_algebra_lecture_notes(value)
            }
            Subject::Calculus => self.bits.set_sasha_has_calculus_lecture_notes(value),
            Subject::GeometryAndTopology => {
                self.bits.set_sasha_has_geometry_lecture_notes(value)
            }
            _ => panic!("No lecture notes for this subject"),
        }
    }

    pub(in crate::logic) fn terkom_has_places(&self) -> bool {
        self.bits.terkom_has_places()
    }

    pub(in crate::logic) fn set_terkom_has_places(&mut self, value: bool) {
        self.bits.set_terkom_has_places(value)
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
            .field("current_day_index", &self.bits.current_day_index())
            .field("current_time", &self.current_time())
            .field("timetable", &self.timetable)
            .field("location", &self.location())
            .field("classmates", &self.classmates)
            .field(
                "additional_computer_science_exams",
                &self.additional_computer_science_exams(),
            )
            .field("sasha_has_lecture_notes", &LectureNotesInfoAdapter(self))
            .field("terkom_has_places", &self.terkom_has_places())
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, FromRepr)]
pub enum Location {
    PUNK = 1,
    PDMI = 2,
    ComputerClass = 3,
    Dorm = 4,
    Mausoleum = 5,
}

impl Location {
    const fn from_bits(bits: u8) -> Location {
        match Location::from_repr(bits as usize) {
            Some(location) => location,
            None => panic!("Invalid location"),
        }
    }

    const fn into_bits(self) -> u8 {
        self as _
    }

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
        let mut state =
            GameState::new(player, Timetable::random(&mut rng), Location::Dorm);
        assert_eq!(state.bits.0, 0b110_1_100_111_00_01000_000);
        assert!(state.sasha_has_lecture_notes(Subject::AlgebraAndNumberTheory));
        assert!(state.sasha_has_lecture_notes(Subject::Calculus));
        assert!(state.sasha_has_lecture_notes(Subject::GeometryAndTopology));

        state.set_sasha_has_lecture_notes(Subject::Calculus, false);

        assert_eq!(state.bits.0, 0b110_1_100_101_00_01000_000);
        assert!(state.sasha_has_lecture_notes(Subject::AlgebraAndNumberTheory));
        assert!(!state.sasha_has_lecture_notes(Subject::Calculus));
        assert!(state.sasha_has_lecture_notes(Subject::GeometryAndTopology));

        assert_eq!(state.additional_computer_science_exams(), 0);
        state.add_additional_computer_science_exam();
        assert_eq!(state.bits.0, 0b110_1_100_101_01_01000_000);
        assert_eq!(state.additional_computer_science_exams(), 1);

        state.add_additional_computer_science_exam();
        assert_eq!(state.bits.0, 0b110_1_100_101_10_01000_000);
        assert_eq!(state.additional_computer_science_exams(), 2);

        state.next_day();
        assert_eq!(state.bits.0, 0b110_1_100_101_10_01000_001);
        assert_eq!(state.current_day_index(), 1);

        state.next_hour();
        assert_eq!(state.bits.0, 0b110_1_100_101_10_01001_001);
        assert_eq!(state.current_time(), Time(9));

        state.midnight();
        assert_eq!(state.bits.0, 0b110_1_100_101_10_00000_001);
        assert_eq!(state.current_time(), Time(0));

        state.set_location(Location::PUNK);
        assert_eq!(state.bits.0, 0b110_1_001_101_10_00000_001);
        assert_eq!(state.location(), Location::PUNK);

        state.set_terkom_has_places(false);
        assert_eq!(state.bits.0, 0b110_0_001_101_10_00000_001);
        assert!(!state.terkom_has_places());

        state.set_exam_in_progress(Some(Subject::English));
        assert_eq!(state.bits.0, 0b100_0_001_101_10_00000_001);
        assert_eq!(state.exam_in_progress(), Some(Subject::English));

        state.set_exam_in_progress(Some(Subject::Calculus));
        assert_eq!(state.bits.0, 0b001_0_001_101_10_00000_001);
        assert_eq!(state.exam_in_progress(), Some(Subject::Calculus));
    }
}
