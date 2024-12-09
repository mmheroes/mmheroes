use super::*;
use strum::{EnumCount, FromRepr, VariantArray};

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr, EnumCount, VariantArray)]
pub enum Subject {
    AlgebraAndNumberTheory = 0,
    Calculus,
    GeometryAndTopology,
    ComputerScience,
    English,
    PhysicalEducation,
}

use Subject::*;

impl Subject {
    pub fn all_subjects() -> impl DoubleEndedIterator<Item = Subject> {
        Self::VARIANTS.iter().cloned()
    }

    pub(super) fn math_subjects() -> impl DoubleEndedIterator<Item = Subject> {
        Self::all_subjects().filter(|subject| subject.is_math())
    }

    pub(super) const fn from_bits(bits: u8) -> Subject {
        match Subject::from_repr(bits as usize) {
            Some(subject) => subject,
            None => panic!("Invalid subject bits."),
        }
    }

    pub(super) const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Является ли предмет математической дисциплиной.
    /// Некоторые действия в игре применимы только для таких предметов.
    pub(super) const fn is_math(self) -> bool {
        match self {
            AlgebraAndNumberTheory | Calculus | GeometryAndTopology => true,
            ComputerScience | English | PhysicalEducation => false,
        }
    }

    /// Количество задач, которые необходимо решить для получения зачёта по предмету.
    pub const fn required_problems(self) -> u8 {
        match self {
            AlgebraAndNumberTheory => 12,
            Calculus => 10,
            GeometryAndTopology | English => 3,
            ComputerScience => 2,
            PhysicalEducation => 1,
        }
    }

    /// Количество дней в изначальном расписании, в которые может проходить зачёт по
    /// предмету.
    /// В течение игры, впрочем, расписание может меняться — могут добавляться новые дни.
    pub(super) const fn exam_days(self) -> u16 {
        match self {
            AlgebraAndNumberTheory | Calculus => 4,
            GeometryAndTopology | ComputerScience | English | PhysicalEducation => 2,
        }
    }

    /// Минимальная продолжительность зачёта по предмету в часах.
    pub(super) const fn exam_min_duration(self) -> Duration {
        match self {
            AlgebraAndNumberTheory | Calculus | English => Duration(2),
            GeometryAndTopology | ComputerScience | PhysicalEducation => Duration(1),
        }
    }

    /// Максимальная продолжительность зачёта по предмету в часах.
    /// Используется только при составлении изначального расписания.
    ///
    /// Важно: расписание может меняться, зачёт может затягиваться сверх этого числа.
    pub(super) const fn exam_max_duration(self) -> Duration {
        match self {
            AlgebraAndNumberTheory => Duration(4),
            Calculus | GeometryAndTopology => Duration(3),
            ComputerScience | English => Duration(2),
            PhysicalEducation => Duration(1),
        }
    }

    /// Чем больше это число, тем больше знаний, интеллекта и здоровья нужно для того,
    /// чтобы преподаватель зачёл задачу.
    pub(super) const fn mental_load(self) -> BrainLevel {
        match self {
            AlgebraAndNumberTheory => BrainLevel(10),
            Calculus => BrainLevel(8),
            GeometryAndTopology => BrainLevel(4),
            ComputerScience => BrainLevel(5),
            English | PhysicalEducation => BrainLevel(7),
        }
    }

    /// Чем больше это число, тем больше выносливости нужно чтобы не терять здоровье
    /// при попытке сдать зачёт.
    pub(super) const fn health_penalty(self) -> HealthLevel {
        match self {
            AlgebraAndNumberTheory => 17,
            Calculus => 14,
            GeometryAndTopology => 8,
            ComputerScience => 6,
            English => 10,
            PhysicalEducation => 20,
        }
    }

    /// Чем больше это число, тем больше mental capacity нужно для того,
    /// чтобы зачли _одну_ задачу.
    pub(super) const fn single_problem_mental_factor(self) -> f32 {
        match self {
            AlgebraAndNumberTheory | GeometryAndTopology | ComputerScience => 3.0,
            Calculus => 2.0,
            English | PhysicalEducation => 1.0,
        }
    }

    /// В каких местах может проходить зачёт по предмету.
    ///
    /// При составлении расписания из этого массива выбирается случайный элемент.
    pub(super) const fn exam_places(self) -> &'static [Location] {
        use Location::*;
        match self {
            AlgebraAndNumberTheory => &[PUNK, PUNK, PDMI],
            GeometryAndTopology => &[PUNK, PDMI, PDMI],
            ComputerScience => &[ComputerClass, ComputerClass, ComputerClass],
            Calculus | English | PhysicalEducation => &[PUNK, PUNK, PUNK],
        }
    }

    /// Какой уровень знаний соответствует какой оценке по шкале этого препода.
    pub(super) const fn assessment_bounds(
        self,
    ) -> &'static [(BrainLevel, KnowledgeAssessment)] {
        use KnowledgeAssessment::*;
        match self {
            AlgebraAndNumberTheory => &[
                (BrainLevel(11), Bad),
                (BrainLevel(21), Satisfactory),
                (BrainLevel(51), Good),
            ],
            Calculus => &[
                (BrainLevel(9), Bad),
                (BrainLevel(19), Satisfactory),
                (BrainLevel(41), Good),
            ],
            GeometryAndTopology => &[
                (BrainLevel(6), Bad),
                (BrainLevel(11), Satisfactory),
                (BrainLevel(31), Good),
            ],
            ComputerScience => &[
                (BrainLevel(10), Bad),
                (BrainLevel(16), Satisfactory),
                (BrainLevel(31), Good),
            ],
            English => &[
                (BrainLevel(5), Bad),
                (BrainLevel(9), Satisfactory),
                (BrainLevel(16), Good),
            ],
            PhysicalEducation => &[
                (BrainLevel(5), Bad),
                (BrainLevel(9), Satisfactory),
                (BrainLevel(16), Good),
            ],
        }
    }
}
