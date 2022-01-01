use super::*;

use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    struct PlayerFlags: u16 {

        /// Получил ли персонаж дискету с новой версией MMHEROES от Diamond
        const HAS_MMHEROES_FLOPPY = 1 << 0;

        const HAS_INTERNET = 1 << 1;

        const IS_INVITED = 1 << 2;

        const INCEPTION = 1 << 3;

        const IS_EMPLOYED_AT_TERKOM = 1 << 4;

        const GOT_STIPEND = 1 << 5;

        const HAS_TICKET = 1 << 6;

        const KNOWS_DJUG = 1 << 7;

        const GOD_MODE = 1 << 15;
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub(in crate::logic) subjects: [SubjectStatus; NUM_SUBJECTS],

    flags: PlayerFlags,

    /// Запах чеснока изо рта
    pub(in crate::logic) garlic: i16,

    pub(in crate::logic) health: HealthLevel,
    pub(in crate::logic) money: Money,
    pub(in crate::logic) brain: BrainLevel,
    pub(in crate::logic) stamina: StaminaLevel,
    pub(in crate::logic) charisma: CharismaLevel,

    pub(in crate::logic) cause_of_death: Option<CauseOfDeath>,
}

impl Player {
    pub(in crate::logic) fn new(
        god_mode: bool,
        health: HealthLevel,
        brain: BrainLevel,
        stamina: StaminaLevel,
        charisma: CharismaLevel,
        mut knowledge: impl FnMut(Subject) -> BrainLevel,
    ) -> Player {
        let mut flags = PlayerFlags::empty();
        flags.set(PlayerFlags::GOD_MODE, god_mode);
        let player = Player {
            subjects: [
                SubjectStatus::new(Subject::AlgebraAndNumberTheory, knowledge(Subject::AlgebraAndNumberTheory)),
                SubjectStatus::new(Subject::Calculus, knowledge(Subject::Calculus)),
                SubjectStatus::new(Subject::GeometryAndTopology, knowledge(Subject::GeometryAndTopology)),
                SubjectStatus::new(Subject::ComputerScience, knowledge(Subject::ComputerScience)),
                SubjectStatus::new(Subject::English, knowledge(Subject::English)),
                SubjectStatus::new(Subject::PhysicalEducation, knowledge(Subject::PhysicalEducation)),
            ],
            flags,
            garlic: 0,
            health,
            money: Money(0),
            brain,
            stamina,
            charisma,
            cause_of_death: None,
        };

        for subject in player.subjects.iter() {
            assert!(subject.knowledge < player.brain);
        }

        player
    }

    pub fn status_for_subject(&self, subject: Subject) -> &SubjectStatus {
        &self.subjects[subject as usize]
    }

    pub(in crate::logic) fn status_for_subject_mut(
        &mut self,
        subject: Subject,
    ) -> &mut SubjectStatus {
        &mut self.subjects[subject as usize]
    }

    pub fn exams_left(&self) -> usize {
        self.subjects
          .iter()
          .filter(|s| !s.passed())
          .count()
    }

    pub fn is_god_mode(&self) -> bool {
        self.flags.contains(PlayerFlags::GOD_MODE)
    }

    pub fn has_mmheroes_floppy(&self) -> bool {
        self.flags.contains(PlayerFlags::HAS_MMHEROES_FLOPPY)
    }

    pub fn has_internet(&self) -> bool {
        self.flags.contains(PlayerFlags::HAS_INTERNET)
    }

    pub(in crate::logic) fn set_has_internet(&mut self) {
        self.flags.insert(PlayerFlags::HAS_INTERNET)
    }

    pub fn is_employed_at_terkom(&self) -> bool {
        self.flags.contains(PlayerFlags::IS_EMPLOYED_AT_TERKOM)
    }

    pub(in crate::logic) fn set_employed_at_terkom(&mut self) {
        self.flags.insert(PlayerFlags::IS_EMPLOYED_AT_TERKOM)
    }

    pub fn health(&self) -> HealthLevel {
        self.health
    }

    pub fn money(&self) -> Money {
        self.money
    }

    pub fn got_stipend(&self) -> bool {
        self.flags.contains(PlayerFlags::GOT_STIPEND)
    }

    pub(in crate::logic) fn set_got_stipend(&mut self) {
        self.flags.insert(PlayerFlags::GOT_STIPEND);
    }

    pub fn brain(&self) -> BrainLevel {
        self.brain
    }

    pub fn stamina(&self) -> StaminaLevel {
        self.stamina
    }

    pub fn charisma(&self) -> CharismaLevel {
        self.charisma
    }

    pub fn cause_of_death(&self) -> Option<CauseOfDeath> {
        self.cause_of_death
    }
}
