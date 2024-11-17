use super::*;
use strum::{EnumCount, FromRepr};

#[derive(Debug)]
#[allow(non_snake_case)] // TODO: Remove this
pub struct SubjectInfo {
    pub(in crate::logic) required_problems: u8,
    pub(in crate::logic) exam_days: u16,
    pub(in crate::logic) exam_min_duration: Duration,
    pub(in crate::logic) exam_max_duration: Duration,
    pub(in crate::logic) exam_places: [Location; 3],

    // TODO: Rename these fields
    #[allow(dead_code)]
    pub(in crate::logic) member0xFA: i16,

    #[allow(dead_code)]
    pub(in crate::logic) member0xFC: i16, // Минимальный уровень познания?

    #[allow(dead_code)]
    pub(in crate::logic) member0x100: i16,

    /// Какой уровень знаний соответствует какой оценке по шкале этого препода.
    pub(in crate::logic) assessment_bounds: [(BrainLevel, KnowledgeAssessment); 3],
}

impl SubjectInfo {
    pub fn required_problems(&self) -> u8 {
        self.required_problems
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr, EnumCount)]
pub enum Subject {
    AlgebraAndNumberTheory = 0,
    Calculus,
    GeometryAndTopology,
    ComputerScience,
    English,
    PhysicalEducation,
}

impl Subject {
    pub(super) const fn from_bits(bits: u8) -> Subject {
        match subject_from_bits(bits) {
            Some(subject) => subject,
            None => panic!("Invalid subject bits."),
        }
    }

    pub(super) const fn into_bits(self) -> u8 {
        self as u8
    }

    pub(super) fn all_subjects() -> impl DoubleEndedIterator<Item = Subject> {
        SUBJECTS.iter().map(|(subject, _)| *subject)
    }
}

pub(super) const fn subject_from_bits(bits: u8) -> Option<Subject> {
    Subject::from_repr(bits as usize)
}

pub(super) const fn subject_into_bits(subject: Option<Subject>) -> u8 {
    match subject {
        None => Subject::COUNT as u8,
        Some(s) => s as u8,
    }
}

pub struct Subjects([(Subject, SubjectInfo); Subject::COUNT]);

pub const SUBJECTS_WITH_LECTURE_NOTES: [Subject; 3] = [
    Subject::AlgebraAndNumberTheory,
    Subject::Calculus,
    Subject::GeometryAndTopology,
];

impl Subjects {
    const fn new() -> Subjects {
        use KnowledgeAssessment::*;
        use Location::*;
        use Subject::*;
        Subjects([
            (
                AlgebraAndNumberTheory,
                SubjectInfo {
                    required_problems: 12,
                    exam_days: 4,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(4),
                    exam_places: [PUNK, PUNK, PDMI],
                    member0xFA: 10,
                    member0xFC: 17,
                    member0x100: 3,
                    assessment_bounds: [
                        (BrainLevel(11), Bad),
                        (BrainLevel(21), Satisfactory),
                        (BrainLevel(51), Good),
                    ],
                },
            ),
            (
                Calculus,
                SubjectInfo {
                    required_problems: 10,
                    exam_days: 4,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(3),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 8,
                    member0xFC: 14,
                    member0x100: 2,
                    assessment_bounds: [
                        (BrainLevel(9), Bad),
                        (BrainLevel(19), Satisfactory),
                        (BrainLevel(41), Good),
                    ],
                },
            ),
            (
                GeometryAndTopology,
                SubjectInfo {
                    required_problems: 3,
                    exam_days: 2,
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(3),
                    exam_places: [PUNK, PDMI, PDMI],
                    member0xFA: 4,
                    member0xFC: 8,
                    member0x100: 3,
                    assessment_bounds: [
                        (BrainLevel(6), Bad),
                        (BrainLevel(11), Satisfactory),
                        (BrainLevel(31), Good),
                    ],
                },
            ),
            (
                ComputerScience,
                SubjectInfo {
                    required_problems: 2,
                    exam_days: 2, // FIXME: May be 3.
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(2),
                    exam_places: [ComputerClass, ComputerClass, ComputerClass],
                    member0xFA: 5,
                    member0xFC: 6,
                    member0x100: 3,
                    assessment_bounds: [
                        (BrainLevel(10), Bad),
                        (BrainLevel(16), Satisfactory),
                        (BrainLevel(31), Good),
                    ],
                },
            ),
            (
                English,
                SubjectInfo {
                    required_problems: 3,
                    exam_days: 2,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(2),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 7,
                    member0xFC: 10,
                    member0x100: 1,
                    assessment_bounds: [
                        (BrainLevel(5), Bad),
                        (BrainLevel(9), Satisfactory),
                        (BrainLevel(16), Good),
                    ],
                },
            ),
            (
                PhysicalEducation,
                SubjectInfo {
                    required_problems: 1,
                    exam_days: 2,
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(1),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 7,
                    member0xFC: 20,
                    member0x100: 1,
                    assessment_bounds: [
                        (BrainLevel(5), Bad),
                        (BrainLevel(9), Satisfactory),
                        (BrainLevel(16), Good),
                    ],
                },
            ),
        ])
    }

    pub fn iter(&self) -> core::slice::Iter<'_, (Subject, SubjectInfo)> {
        self.0.iter()
    }
}

impl core::ops::Index<Subject> for Subjects {
    type Output = SubjectInfo;

    fn index(&self, index: Subject) -> &Self::Output {
        &self.0[index as usize].1
    }
}

pub const SUBJECTS: Subjects = Subjects::new();
