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

define_characteristic!(HealthLevel);
define_characteristic!(Money);
define_characteristic!(BrainLevel);
define_characteristic!(StaminaLevel);
define_characteristic!(CharismaLevel);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HealthAssessment {
    /// "?????????? ????????"
    LivingDead,

    /// "???????? ???????????????? ..."
    TimeToDie,

    /// "????????????"
    Bad,

    /// "?????? ????????"
    SoSo,

    /// "??????????????"
    Average,

    /// "??????????????"
    Good,

    /// "????????????????"
    Great,
}

impl HealthLevel {
    pub fn assessment(&self) -> HealthAssessment {
        use HealthAssessment::*;
        let scale = [
            (HealthLevel(1), LivingDead),
            (HealthLevel(9), TimeToDie),
            (HealthLevel(17), Bad),
            (HealthLevel(25), SoSo),
            (HealthLevel(33), Average),
            (HealthLevel(41), Good),
        ];
        *crate::util::assess(&scale, self, &Great)
    }

    pub(in crate::logic) const fn location_change_large_penalty() -> HealthLevel {
        HealthLevel(3)
    }

    pub(in crate::logic) const fn location_change_small_penalty() -> HealthLevel {
        HealthLevel(2)
    }
}

impl Money {
    pub const fn zero() -> Money {
        Money(0)
    }

    /// ?????????????????? ???????????????? ???????? ?????? ????????
    pub const fn oat_tincture_cost() -> Money {
        Money(15)
    }

    /// ???????????? ??????????????????
    pub const fn stipend() -> Money {
        Money(50)
    }

    /// ?????????????????? ?????????????? ???????? ?? ????????????????
    pub const fn cola_cost() -> Money {
        Money(4)
    }

    /// ?????????????????? ???????? ?? ????????????????
    pub const fn soup_cost() -> Money {
        Money(6)
    }

    /// ?????????????????? ???????? ?? ????????????????
    pub const fn beer_cost() -> Money {
        Money(8)
    }

    /// ?????????????????? ?????? ?? ???????????? ??????????/????????
    pub const fn tea_cost() -> Money {
        Money(2)
    }

    /// ?????????????????? ?????????? ?? ???????????? ??????????/????????
    pub const fn cake_cost() -> Money {
        Money(4)
    }

    /// ?????????????????? ?????? ?? ???????????????? ?? ???????????? ??????????/????????
    pub const fn tea_with_cake_cost() -> Money {
        Money(6)
    }

    /// ?????????????????? ???????????? ???? ???????????????????? ?? ????????
    pub const fn roundtrip_train_ticket_cost() -> Money {
        Money(10)
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
    /// "?????????????????????? ???????????? ??????????"
    ClinicalBrainDeath,

    /// "???????????? ???????????? ??????????????"
    BrainIsAlmostNonFunctioning,

    /// "???????????? ?????????????????????? ????????????????????"
    ThinkingIsAlmostImpossible,

    /// "???????????? ????????????"
    ThinkingIsDifficult,

    /// "???????????? ?????????? ?? ??????????"
    BrainIsAlmostOK,

    /// "???????????? ?? ??????????"
    BrainIsOK,

    /// "???????????? ????????????"
    BrainIsFresh,

    /// "???????????????? ?? ???????????? ????????????????????????????"
    ExtraordinaryEaseOfThought,

    /// "???????????????????? ?? ???????????????????????? ;)"
    ContactTheDeveloper, // TODO: Find out if this is ever reachable :)
}

impl BrainLevel {
    pub fn assessment(&self) -> BrainAssessment {
        use BrainAssessment::*;
        let scale = [
            (BrainLevel(0), ClinicalBrainDeath),
            (BrainLevel(1), BrainIsAlmostNonFunctioning),
            (BrainLevel(2), ThinkingIsAlmostImpossible),
            (BrainLevel(3), ThinkingIsDifficult),
            (BrainLevel(4), BrainIsAlmostOK),
            (BrainLevel(5), BrainIsOK),
            (BrainLevel(6), BrainIsFresh),
            (BrainLevel(101), ExtraordinaryEaseOfThought),
        ];
        *crate::util::assess(&scale, self, &ContactTheDeveloper)
    }

    pub fn absolute_knowledge_assessment(&self) -> KnowledgeAssessment {
        use KnowledgeAssessment::*;
        let scale = [
            (BrainLevel(6), Bad),
            (BrainLevel(13), Satisfactory),
            (BrainLevel(21), Good),
            (BrainLevel(31), VeryGood),
        ];
        *crate::util::assess(&scale, self, &Excellent)
    }

    pub fn relative_knowledge_assessment(&self, subject: Subject) -> KnowledgeAssessment {
        *crate::util::assess(
            &SUBJECTS[subject].assessment_bounds,
            self,
            &KnowledgeAssessment::Excellent,
        )
    }

    pub fn is_lethal(self) -> bool {
        self.0 > 45
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StaminaAssessment {
    /// "????????, ???????? ???????? ??????????????!"
    MamaTakeMeBack,

    /// "???????????????????????? ????????????????"
    CompletelyOverstudied,

    /// "?? ?????? ???????????? ????????????????!"
    ICantTakeIt,

    /// "???????????? ???? ?????? ?????? ??????????????????..."
    IWishItAllEndedSoon,

    /// "?????? ?????????????? ?? ???????? ????????????????"
    ALittleMoreAndThenRest,

    /// "?????????????? ??????????"
    ABitTired,

    /// "?????????? ?? ?????????? ?? ??????????????"
    ReadyForEverything,

    /// "?????? ???????? ?????????????? ????????"
    GreatThingsAwaitUs,
}

impl StaminaLevel {
    pub fn assessment(&self) -> StaminaAssessment {
        use StaminaAssessment::*;
        let scale = [
            (StaminaLevel(0), MamaTakeMeBack),
            (StaminaLevel(1), CompletelyOverstudied),
            (StaminaLevel(2), ICantTakeIt),
            (StaminaLevel(3), IWishItAllEndedSoon),
            (StaminaLevel(4), ALittleMoreAndThenRest),
            (StaminaLevel(5), ABitTired),
            (StaminaLevel(6), ReadyForEverything),
        ];
        *crate::util::assess(&scale, self, &GreatThingsAwaitUs)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CharismaAssessment {
    /// "?????????? ?????????????????? ??????????????"
    VeryIntroverted,

    /// "?????????????????????????? ??????????????????????"
    PreferSolitariness,

    /// "???????? ???????????? ???????????????? ?? ????????????"
    VeryHardToTalkToPeople,

    /// "???????? ???????????????? ???????????????? ?? ????????????"
    NotEasyToTalkToPeople,

    /// "???? ?????????????????? ???????????????????? ?? ????????????????????"
    Normal,

    /// "?? ???????? ?????????? ????????????"
    ManyFriends,

    /// "?? ???????? ?????????? ?????????? ????????????"
    TonsOfFriends,
}

impl CharismaLevel {
    pub fn assessment(&self) -> CharismaAssessment {
        use CharismaAssessment::*;
        let scale = [
            (CharismaLevel(1), VeryIntroverted),
            (CharismaLevel(2), PreferSolitariness),
            (CharismaLevel(3), VeryHardToTalkToPeople),
            (CharismaLevel(4), NotEasyToTalkToPeople),
            (CharismaLevel(5), Normal),
            (CharismaLevel(6), ManyFriends),
        ];
        *crate::util::assess(&scale, self, &TonsOfFriends)
    }
}
