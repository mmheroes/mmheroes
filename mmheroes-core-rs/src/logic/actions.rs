use crate::logic::{Classmate, Subject};
use crate::util::TinyVec;
use strum::EnumIter;

macro_rules! action_conversion {
    ($sub_action:ty, $action:ident) => {
        impl core::convert::From<$sub_action> for $crate::logic::Action {
            fn from(value: $sub_action) -> Self {
                Self::$action(value)
            }
        }

        impl core::convert::TryFrom<$crate::logic::Action> for $sub_action {
            type Error = ();

            fn try_from(action: $crate::logic::Action) -> Result<Self, Self::Error> {
                match action {
                    $crate::logic::Action::$action(value) => Ok(value),
                    _ => Err(()),
                }
            }
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum PlayStyle {
    RandomStudent,
    CleverStudent,
    ImpudentStudent,
    SociableStudent,
    GodMode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum HelpAction {
    WhatToDoAtAll,
    AboutScreen,
    WhereToGoAndWhy,
    AboutProfessors,
    AboutCharacters,
    AboutThisProgram,
    ThanksButNothing,
}

action_conversion!(HelpAction, Help);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum TerkomEmploymentAction {
    Accept,
    Decline,
}

action_conversion!(TerkomEmploymentAction, TerkomEmployment);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum UseLectureNotesAction {
    Yes,
    No,
}

action_conversion!(UseLectureNotesAction, UseLectureNotes);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum YesOrNoAction {
    Yes,
    No,
}

action_conversion!(YesOrNoAction, YesOrNo);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum GameEndAction {
    NoIAmNotDone,
    IAmCertainlyDone,
}

action_conversion!(GameEndAction, GameEnd);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum TryAgainAction {
    WantToTryAgain,
    DontWantToTryAgain,
}

action_conversion!(TryAgainAction, TryAgain);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum MmheroesFloppyAction {
    WantToTestNewMMHEROES,
    DontWantToTestNewMMHEROES,
}

action_conversion!(MmheroesFloppyAction, MmheroesFloppy);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NpcApproachAction {
    Ignore,
    TalkToClassmate(Classmate),
}

action_conversion!(NpcApproachAction, NpcApproach);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum ContinueSufferingWithExamInTrainAction {
    WantToSufferMore,
    NoThanks,
}

action_conversion!(
    ContinueSufferingWithExamInTrainAction,
    ContinueSufferingWithExamInTrain
);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum BaltiyskiyRailwayStationAction {
    GoToPUNK,
    GoToPDMI,
}

action_conversion!(BaltiyskiyRailwayStationAction, BaltiyskiyRailwayStation);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum RaiAction {
    YesOfCourse,
    NoSorry,
}

action_conversion!(RaiAction, Rai);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum NilAction {
    YesOfCourse,
    MaybeNextTime,
}

action_conversion!(NilAction, Nil);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum BugSquasherAction {
    LetsGo,
    NoIWontPlay,
}

action_conversion!(BugSquasherAction, BugSquasher);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum TennisAction {
    Sure,
    SorryMaybeLater,
}

action_conversion!(TennisAction, Tennis);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum HelpFromAndrewAction {
    YesAmIWorseThanEveryoneElse,
    IWillDoItMyself,
}

action_conversion!(HelpFromAndrewAction, HelpFromAndrew);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum InvitationFromNeighborAction {
    Accept,
    Deny,
}

action_conversion!(InvitationFromNeighborAction, InvitationFromNeighbor);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    AnyKey,
    YesOrNo(YesOrNoAction),
    InteractWithClassmate(Classmate),
    Exam(Subject),
    DontGoToProfessor,
    SelectPlayStyle(PlayStyle),
    Study,
    DoStudy {
        subject: Subject,
        lecture_notes_available: bool,
    },
    DontStudy,
    UseLectureNotes(UseLectureNotesAction),
    RequestLectureNotesFromSasha(Subject),
    DontNeedAnythingFromSasha,
    ViewTimetable,
    Rest,
    GoToBed,
    InvitationFromNeighbor(InvitationFromNeighborAction),
    GoFromPunkToDorm,
    GoFromDormToPunk,
    GoFromMausoleumToDorm,
    GoFromMausoleumToPunk,
    RestByOurselvesInMausoleum,
    NoRestIsNoGood,
    TerkomEmployment(TerkomEmploymentAction),
    GoToComputerClass,
    LeaveComputerClass,
    GoToPDMI,
    GoToPUNKFromPDMI,
    TrainToPDMIGatecrash,
    TrainToPDMIBuyTicket,
    TrainFromPDMIGatecrash,
    TrainFromPDMIBuyTicket,
    GoToMausoleum,
    GoToCafePUNK,
    SurfInternet,
    PlayMMHEROES,
    EarnAtTerkom,
    SurfInternetAtTerkom,
    ExitTerkom,
    GoToProfessor,
    SufferMore,
    NpcApproach(NpcApproachAction),
    ExitExam,
    ContinueSufferingWithExamInTrain(ContinueSufferingWithExamInTrainAction),
    BaltiyskiyRailwayStation(BaltiyskiyRailwayStationAction),
    Rai(RaiAction),
    Nil(NilAction),
    BugSquasher(BugSquasherAction),
    Tennis(TennisAction),
    HelpFromAndrew(HelpFromAndrewAction),
    GoToWork,
    LookAtBaobab,
    LookAtBulletinBoard,
    OrderCola,
    OrderSoup,
    OrderBeer,
    OrderTea,
    OrderCake,
    OrderTeaWithCake,
    OrderCoffee,
    OrderPastry,
    OrderCoffeeWithPastry,
    RestInCafePUNK,
    RestInCafePDMI,
    ShouldntHaveComeToCafePUNK,
    LeaveCafePDMI,
    GoToCafePDMI,
    MmheroesFloppy(MmheroesFloppyAction),
    IAmDone,
    GameEnd(GameEndAction),
    WhatToDo,
    Help(HelpAction),
    TryAgain(TryAgainAction),
}

pub(in crate::logic) type ActionVec = TinyVec<Action, 16>;

macro_rules! illegal_action {
    ($action:expr) => {
        panic!("Illegal action: {:?}", $action)
    };
}

pub(in crate::logic) use illegal_action;
