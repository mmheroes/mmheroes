pub mod diamond;
pub mod grisha;
pub mod kolya;
pub mod kuzmenko;
pub mod pasha;
pub mod sasha;
pub mod serj;

use super::*;
use strum::VariantArray;

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum Classmate {
    Kolya = 0,
    Pasha,
    Diamond,
    RAI,
    Misha,
    Serj,
    Sasha,
    NiL,
    Kuzmenko,
    DJuG,
    Andrew,
    Grisha,
}

impl From<Classmate> for u16 {
    fn from(value: Classmate) -> u16 {
        value as u16
    }
}

impl Classmate {
    pub(in crate::logic) fn health_penalty(self) -> HealthLevel {
        match self {
            Kolya | Pasha | Diamond | Kuzmenko | DJuG | Andrew | Grisha | Misha
            | Serj | Sasha => HealthLevel(0),
            RAI | NiL => HealthLevel(8),
        }
    }

    /// «Вероятность» того, что человек будет приставать во время сдачи зачёта.
    pub(in crate::logic) fn annoyance(self) -> i16 {
        match self {
            Kolya | Pasha | Diamond | Serj | Sasha | Kuzmenko | DJuG | Andrew
            | Grisha => 0,
            RAI => 4,
            Misha => 2,
            NiL => 6,
        }
    }
}

use Classmate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassmateLocation {
    Nowhere,
    Exam(Subject),
    Location(Location),
}

#[derive(Debug, Clone)]
pub struct ClassmateInfo {
    classmate: Classmate,
    current_location: ClassmateLocation,
}

impl ClassmateInfo {
    pub fn classmate(&self) -> Classmate {
        self.classmate
    }

    pub fn current_location(&self) -> ClassmateLocation {
        self.current_location
    }

    pub(in crate::logic) fn update(
        &mut self,
        rng: &mut random::Rng,
        current_location: Location,
        today: &Day,
        time: Time,
    ) {
        let typically_in = |location: Location, classmate: &mut ClassmateInfo| {
            classmate.current_location = if time.is_between_9_and_19() {
                ClassmateLocation::Location(location)
            } else {
                ClassmateLocation::Nowhere
            }
        };

        match self.classmate {
            Kolya => {
                typically_in(Location::Mausoleum, self);
            }
            Pasha => {
                typically_in(Location::PUNK, self);
                maybe_on_exam(rng, current_location, today, self, || {
                    [
                        Subject::AlgebraAndNumberTheory,
                        Subject::Calculus,
                        Subject::GeometryAndTopology,
                    ]
                });
            }
            Diamond => {
                typically_in(Location::ComputerClass, self);
                for subject in Subject::all_subjects().rev() {
                    if current_location.is_exam_here_on_day(subject, today)
                        && rng.random(10) > 5
                    {
                        self.current_location = ClassmateLocation::Exam(subject);
                    }
                }
            }
            RAI => {
                typically_in(Location::ComputerClass, self);
                if current_location.is_exam_here_now(
                    Subject::AlgebraAndNumberTheory,
                    today,
                    time,
                ) {
                    self.current_location =
                        ClassmateLocation::Exam(Subject::AlgebraAndNumberTheory)
                } else if current_location.is_exam_here_now(
                    Subject::Calculus,
                    today,
                    time,
                ) {
                    self.current_location = ClassmateLocation::Exam(Subject::Calculus)
                };
            }
            Misha => {
                typically_in(Location::PUNK, self);
                maybe_on_exam(rng, current_location, today, self, || {
                    Subject::all_subjects()
                        .rev()
                        .filter(|&subject| subject != Subject::PhysicalEducation)
                })
            }
            Serj => {
                typically_in(Location::PUNK, self);
                maybe_on_exam(rng, current_location, today, self, || {
                    Subject::all_subjects().rev()
                });
            }
            Sasha => {
                self.current_location = if time.is_between_9_and_19() && rng.roll_dice(4)
                {
                    ClassmateLocation::Location(Location::PUNK)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            NiL => maybe_on_exam(rng, current_location, today, self, || {
                [
                    Subject::AlgebraAndNumberTheory,
                    Subject::Calculus,
                    Subject::GeometryAndTopology,
                ]
            }),
            Kuzmenko => {
                self.current_location = if time.is_between_9_and_19() && rng.roll_dice(4)
                {
                    ClassmateLocation::Location(Location::ComputerClass)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            DJuG => { /* TODO */ }
            Andrew => { /* TODO */ }
            Grisha => {
                self.current_location = if rng.roll_dice(3) {
                    ClassmateLocation::Location(Location::Mausoleum)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
        }
    }
}

fn maybe_on_exam<I: IntoIterator<Item = Subject>>(
    rng: &mut random::Rng,
    current_location: Location,
    today: &Day,
    classmate: &mut ClassmateInfo,
    mut possible_subjects: impl FnMut() -> I,
) {
    let mut at_least_one_exam_is_today = false;
    let mut is_present_at_some_exam = false;

    loop {
        for subject in possible_subjects().into_iter() {
            if current_location.is_exam_here_on_day(subject, today) {
                at_least_one_exam_is_today = true;
                if rng.random(10) > 5 {
                    is_present_at_some_exam = true;
                    classmate.current_location = ClassmateLocation::Exam(subject)
                }
            }
        }

        if is_present_at_some_exam || !at_least_one_exam_is_today {
            break;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Classmates([ClassmateInfo; Classmate::VARIANTS.len()]);

impl Classmates {
    pub(in crate::logic) fn new() -> Classmates {
        Classmates([
            ClassmateInfo {
                classmate: Kolya,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Pasha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Diamond,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: RAI,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Misha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Serj,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Sasha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: NiL,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Kuzmenko,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: DJuG,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Andrew,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Grisha,
                current_location: ClassmateLocation::Nowhere,
            },
        ])
    }

    pub fn filter_by_location(
        &self,
        location: Location,
    ) -> impl Iterator<Item = &ClassmateInfo> {
        self.iter().filter(move |&classmate| {
            matches!(classmate.current_location, ClassmateLocation::Location(l) if l == location)
        })
    }

    pub fn filter_by_exam(
        &self,
        subject: Subject,
    ) -> impl Iterator<Item = &ClassmateInfo> {
        self.iter().filter(move |&classmate| {
            matches!(classmate.current_location, ClassmateLocation::Exam(s) if s == subject)
        })
    }
}

impl core::ops::Index<Classmate> for Classmates {
    type Output = ClassmateInfo;

    fn index(&self, index: Classmate) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl core::ops::IndexMut<Classmate> for Classmates {
    fn index_mut(&mut self, index: Classmate) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl core::ops::Deref for Classmates {
    type Target = [ClassmateInfo];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Classmates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(super) async fn interact_with_classmate(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    classmate: Classmate,
    exam_in_progress: Option<Subject>,
) {
    match classmate {
        Kolya => kolya::interact(g, state).await,
        Pasha => pasha::interact(g, state).await,
        Diamond => diamond::interact(g, state, exam_in_progress).await,
        RAI => todo!("RAI"),
        Misha => todo!("Misha"),
        Serj => serj::interact(g, state, exam_in_progress).await,
        Sasha => sasha::interact(g, state).await,
        NiL => todo!("NiL"),
        Kuzmenko => kuzmenko::interact(g, state).await,
        DJuG => todo!("DJuG"),
        Andrew => todo!("Andrew"),
        Grisha => grisha::interact(g, state).await,
    };
}
