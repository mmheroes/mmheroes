use super::*;
use bitfield_struct::bitfield;
use strum::EnumCount;

#[bitfield(u16, debug = true, default = false)]
struct PlayerBits {
    /// Получил ли персонаж дискету с новой версией MMHEROES от Diamond
    #[bits(1)]
    has_mmheroes_floppy: bool,

    #[bits(1)]
    has_internet: bool,

    #[bits(1)]
    is_invited: bool,

    #[bits(1)]
    inception: bool,

    #[bits(1)]
    is_employed_at_terkom: bool,

    #[bits(1)]
    got_stipend: bool,

    #[bits(1)]
    has_roundtrip_train_ticket: bool,

    #[bits(1)]
    knows_djug: bool,

    /// Последний зачёт, который пытался сдать игрок. Он может присниться.
    #[bits(3, default = None, from = subject_from_bits, into = subject_into_bits)]
    last_exam: Option<Subject>,

    #[bits(4)]
    _padding: u16,

    #[bits(1)]
    god_mode: bool,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub(in crate::logic) subjects: [SubjectStatus; Subject::COUNT],

    bits: PlayerBits,

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
        let player = Player {
            subjects: [
                SubjectStatus::new(
                    Subject::AlgebraAndNumberTheory,
                    knowledge(Subject::AlgebraAndNumberTheory),
                ),
                SubjectStatus::new(Subject::Calculus, knowledge(Subject::Calculus)),
                SubjectStatus::new(
                    Subject::GeometryAndTopology,
                    knowledge(Subject::GeometryAndTopology),
                ),
                SubjectStatus::new(
                    Subject::ComputerScience,
                    knowledge(Subject::ComputerScience),
                ),
                SubjectStatus::new(Subject::English, knowledge(Subject::English)),
                SubjectStatus::new(
                    Subject::PhysicalEducation,
                    knowledge(Subject::PhysicalEducation),
                ),
            ],
            bits: PlayerBits::new().with_god_mode(god_mode),
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
        self.subjects.iter().filter(|s| !s.passed()).count()
    }

    pub fn is_god_mode(&self) -> bool {
        self.bits.god_mode()
    }

    pub fn has_mmheroes_floppy(&self) -> bool {
        self.bits.has_mmheroes_floppy()
    }

    pub(in crate::logic) fn set_has_mmheroes_floppy(&mut self) {
        self.bits.set_has_mmheroes_floppy(true);
    }

    pub fn has_internet(&self) -> bool {
        self.bits.has_internet()
    }

    pub(in crate::logic) fn set_has_internet(&mut self) {
        self.bits.set_has_internet(true);
    }

    pub fn is_employed_at_terkom(&self) -> bool {
        self.bits.is_employed_at_terkom()
    }

    pub(in crate::logic) fn set_employed_at_terkom(&mut self) {
        self.bits.set_is_employed_at_terkom(true);
    }

    pub fn health(&self) -> HealthLevel {
        self.health
    }

    pub fn money(&self) -> Money {
        self.money
    }

    pub fn got_stipend(&self) -> bool {
        self.bits.got_stipend()
    }

    pub(in crate::logic) fn set_got_stipend(&mut self) {
        self.bits.set_got_stipend(true);
    }

    pub fn has_roundtrip_train_ticket(&self) -> bool {
        self.bits.has_roundtrip_train_ticket()
    }

    pub(in crate::logic) fn set_has_roundtrip_train_ticket(&mut self) {
        self.bits.set_has_roundtrip_train_ticket(true);
    }

    #[allow(dead_code)]
    pub(in crate::logic) fn last_exam(&self) -> Option<Subject> {
        self.bits.last_exam()
    }

    pub(in crate::logic) fn set_last_exam(&mut self, subject: Subject) {
        self.bits.set_last_exam(Some(subject))
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
