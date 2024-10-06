use crate::logic::{Classmate, GameScreen, GameState, InternalGameState, Subject};
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
    WhatToDoAtAll,
    WantToTryAgain,
    DontWantToTryAgain,
    AboutScreen,
    WhereToGoAndWhy,
    AboutProfessors,
    AboutCharacters,
    AboutThisProgram,
    ThanksButNothing,
}

pub(in crate::logic) type ActionVec = TinyVec<Action, 16>;

pub(in crate::logic) fn wait_for_any_key() -> ActionVec {
    ActionVec::from([Action::AnyKey])
}

pub(in crate::logic) fn go_to_professor(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    let mut available_actions = state
        .current_day()
        .current_exams(state.location, state.current_time)
        .map(|exam| Action::Exam(exam.subject()))
        .collect::<ActionVec>();
    available_actions.push(Action::DontGoToProfessor);
    game.set_screen(GameScreen::GoToProfessor(state));
    available_actions
}

macro_rules! illegal_action {
    ($action:expr) => {
        panic!("Illegal action: {:?}", $action)
    };
}

pub(in crate::logic) use illegal_action;
