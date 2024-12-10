pub use crate::logic::{Location, Subject};

use crate::logic::{GameScreen, GameState, InternalGameState};
use bitfield_struct::bitfield;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Rem, Sub};
use strum::EnumCount;

pub const NUM_DAYS: usize = 6;

/// Количество часов, прошедших с полуночи.
///
/// Имеет семантику таймстэмпа, то есть, экземпляры этого типа нельзя складывать,
/// но к ним можно прибавлять экземпляры типа `Duration` и получать новый экземпляр
/// типа `Time`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Time(pub u8);

pub const WORKDAY_BEGINS: Time = Time(9);
pub const WORKDAY_ENDS: Time = Time(18);

impl Time {
    pub(super) const fn from_bits(bits: u8) -> Time {
        Time(bits)
    }

    pub(super) const fn into_bits(self) -> u8 {
        self.0
    }

    pub fn is_between_9_and_19(self) -> bool {
        self.0 >= 9 && self.0 <= 19
    }

    pub fn is_optimal_study_time(self) -> bool {
        self.0 < 19
    }

    pub fn is_suboptimal_study_time(self) -> bool {
        self.0 > 21 || self.0 < 4
    }

    /// Во сколько закрывается компьютерный класс.
    pub const fn computer_class_closing() -> Time {
        Time(20)
    }

    pub fn is_cafe_open(self) -> bool {
        self.0 >= 10 && self.0 <= 18
    }

    pub fn terkom_closing_time() -> Time {
        Time(19)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        assert!(self.0 < 24);
        f.write_fmt(format_args!("{}", self.0))
    }
}

/// A number of hours.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Duration(pub i8);

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        if rhs.0 < 0 {
            Time(self.0 - (-rhs.0 as u8))
        } else {
            Time(self.0 + rhs.0 as u8)
        }
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        self + Duration(-rhs.0)
    }
}

impl Rem<u8> for Time {
    type Output = Time;

    fn rem(self, rhs: u8) -> Self::Output {
        Time(self.0 % rhs)
    }
}

impl TryFrom<u64> for Time {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        u8::try_from(value).map(Time)
    }
}

impl TryFrom<Time> for u64 {
    type Error = core::convert::Infallible;

    fn try_from(value: Time) -> Result<Self, Self::Error> {
        Ok(u64::from(value.0))
    }
}

impl TryFrom<u64> for Duration {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        i8::try_from(value).map(Duration)
    }
}

impl TryFrom<Duration> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        u64::try_from(value.0)
    }
}

#[bitfield(u16, debug = true, default = false)]
#[derive(Eq, PartialEq)]
struct ExamBits {
    #[bits(3)]
    subject: Subject,

    #[bits(5)]
    from: Time,

    #[bits(5)]
    to: Time,

    #[bits(3, default = Location::PUNK)]
    location: Location,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Exam {
    bits: ExamBits,
}

impl Exam {
    pub const NO_EXAM: Exam = Exam {
        bits: ExamBits::new(),
    };

    pub(in crate::logic) const fn new(
        subject: Subject,
        from: Time,
        to: Time,
        location: Location,
    ) -> Exam {
        Exam {
            bits: ExamBits::new()
                .with_subject(subject)
                .with_from(from)
                .with_to(to)
                .with_location(location),
        }
    }

    pub fn is_some(self) -> bool {
        self != Exam::NO_EXAM
    }

    pub fn subject(self) -> Subject {
        self.bits.subject()
    }

    /// Время начала зачёта
    pub fn from(self) -> Time {
        self.bits.from()
    }

    /// Время окончания зачёта
    pub fn to(self) -> Time {
        self.bits.to()
    }

    pub(in crate::logic) fn one_hour_more(&mut self) {
        self.bits.set_to(self.bits.to() + Duration(1));
    }

    pub fn location(self) -> Location {
        self.bits.location()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Day {
    index: u8,
    exams: [Exam; Subject::COUNT],
}

impl Day {
    pub fn exam(&self, subject: Subject) -> Option<Exam> {
        let exam = self.exams[subject as usize];
        if exam.is_some() {
            Some(exam)
        } else {
            None
        }
    }

    pub(in crate::logic) fn exam_mut(&mut self, subject: Subject) -> Option<&mut Exam> {
        let exam = &mut self.exams[subject as usize];
        if exam.is_some() {
            Some(exam)
        } else {
            None
        }
    }

    pub(in crate::logic) fn add_exam(&mut self, exam: Exam) {
        self.exams[exam.subject() as usize] = exam
    }

    pub fn index(&self) -> usize {
        self.index as usize
    }

    pub fn exams(&self) -> impl Iterator<Item = &Exam> {
        self.exams.iter().filter(|&exam| exam.is_some())
    }

    pub fn current_exams(
        &self,
        location: Location,
        time: Time,
    ) -> impl Iterator<Item = &Exam> {
        self.exams().filter(move |exam| {
            exam.location() == location && time >= exam.from() && time < exam.to()
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timetable {
    days: [Day; NUM_DAYS],
}

impl Timetable {
    pub(in crate::logic) fn random(rng: &mut crate::random::Rng) -> Timetable {
        let mut days = [const {
            Day {
                index: 0,
                exams: [const { Exam::NO_EXAM }; Subject::COUNT],
            }
        }; NUM_DAYS];

        for (i, day) in days.iter_mut().enumerate() {
            day.index = i as u8;
        }

        for subject in Subject::all_subjects() {
            let mut day_used = [false; NUM_DAYS];
            for _ in 0..subject.exam_days() {
                let day_idx = loop {
                    let day = rng.random_in_range(0..NUM_DAYS);
                    if !day_used[day] {
                        day_used[day] = true;
                        break day;
                    }
                };

                let exam_begins_max = WORKDAY_ENDS - subject.exam_max_duration();

                let exam_start_time =
                    rng.random_in_range(WORKDAY_BEGINS..=exam_begins_max);
                let exam_duration = rng.random_in_range(
                    subject.exam_min_duration()..=subject.exam_max_duration(),
                );

                let exam = Exam::new(
                    subject,
                    exam_start_time,
                    exam_start_time + exam_duration,
                    *rng.random_element(subject.exam_places()),
                );
                days[day_idx].add_exam(exam);
            }
        }
        Timetable { days }
    }

    pub(in crate::logic) fn randomize_from_day(
        &mut self,
        day_index: u8,
        rng: &mut crate::random::Rng,
    ) {
        let old_timetable = self.clone();
        *self = Timetable::random(rng);
        for i in 0..=day_index {
            *self.day_mut(i) = old_timetable.day(i).clone();
        }
    }

    pub fn day(&self, index: u8) -> &Day {
        &self.days[index as usize]
    }

    pub(in crate::logic) fn day_mut(&mut self, index: u8) -> &mut Day {
        &mut self.days[index as usize]
    }

    pub fn days(&self) -> &[Day] {
        &self.days
    }
}

pub(in crate::logic) async fn show(g: &mut InternalGameState<'_>, state: &GameState) {
    g.set_screen_and_wait_for_any_key(GameScreen::Timetable(state.clone()))
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Rng;
    use Location::*;
    use Subject::*;

    #[test]
    fn generate_random_timetable() {
        let mut rng = Rng::new(42);

        {
            let timetable = Timetable::random(&mut rng);

            let expected = Timetable {
                days: [
                    Day {
                        index: 0,
                        exams: [
                            Exam::NO_EXAM,
                            Exam::new(Calculus, Time(13), Time(16), PUNK),
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::new(PhysicalEducation, Time(12), Time(13), PUNK),
                        ],
                    },
                    Day {
                        index: 1,
                        exams: [
                            Exam::new(AlgebraAndNumberTheory, Time(10), Time(12), PUNK),
                            Exam::new(Calculus, Time(10), Time(13), PUNK),
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::new(English, Time(11), Time(13), PUNK),
                            Exam::NO_EXAM,
                        ],
                    },
                    Day {
                        index: 2,
                        exams: [
                            Exam::new(AlgebraAndNumberTheory, Time(14), Time(17), PDMI),
                            Exam::new(Calculus, Time(14), Time(17), PUNK),
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::new(PhysicalEducation, Time(12), Time(13), PUNK),
                        ],
                    },
                    Day {
                        index: 3,
                        exams: [
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                        ],
                    },
                    Day {
                        index: 4,
                        exams: [
                            Exam::new(AlgebraAndNumberTheory, Time(9), Time(12), PDMI),
                            Exam::NO_EXAM,
                            Exam::new(GeometryAndTopology, Time(12), Time(14), PDMI),
                            Exam::new(ComputerScience, Time(16), Time(18), ComputerClass),
                            Exam::new(English, Time(12), Time(14), PUNK),
                            Exam::NO_EXAM,
                        ],
                    },
                    Day {
                        index: 5,
                        exams: [
                            Exam::new(AlgebraAndNumberTheory, Time(12), Time(14), PUNK),
                            Exam::new(Calculus, Time(12), Time(15), PUNK),
                            Exam::new(GeometryAndTopology, Time(14), Time(16), PDMI),
                            Exam::new(ComputerScience, Time(11), Time(12), ComputerClass),
                            Exam::NO_EXAM,
                            Exam::NO_EXAM,
                        ],
                    },
                ],
            };

            assert_eq!(timetable, expected);
        }
    }
}
