use super::*;
use core::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct SubjectStatus {
    pub(in crate::logic) knowledge: BrainLevel,

    /// Компактное представления для `Subject`, количества сданных задач, дня,
    /// когда зачёт был сдан (если был сдан).
    ///
    /// Младшие 3 бита — предмет, следующие 8 бит отвечают за количество сданных задач,
    /// следующие 3 бита обозначают индекс дня, в который экзамен был сдан.
    /// Если экзамен не был сдан, все три бита выставлены в единицу.
    /// 14-й бит — флаг наличия конспекта по этом предмету.
    ///
    /// ```plain
    ///             15                             0
    ///              * * * * * * * * * * * * * * * *
    ///               │ │     │               │     │
    ///               └┬┴──┬──┴───────┬───────┴──┬──┘
    /// has_lecture_notes  │          │          │
    ///                    │          │          │
    ///    passed_exam_day_index      │       subject
    ///                               │
    ///                         problems_done
    /// ```
    bits: u16,
}

const SUBJECT_NBITS: u16 = 3;
const SUBJECT_BITMASK: u16 = (1 << SUBJECT_NBITS) - 1;

const PROBLEMS_DONE_NBITS: u16 = 8;
const MAX_PROBLEMS_DONE: u16 = (1 << PROBLEMS_DONE_NBITS) - 1;
const PROBLEMS_DONE_BITMASK: u16 = MAX_PROBLEMS_DONE << SUBJECT_NBITS;

const PASSED_EXAM_DAY_INDEX_NBITS: u16 = 3;
const NOT_PASSED: u16 = (1 << PASSED_EXAM_DAY_INDEX_NBITS) - 1;
const PASSED_EXAM_DAY_INDEX_BITMASK: u16 =
    NOT_PASSED << (SUBJECT_NBITS + PROBLEMS_DONE_NBITS);

const HAS_LECTURE_NOTES_BIT: u16 =
    SUBJECT_NBITS + PROBLEMS_DONE_NBITS + PASSED_EXAM_DAY_INDEX_NBITS;

fn bits(
    subject: Subject,
    has_lecture_notes: bool,
    passed_exam_day_index: Option<usize>,
    problems_done: u8,
) -> u16 {
    let mut result = 0;

    let day_index = if let Some(day_index) = passed_exam_day_index {
        assert!(
            day_index < NOT_PASSED as usize,
            "day index must be less than {}",
            NOT_PASSED
        );
        day_index as u16
    } else {
        NOT_PASSED
    };
    result |= day_index;

    result <<= PROBLEMS_DONE_NBITS;
    let problems_done = problems_done as u16;
    assert!(
        problems_done <= MAX_PROBLEMS_DONE,
        "number of solved problems must be less than {}",
        MAX_PROBLEMS_DONE
    );
    result |= problems_done;

    result <<= SUBJECT_NBITS;
    let subject_as_number = subject as u16;
    assert!(
        subject_as_number < (1 << SUBJECT_NBITS),
        "subject must fit in {} bits",
        SUBJECT_NBITS
    );
    result |= subject_as_number;

    if has_lecture_notes {
        result |= 1 << HAS_LECTURE_NOTES_BIT
    }

    result
}

impl SubjectStatus {
    pub(in crate::logic) fn new(subject: Subject, knowledge: BrainLevel) -> Self {
        Self {
            knowledge,
            bits: bits(subject, false, None, 0),
        }
    }

    pub fn knowledge(&self) -> BrainLevel {
        self.knowledge
    }

    pub fn subject(&self) -> Subject {
        let subject_bits = (self.bits & SUBJECT_BITMASK) as u8;
        Subject::try_from(subject_bits).unwrap()
    }

    pub fn problems_done(&self) -> u8 {
        ((self.bits & PROBLEMS_DONE_BITMASK) >> SUBJECT_NBITS) as u8
    }

    pub(in crate::logic) fn more_problems_solved(&mut self, more: u8) {
        let problems_done = (self.problems_done() + more) as u16;
        assert!(problems_done <= MAX_PROBLEMS_DONE);

        // set all problems_done bits to 0
        self.bits &= !(MAX_PROBLEMS_DONE << SUBJECT_NBITS);

        // then set those bits to the desired number
        self.bits ^= problems_done << SUBJECT_NBITS;
    }

    fn passed_exam_day_index(&self) -> Option<u8> {
        let day_index_bits = (self.bits & PASSED_EXAM_DAY_INDEX_BITMASK)
            >> (SUBJECT_NBITS + PROBLEMS_DONE_NBITS);
        if day_index_bits == NOT_PASSED {
            None
        } else {
            Some(day_index_bits as u8)
        }
    }

    pub fn passed(&self) -> bool {
        self.passed_exam_day_index().is_some()
    }

    pub fn passed_exam_day<'a>(&self, timetable: &'a Timetable) -> Option<&'a Day> {
        self.passed_exam_day_index().map(|i| timetable.day(i))
    }

    #[allow(dead_code)]
    pub(in crate::logic) fn set_passed_exam_day_index(&mut self, day_index: usize) {
        assert!(day_index < NOT_PASSED as usize, "Too big day index");
        let prev_day_index = (self.bits & PASSED_EXAM_DAY_INDEX_BITMASK)
            >> (SUBJECT_NBITS + PROBLEMS_DONE_NBITS);
        assert_eq!(
            prev_day_index, NOT_PASSED,
            "Cannot pass an exam more than once"
        );

        let day_index = day_index as u16;

        // set all passed_exam_day_index bits to 0
        self.bits &= !(NOT_PASSED << (SUBJECT_NBITS + PROBLEMS_DONE_NBITS));

        // then set those bits to the desired number
        self.bits ^= day_index << (SUBJECT_NBITS + PROBLEMS_DONE_NBITS);
    }

    pub fn has_lecture_notes(&self) -> bool {
        (self.bits >> HAS_LECTURE_NOTES_BIT) & 1 != 0
    }

    pub(in crate::logic) fn set_has_lecture_notes(&mut self) {
        assert!(!self.has_lecture_notes(), "Already has lecture notes");
        self.bits |= 1 << HAS_LECTURE_NOTES_BIT
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
            SubjectStatus::new(Subject::AlgebraAndNumberTheory, BrainLevel(0)).bits,
            0b0_111_00000000_000
        );
        assert_eq!(
            SubjectStatus::new(Subject::Calculus, BrainLevel(0)).bits,
            0b0_111_00000000_001
        );
        assert_eq!(
            SubjectStatus::new(Subject::GeometryAndTopology, BrainLevel(0)).bits,
            0b0_111_00000000_010
        );
        assert_eq!(
            SubjectStatus::new(Subject::ComputerScience, BrainLevel(0)).bits,
            0b0_111_00000000_011
        );
        assert_eq!(
            SubjectStatus::new(Subject::English, BrainLevel(0)).bits,
            0b0_111_00000000_100
        );
        assert_eq!(
            SubjectStatus::new(Subject::PhysicalEducation, BrainLevel(0)).bits,
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
        assert_eq!(status.bits, 0b0_111_00000000_001);

        status.more_problems_solved(1);
        assert_eq!(status.problems_done(), 1);
        assert_eq!(status.bits, 0b0_111_00000001_001);

        status.more_problems_solved(13);
        assert_eq!(status.problems_done(), 14);
        assert_eq!(status.bits, 0b0_111_00001110_001);

        status.more_problems_solved(240);
        assert_eq!(status.problems_done(), 254);
        assert_eq!(status.bits, 0b0_111_11111110_001);

        status.more_problems_solved(1);
        assert_eq!(status.problems_done(), 255);
        assert_eq!(status.bits, 0b0_111_11111111_001);
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
        assert_eq!(status1.bits, 0b0_000_00000000_001);

        let mut status2 = SubjectStatus::new(Subject::Calculus, BrainLevel(0));
        assert_eq!(status2.passed_exam_day_index(), None);

        status2.set_passed_exam_day_index(6);
        assert_eq!(status2.passed_exam_day_index(), Some(6));
        assert_eq!(status2.bits, 0b0_110_00000000_001);
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
        assert_eq!(status.bits, 0b1_111_00000000_001);
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
