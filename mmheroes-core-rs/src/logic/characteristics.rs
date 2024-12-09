use super::*;

macro_rules! define_characteristic {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
        pub struct $name(pub i16);

        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::Add<i16> for $name {
            type Output = Self;
            fn add(self, rhs: i16) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl core::ops::AddAssign<i16> for $name {
            fn add_assign(&mut self, rhs: i16) {
                self.0 += rhs
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl core::ops::Sub<i16> for $name {
            type Output = Self;
            fn sub(self, rhs: i16) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl core::ops::SubAssign<i16> for $name {
            fn sub_assign(&mut self, rhs: i16) {
                self.0 -= rhs
            }
        }

        impl core::ops::DivAssign<i16> for $name {
            fn div_assign(&mut self, rhs: i16) {
                self.0 /= rhs
            }
        }

        impl core::cmp::PartialEq<i16> for $name {
            fn eq(&self, other: &i16) -> bool {
                self.0 == *other
            }
        }

        impl core::cmp::PartialOrd<i16> for $name {
            fn partial_cmp(&self, other: &i16) -> Option<core::cmp::Ordering> {
                i16::partial_cmp(&self.0, other)
            }
        }

        impl TryFrom<u64> for $name {
            type Error = core::num::TryFromIntError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                i16::try_from(value).map($name)
            }
        }

        impl TryFrom<$name> for u64 {
            type Error = core::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                u64::try_from(value.0)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

pub type HealthLevel = i16;
define_characteristic!(Money);
pub type BrainLevel = i16;
pub type StaminaLevel = i16;
pub type CharismaLevel = i16;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HealthAssessment {
    /// "живой труп"
    LivingDead,

    /// "пора помирать ..."
    TimeToDie,

    /// "плохое"
    Bad,

    /// "так себе"
    SoSo,

    /// "среднее"
    Average,

    /// "хорошее"
    Good,

    /// "отличное"
    Great,
}

pub(in crate::logic) const LOCATION_CHANGE_LARGE_HEALTH_PENALTY: HealthLevel = 3;

pub(in crate::logic) const LOCATION_CHANGE_SMALL_HEALTH_PENALTY: HealthLevel = 2;

impl HealthAssessment {
    pub fn from_health_level(level: HealthLevel) -> HealthAssessment {
        use HealthAssessment::*;
        let scale = [
            (1, LivingDead),
            (9, TimeToDie),
            (17, Bad),
            (25, SoSo),
            (33, Average),
            (41, Good),
        ];
        *crate::util::assess(&scale, &level, &Great)
    }
}

impl Money {
    pub const fn zero() -> Money {
        Money(0)
    }

    /// Стоимость настойки овса для Коли
    pub const fn oat_tincture_cost() -> Money {
        Money(15)
    }

    /// Размер стипендии
    pub const fn stipend() -> Money {
        Money(50)
    }

    /// Стоимость стакана колы в мавзолее
    pub const fn cola_cost() -> Money {
        Money(4)
    }

    /// Стоимость супа в мавзолее
    pub const fn soup_cost() -> Money {
        Money(6)
    }

    /// Стоимость пива в мавзолее
    pub const fn beer_cost() -> Money {
        Money(8)
    }

    /// Стоимость чая/кофе в буфете ПУНКа/ПОМИ
    pub const fn drink_cost() -> Money {
        Money(2)
    }

    /// Стоимость выпечки в буфете ПУНКа/ПОМИ
    pub const fn pastry_cost() -> Money {
        Money(4)
    }

    /// Стоимость чая/кофе с выпечкой в буфете ПУНКа/ПОМИ
    pub const fn drink_with_pastry_cost() -> Money {
        Money(6)
    }

    /// Стоимость билета на электричку в ПОМИ и обратно
    pub const fn roundtrip_train_ticket_cost() -> Money {
        Money(10)
    }

    /// Стоимость билета на электричку из ПОМИ в ПУНК
    pub const fn one_way_train_ticket_cost() -> Money {
        Money(5)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum KnowledgeAssessment {
    Bad,
    Satisfactory,
    Good,
    VeryGood,
    Excellent,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BrainAssessment {
    /// "Клиническая смерть мозга"
    ClinicalBrainDeath,

    /// "Голова просто никакая"
    BrainIsAlmostNonFunctioning,

    /// "Думать практически невозможно"
    ThinkingIsAlmostImpossible,

    /// "Думать трудно"
    ThinkingIsDifficult,

    /// "Голова почти в норме"
    BrainIsAlmostOK,

    /// "Голова в норме"
    BrainIsOK,

    /// "Голова свежая"
    BrainIsFresh,

    /// "Легкость в мыслях необыкновенная"
    ExtraordinaryEaseOfThought,

    /// "Обратитесь к разработчику ;)"
    ContactTheDeveloper, // TODO: Find out if this is ever reachable :)
}

impl BrainAssessment {
    pub fn from_brain_level(brain_level: BrainLevel) -> BrainAssessment {
        use BrainAssessment::*;
        let scale = [
            (0, ClinicalBrainDeath),
            (1, BrainIsAlmostNonFunctioning),
            (2, ThinkingIsAlmostImpossible),
            (3, ThinkingIsDifficult),
            (4, BrainIsAlmostOK),
            (5, BrainIsOK),
            (6, BrainIsFresh),
            (101, ExtraordinaryEaseOfThought),
        ];
        let assessment = *crate::util::assess(&scale, &brain_level, &ContactTheDeveloper);
        assert_ne!(assessment, ContactTheDeveloper, "Чересчур умный");
        assessment
    }
}
impl KnowledgeAssessment {
    pub fn absolute(knowledge: BrainLevel) -> KnowledgeAssessment {
        use KnowledgeAssessment::*;
        let scale = [(6, Bad), (13, Satisfactory), (21, Good), (31, VeryGood)];
        *crate::util::assess(&scale, &knowledge, &Excellent)
    }

    pub fn relative(knowledge: BrainLevel, subject: Subject) -> KnowledgeAssessment {
        *crate::util::assess(
            subject.assessment_bounds(),
            &knowledge,
            &KnowledgeAssessment::Excellent,
        )
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StaminaAssessment {
    /// "Мама, роди меня обратно!"
    MamaTakeMeBack,

    /// "Окончательно заучился"
    CompletelyOverstudied,

    /// "Я так больше немогууу!"
    ICantTakeIt,

    /// "Скорее бы все это кончилось..."
    IWishItAllEndedSoon,

    /// "Еще немного и пора отдыхать"
    ALittleMoreAndThenRest,

    /// "Немного устал"
    ABitTired,

    /// "Готов к труду и обороне"
    ReadyForEverything,

    /// "Нас ждут великие дела"
    GreatThingsAwaitUs,
}

impl StaminaAssessment {
    pub fn from_stamina_level(stamina_level: StaminaLevel) -> StaminaAssessment {
        use StaminaAssessment::*;
        let scale = [
            (0, MamaTakeMeBack),
            (1, CompletelyOverstudied),
            (2, ICantTakeIt),
            (3, IWishItAllEndedSoon),
            (4, ALittleMoreAndThenRest),
            (5, ABitTired),
            (6, ReadyForEverything),
        ];
        *crate::util::assess(&scale, &stamina_level, &GreatThingsAwaitUs)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CharismaAssessment {
    /// "Очень замкнутый товарищ"
    VeryIntroverted,

    /// "Предпочитаешь одиночество"
    PreferSolitariness,

    /// "Тебе трудно общаться с людьми"
    VeryHardToTalkToPeople,

    /// "Тебе непросто общаться с людьми"
    NotEasyToTalkToPeople,

    /// "Ты нормально относишься к окружающим"
    Normal,

    /// "У тебя много друзей"
    ManyFriends,

    /// "У тебя очень много друзей"
    TonsOfFriends,
}

impl CharismaAssessment {
    pub fn from_charisma_level(charisma_level: CharismaLevel) -> CharismaAssessment {
        use CharismaAssessment::*;
        let scale = [
            (1, VeryIntroverted),
            (2, PreferSolitariness),
            (3, VeryHardToTalkToPeople),
            (4, NotEasyToTalkToPeople),
            (5, Normal),
            (6, ManyFriends),
        ];
        *crate::util::assess(&scale, &charisma_level, &TonsOfFriends)
    }
}
