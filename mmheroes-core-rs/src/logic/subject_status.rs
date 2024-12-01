use super::*;
use bitfield_struct::bitfield;
use core::fmt::{Debug, Formatter, Result as FmtResult};

#[bitfield(u16, debug = false, default = false)]
struct SubjectStatusBits {
    #[bits(3)]
    subject: Subject,

    #[bits(8)]
    problems_done: u8,

    #[bits(
        3,
        default = None,
        from = SubjectStatusBits::passed_exam_day_index_from_bits,
        into = SubjectStatusBits::passed_exam_day_index_to_bits
    )]
    passed_exam_day_index: Option<u8>,

    #[bits(1)]
    has_lecture_notes: bool,

    #[bits(1)]
    __: bool,
}

impl SubjectStatusBits {
    const fn passed_exam_day_index_from_bits(bits: u8) -> Option<u8> {
        if bits == NOT_PASSED as u8 {
            None
        } else {
            Some(bits)
        }
    }

    //noinspection RsReplaceMatchExpr
    const fn passed_exam_day_index_to_bits(passed_exam_day_index: Option<u8>) -> u8 {
        match passed_exam_day_index {
            Some(index) => index,
            None => NOT_PASSED as u8,
        }
    }
}

#[derive(Clone)]
pub struct SubjectStatus {
    pub(in crate::logic) knowledge: BrainLevel,
    bits: SubjectStatusBits,
}

const NOT_PASSED: u16 = (1 << SubjectStatusBits::PASSED_EXAM_DAY_INDEX_BITS) - 1;

impl SubjectStatus {
    pub(in crate::logic) fn new(subject: Subject, knowledge: BrainLevel) -> Self {
        Self {
            knowledge,
            bits: SubjectStatusBits::new().with_subject(subject),
        }
    }

    pub fn knowledge(&self) -> BrainLevel {
        self.knowledge
    }

    pub fn subject(&self) -> Subject {
        self.bits.subject()
    }

    pub fn problems_done(&self) -> u8 {
        self.bits.problems_done()
    }

    pub fn problems_remaining(&self) -> u8 {
        self.subject()
            .required_problems()
            .saturating_sub(self.problems_done())
    }

    pub fn solved_all_problems(&self) -> bool {
        self.problems_done() >= self.subject().required_problems()
    }

    pub(in crate::logic) fn more_problems_solved(&mut self, more: u8) {
        self.bits
            .set_problems_done(self.bits.problems_done() + more);
    }

    fn passed_exam_day_index(&self) -> Option<u8> {
        self.bits.passed_exam_day_index()
    }

    pub fn passed(&self) -> bool {
        self.passed_exam_day_index().is_some()
    }

    pub fn passed_exam_day<'a>(&self, timetable: &'a Timetable) -> Option<&'a Day> {
        self.passed_exam_day_index().map(|i| timetable.day(i))
    }

    pub(in crate::logic) fn set_passed_exam_day_index(&mut self, day_index: u8) {
        assert!(day_index < NOT_PASSED as u8, "Too big day index");
        assert!(
            self.bits.passed_exam_day_index().is_none(),
            "Cannot pass an exam more than once"
        );
        self.bits.set_passed_exam_day_index(Some(day_index));
    }

    pub fn has_lecture_notes(&self) -> bool {
        self.bits.has_lecture_notes()
    }

    pub(in crate::logic) fn set_has_lecture_notes(&mut self) {
        assert!(!self.has_lecture_notes(), "Already has lecture notes");
        self.bits.set_has_lecture_notes(true);
    }
}

impl Debug for SubjectStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SubjectStatus")
            .field("subject", &self.subject())
            .field("knowledge", &self.knowledge())
            .field("passed_exam_day_index", &self.passed_exam_day_index())
            .field("problems_done", &self.problems_done())
            .field("has_lecture_notes", &self.has_lecture_notes())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            SubjectStatus::new(Subject::AlgebraAndNumberTheory, BrainLevel(0))
                .bits
                .0,
            0b0_111_00000000_000
        );
        assert_eq!(
            SubjectStatus::new(Subject::Calculus, BrainLevel(0)).bits.0,
            0b0_111_00000000_001
        );
        assert_eq!(
            SubjectStatus::new(Subject::GeometryAndTopology, BrainLevel(0))
                .bits
                .0,
            0b0_111_00000000_010
        );
        assert_eq!(
            SubjectStatus::new(Subject::ComputerScience, BrainLevel(0))
                .bits
                .0,
            0b0_111_00000000_011
        );
        assert_eq!(
            SubjectStatus::new(Subject::English, BrainLevel(0)).bits.0,
            0b0_111_00000000_100
        );
        assert_eq!(
            SubjectStatus::new(Subject::PhysicalEducation, BrainLevel(0))
                .bits
                .0,
            0b0_111_00000000_101
        );
    }

    #[test]
    fn test_subject() {
        assert_eq!(
            SubjectStatus::new(Subject::AlgebraAndNumberTheory, BrainLevel(0)).subject(),
            Subject::AlgebraAndNumberTheory
        );
        assert_eq!(
            SubjectStatus::new(Subject::Calculus, BrainLevel(0)).subject(),
            Subject::Calculus
        );
        assert_eq!(
            SubjectStatus::new(Subject::GeometryAndTopology, BrainLevel(0)).subject(),
            Subject::GeometryAndTopology
        );
        assert_eq!(
            SubjectStatus::new(Subject::ComputerScience, BrainLevel(0)).subject(),
            Subject::ComputerScience
        );
        assert_eq!(
            SubjectStatus::new(Subject::English, BrainLevel(0)).subject(),
            Subject::English
        );
        assert_eq!(
            SubjectStatus::new(Subject::PhysicalEducation, BrainLevel(0)).subject(),
            Subject::PhysicalEducation
        );
    }

    #[test]
    fn test_problems_done() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        assert_eq!(status.problems_done(), 0);

        status.more_problems_solved(0);
        assert_eq!(status.problems_done(), 0);
        assert_eq!(status.bits.0, 0b0_111_00000000_001);

        status.more_problems_solved(1);
        assert_eq!(status.problems_done(), 1);
        assert_eq!(status.bits.0, 0b0_111_00000001_001);

        status.more_problems_solved(13);
        assert_eq!(status.problems_done(), 14);
        assert_eq!(status.bits.0, 0b0_111_00001110_001);

        status.more_problems_solved(240);
        assert_eq!(status.problems_done(), 254);
        assert_eq!(status.bits.0, 0b0_111_11111110_001);

        status.more_problems_solved(1);
        assert_eq!(status.problems_done(), 255);
        assert_eq!(status.bits.0, 0b0_111_11111111_001);
    }

    #[test]
    #[should_panic]
    fn test_too_many_problems_done() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        status.more_problems_solved(255);
        status.more_problems_solved(1);
    }

    #[test]
    fn test_passed_exam_day_index() {
        let mut status1 = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        assert_eq!(status1.passed_exam_day_index(), None);

        status1.set_passed_exam_day_index(0);
        assert_eq!(status1.passed_exam_day_index(), Some(0));
        assert_eq!(status1.bits.0, 0b0_000_00000000_001);

        let mut status2 = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        assert_eq!(status2.passed_exam_day_index(), None);

        status2.set_passed_exam_day_index(6);
        assert_eq!(status2.passed_exam_day_index(), Some(6));
        assert_eq!(status2.bits.0, 0b0_110_00000000_001);
    }

    #[test]
    #[should_panic]
    fn test_passed_exam_twice() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        status.set_passed_exam_day_index(1);
        status.set_passed_exam_day_index(2);
    }

    #[test]
    #[should_panic]
    fn test_too_bid_day_index() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        status.set_passed_exam_day_index(7);
    }

    #[test]
    fn test_has_lecture_notes() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        assert!(!status.has_lecture_notes());

        status.set_has_lecture_notes();
        assert!(status.has_lecture_notes());
        assert_eq!(status.bits.0, 0b1_111_00000000_001);
    }

    #[test]
    #[should_panic]
    fn test_set_has_lecture_notes_twice() {
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        status.set_has_lecture_notes();
        status.set_has_lecture_notes();
    }

    #[test]
    fn test_debug() {
        use std::fmt::Write;

        let mut result = String::new();
        let mut status = SubjectStatus::new(Subject::Calculus, BrainLevel(13));
        status.more_problems_solved(3);
        status.set_passed_exam_day_index(3);
        status.set_has_lecture_notes();
        writeln!(result, "{:?}", status).unwrap();

        assert_eq!(result, "SubjectStatus { subject: Calculus, knowledge: BrainLevel(13), passed_exam_day_index: Some(3), problems_done: 3, has_lecture_notes: true }\n");
    }
}
