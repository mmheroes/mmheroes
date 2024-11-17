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
pub enum TrainTicketAction {
    GatecrashTrain,
    BuyRoundtripTrainTicket,
}

action_conversion!(TrainTicketAction, TrainTicket);

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum MmheroesFloppyAction {
    WantToTestNewMMHEROES,
    DontWantToTestNewMMHEROES,
}

action_conversion!(MmheroesFloppyAction, MmheroesFloppy);

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
    TrainTicket(TrainTicketAction),
    GoToMausoleum,
    GoToCafePUNK,
    SurfInternet,
    PlayMMHEROES,
    EarnAtTerkom,
    SurfInternetAtTerkom,
    ExitTerkom,
    GoToProfessor,
    SufferMore,
    ExitExam,
    GoToWork,
    LookAtBaobab,
    LookAtBulletinBoard,
    OrderCola,
    OrderSoup,
    OrderBeer,
    OrderTea,
    OrderCake,
    OrderTeaWithCake,
    RestInCafePUNK,
    ShouldntHaveComeToCafePUNK,
    RestInCafePDMI,
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
