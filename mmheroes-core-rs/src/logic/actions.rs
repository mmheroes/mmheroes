use crate::logic::{Classmate, Subject};
use crate::util::TinyVec;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayStyle {
    RandomStudent,
    CleverStudent,
    ImpudentStudent,
    SociableStudent,
    GodMode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HelpAction {
    WhatToDoAtAll,
    AboutScreen,
    WhereToGoAndWhy,
    AboutProfessors,
    AboutCharacters,
    AboutThisProgram,
    ThanksButNothing,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    AnyKey,
    Yes,
    No,
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
    UseLectureNotes(Subject),
    DontUseLectureNotes(Subject),
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
    AcceptEmploymentAtTerkom,
    DeclineEmploymentAtTerkom,
    GoToComputerClass,
    LeaveComputerClass,
    GoToPDMI,
    GoToPUNKFromPDMI,
    BuyRoundtripTrainTicket,
    GatecrashTrain,
    GoToMausoleum,
    GoToCafePUNK,
    SurfInternet,
    PlayMMHEROES,
    EarnAtTerkom,
    SurfInternetAtTerkom,
    ExitTerkom,
    GoToProfessor,
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
    IAmDone,
    NoIAmNotDone,
    IAmCertainlyDone,
    WhatToDo,
    Help(HelpAction),
    WantToTryAgain,
    DontWantToTryAgain,
}

pub(in crate::logic) type ActionVec = TinyVec<Action, 16>;

macro_rules! illegal_action {
    ($action:expr) => {
        panic!("Illegal action: {:?}", $action)
    };
}

pub(in crate::logic) use illegal_action;
