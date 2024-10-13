pub mod grisha;
pub mod kolya;
pub mod kuzmenko;
pub mod pasha;
pub mod sasha;

use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Classmate {
    Kolya,
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

impl Classmate {
    #[allow(dead_code)]
    pub(in crate::logic) fn health_penalty(self) -> HealthLevel {
        match self {
            Kolya | Pasha | Diamond | Kuzmenko | DJuG | Andrew | Grisha | Misha
            | Serj | Sasha => HealthLevel(0),
            RAI | NiL => HealthLevel(8),
        }
    }

    #[allow(dead_code)]
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
        rng: &mut crate::random::Rng,
        current_location: Location,
        today: &Day,
        time: Time,
    ) {
        match self.classmate {
            Kolya => {
                self.current_location = if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::Mausoleum)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            Pasha => {
                self.current_location = if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::PUNK)
                } else {
                    ClassmateLocation::Nowhere
                };

                let subjects = [
                    Subject::AlgebraAndNumberTheory,
                    Subject::Calculus,
                    Subject::GeometryAndTopology,
                ];

                let mut at_least_one_exam_is_today = false;
                let mut pasha_is_present_at_some_exam = false;

                loop {
                    for subject in subjects.iter().cloned() {
                        if current_location.is_exam_here_on_day(subject, today) {
                            at_least_one_exam_is_today = true;
                            if rng.random(10) > 5 {
                                pasha_is_present_at_some_exam = true;
                                self.current_location = ClassmateLocation::Exam(subject)
                            }
                        }
                    }

                    if pasha_is_present_at_some_exam || !at_least_one_exam_is_today {
                        break;
                    }
                }
            }
            Diamond => { /* TODO */ }
            RAI => {
                self.current_location = if current_location.is_exam_here_now(
                    Subject::AlgebraAndNumberTheory,
                    today,
                    time,
                ) {
                    ClassmateLocation::Exam(Subject::AlgebraAndNumberTheory)
                } else if current_location.is_exam_here_now(
                    Subject::Calculus,
                    today,
                    time,
                ) {
                    ClassmateLocation::Exam(Subject::Calculus)
                } else if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::ComputerClass)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            Misha => { /* TODO */ }
            Serj => { /* TODO */ }
            Sasha => {
                self.current_location = if time.is_between_9_and_19() && rng.roll_dice(4)
                {
                    ClassmateLocation::Location(Location::PUNK)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            NiL => { /* TODO */ }
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

#[derive(Debug, Clone)]
pub struct Classmates([ClassmateInfo; 12]);

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
) {
    let available_actions = match classmate {
        Kolya => return kolya::interact(g, state).await,
        Pasha => {
            return pasha::interact(g, state).await;
        }
        Diamond => todo!("Diamond"),
        RAI => todo!("RAI"),
        Misha => todo!("Misha"),
        Serj => todo!("Serj"),
        Sasha => sasha::interact(g, state.clone()),
        NiL => todo!("NiL"),
        Kuzmenko => kuzmenko::interact(g, state.clone()),
        DJuG => todo!("DJuG"),
        Andrew => todo!("Andrew"),
        Grisha => grisha::interact(g, state.clone()),
    };
    g.set_available_actions_from_vec(available_actions);

    // LEGACY
    loop {
        let action = g.wait_for_action().await;
        if action == Action::IAmDone {
            todo!()
        }
        let new_actions = g.perform_action(action);
        g.set_available_actions_from_vec(new_actions);
    }
}
