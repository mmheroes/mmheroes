use super::*;
use crate::logic::scene_router::{dorm, terkom};

#[derive(Debug)]
pub enum GameScreen {
    /// Самый первый экран, который видит пользователь.
    Intro,

    /// Экран, который видит пользователь, если запускает игру с каким-то аргументом.
    /// Предлагает выбрать стиль игры.
    InitialParameters,

    /// Экран с предысторией ("ты просыпаешься от звонка будильника...")
    Ding,

    /// Экран с расписанием.
    Timetable(GameState),

    /// Главный экран.
    SceneRouter(GameState),

    /// Выбор предмета для подготовки к зачёту.
    Study(GameState),

    /// "Воспользуюсь конспектом" или "Буду учиться как умею"
    PromptUseLectureNotes(GameState),

    /// Экран с рекордами — баобаб в ПУНКе или доска объявлений в ПОМИ.
    HighScores(GameState),

    /// Отдых в мавзолее.
    RestInMausoleum(GameState),

    /// Кафе в ПУНКе или ПОМИ
    Cafe(GameState),

    /// В зависимости от локации, экран с сообщением о том, что пора домой.
    Midnight(GameState),

    /// Электричка из ПУНКа в ПОМИ
    TrainToPDMI(GameState, scene_router::train::TrainScene),

    /// Электричка из ПОМИ в ПУНК
    TrainFromPDMI(GameState, scene_router::train::TrainScene),

    /// Взаимодействие с Колей.
    KolyaInteraction(GameState, npc::kolya::KolyaInteraction),

    /// Взаимодействие с Пашей.
    PashaInteraction(GameState, npc::pasha::PashaInteraction),

    /// Взаимодействие с Гришей.
    GrishaInteraction(GameState, npc::grisha::GrishaInteraction),

    /// Взаимодействие с Сашей.
    SashaInteraction(GameState, npc::sasha::SashaInteraction),

    /// Взаимодействие с Кузьменко.
    KuzmenkoInteraction(GameState, npc::kuzmenko::KuzmenkoInteraction),

    /// Взаимодействие с Diamond.
    /// Третий аргумент означает, уходит ли Diamond после взаимодействия.
    DiamondInteraction(GameState, npc::diamond::DiamondInteraction, bool),

    /// Взаимодействие с Сержем.
    /// Третий аргумент означает, уходит ли Серж после взаимодействия.
    SerjInteraction(GameState, npc::serj::SerjInteraction, bool),

    /// Взаимодействие с Мишей
    MishaInteraction(npc::misha::MishaInteraction),

    /// Взаимодействие с Эндрю
    AndrewInteraction(npc::andrew::AndrewInteraction),

    /// Взаимодействие с DJuG
    DjugInteraction(GameState),

    /// Взаимодействие с RAI
    RaiInteraction(npc::rai::RaiInteraction),

    /// Взаимодействие с NiL
    NilInteraction(npc::nil::NilInteraction),

    /// Работа в ТЕРКОМе
    Terkom(GameState, terkom::Terkom),

    /// Экран "Идти к преподу"
    GoToProfessor(GameState),

    /// Опциональный экран с прелюдией к сдаче зачёта
    ExamIntro(scene_router::exams::ExamIntro),

    /// Экран сдачи зачёта.
    Exam(scene_router::exams::ExamScene),

    /// После сдачи зачёта в электричке оказался на Балтийском вокзале,
    /// дальше можно либо поехать в ПОМИ, либо обратно в ПУНК.
    BaltiyskiyRailwayStation(scene_router::train::BaltiyskiyRailwayStationScene),

    /// Экран "Тебя чего-то не тянет поспать"
    DontWantToSleep,

    /// Экран "Тебя неумолимо клонит ко сну ..."
    CantStayAwake(GameState),

    /// Экран, на котором сосед приглашает куда-то
    NeighborInvites(dorm::NeighborInvitation),

    /// Разные сновидения
    Dreaming(sleep::DreamScreen),

    /// Посидеть в интернете. Если `found_program` `true`, это означает, что
    /// герой нашёл в интернете решение задачи по информатике.
    SurfInternet { found_program: bool },

    /// Попытка поиграть в игру внутри игры.
    PlayMmheroes(scene_router::computer_class::PlayMmheroesScene),

    /// Экран "Класс закрывается. Пошли домой!"
    ComputerClassClosing(GameState),

    /// Экран "ты серьёзно хочешь закончить игру?"
    IAmDone(GameState),

    /// Финальный экран с описанием причины смерти/отчисления, либо поздравлением.
    GameEnd(GameState),

    /// Пользователю предлагается либо повторить игру, либо выйти.
    WannaTryAgain,

    /// Экран, который отображается пользователю, если он решил выйти из игры.
    Disclaimer,

    /// Экран помощи с описанием цели игры.
    WhatToDo(GameState),

    /// Экран помощи с описанием главного экрана.
    AboutScreen(GameState),

    /// Экран помощи с описанием локаций.
    WhereToGoAndWhy(GameState),

    /// Экран помощи с описанием преподавателей.
    AboutProfessors(GameState),

    /// Экран помощи с описанием NPC-шек.
    AboutCharacters(GameState),

    /// Экран помощи с информацией о программе.
    AboutThisProgram(GameState),

    /// Терминальное состояние. Ему тоже соответствует никакой экран.
    /// Игра завершена безвозвратно.
    Terminal,
}

impl GameScreen {
    /// Возвращает текущее состояние игры, если оно доступно.
    /// Оно может быть недоступно, например, если игра ещё не началась
    /// или уже закончилась.
    pub fn state(&self) -> Option<&GameState> {
        use crate::logic::scene_router::exams::ExamScene;
        use crate::logic::scene_router::train::BaltiyskiyRailwayStationScene;
        use GameScreen::*;
        match self {
            Timetable(state)
            | SceneRouter(state)
            | Midnight(state)
            | Study(state)
            | PromptUseLectureNotes(state)
            | HighScores(state)
            | IAmDone(state)
            | GameEnd(state)
            | WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state)
            | KolyaInteraction(state, _)
            | PashaInteraction(state, _)
            | GrishaInteraction(state, _)
            | SashaInteraction(state, _)
            | KuzmenkoInteraction(state, _)
            | DiamondInteraction(state, _, _)
            | SerjInteraction(state, _, _)
            | RaiInteraction(rai::RaiInteraction::Ignores(state))
            | RaiInteraction(rai::RaiInteraction::PromptWillYouHelpMe(state))
            | NilInteraction(nil::NilInteraction::WillYouHelpMe(state))
            | MishaInteraction(misha::MishaInteraction::PromptBugSquasher(state))
            | MishaInteraction(misha::MishaInteraction::PromptTennis(state))
            | MishaInteraction(misha::MishaInteraction::RandomReply(state, _))
            | AndrewInteraction(andrew::AndrewInteraction::PromptHelpFromAndrew(state))
            | DjugInteraction(state)
            | Terkom(state, _)
            | GoToProfessor(state)
            | Exam(ExamScene::Router(state, _))
            | Exam(ExamScene::ClassmateWantsSomething(state, _, _))
            | Exam(ExamScene::ProfessorLeaves(state, _))
            | Exam(ExamScene::ProfessorLingers(state, _))
            | Exam(ExamScene::PromptExamInTrain(state, _))
            | Exam(ExamScene::Train(state, _))
            | Exam(ExamScene::SufferInTrain { state, .. })
            | Exam(ExamScene::AlgebraExamPassed(state))
            | Exam(ExamScene::EnglishExamPassed(state, _))
            | Exam(ExamScene::ExamPassed(state, _))
            | BaltiyskiyRailwayStation(BaltiyskiyRailwayStationScene::Prompt(state))
            | RestInMausoleum(state)
            | Cafe(state)
            | ComputerClassClosing(state)
            | CantStayAwake(state)
            | NeighborInvites(dorm::NeighborInvitation::InvitePrompt(state, _))
            | TrainToPDMI(state, _)
            | TrainFromPDMI(state, _) => Some(state),
            Intro
            | InitialParameters
            | Ding
            | DontWantToSleep
            | NeighborInvites(dorm::NeighborInvitation::LetsGo)
            | NeighborInvites(dorm::NeighborInvitation::TooBad)
            | Dreaming(_)
            | SurfInternet { .. }
            | PlayMmheroes(_)
            | ExamIntro(_)
            | Exam(ExamScene::ExamSuffering { .. })
            | Exam(ExamScene::IgnoredClassmate { .. })
            | Exam(ExamScene::CaughtByInspectorsEmptyScreenBug)
            | BaltiyskiyRailwayStation(
                BaltiyskiyRailwayStationScene::CaughtByInspectors,
            )
            | RaiInteraction(rai::RaiInteraction::TakeIt)
            | RaiInteraction(rai::RaiInteraction::YouHelped)
            | RaiInteraction(rai::RaiInteraction::Fail)
            | NilInteraction(nil::NilInteraction::RefusedToHelp)
            | NilInteraction(nil::NilInteraction::ThanksHereIsYourMoney(_))
            | NilInteraction(nil::NilInteraction::DidntWorkOut)
            | MishaInteraction(misha::MishaInteraction::NoWorries)
            | MishaInteraction(misha::MishaInteraction::TooBad)
            | MishaInteraction(misha::MishaInteraction::PlayedBugSquasherWithMisha)
            | MishaInteraction(misha::MishaInteraction::PlayedTennisWithMisha)
            | AndrewInteraction(andrew::AndrewInteraction::ScorePrediction { .. })
            | AndrewInteraction(andrew::AndrewInteraction::RandomReply(_))
            | AndrewInteraction(andrew::AndrewInteraction::AndrewIgnoresYou)
            | AndrewInteraction(andrew::AndrewInteraction::AndrewSolvedProblems {
                ..
            })
            | WannaTryAgain
            | Disclaimer
            | Terminal => None,
        }
    }
}
