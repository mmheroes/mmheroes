use super::*;
use crate::logic::scene_router::terkom;

#[derive(Debug, Copy, Clone)]
pub enum GameScreen {
    /// Самый первый экран, который видит пользователь.
    Intro,

    /// Экран, который видит пользователь, если запускает игру с каким-то аргументом.
    /// Предлагает выбрать стиль игры.
    InitialParameters,

    /// Экран с предысторией ("ты просыпаешься от звонка будильника...")
    Ding,

    /// Экран с расписанием.
    Timetable,

    /// Главный экран.
    SceneRouter,

    /// Выбор предмета для подготовки к зачёту.
    Study,

    /// "Воспользуюсь конспектом" или "Буду учиться как умею"
    PromptUseLectureNotes,

    /// Экран с рекордами — баобаб в ПУНКе или доска объявлений в ПОМИ.
    HighScores,

    /// Отдых в мавзолее.
    RestInMausoleum,

    /// Кафе в ПУНКе
    CafePUNK,

    TrainToPDMI(scene_router::train::TrainScene),

    /// Взаимодействие с Колей.
    KolyaInteraction(npc::kolya::KolyaInteraction),

    /// Взаимодействие с Пашей.
    PashaInteraction(npc::pasha::PashaInteraction),

    /// Взаимодействие с Гришей.
    GrishaInteraction(npc::grisha::GrishaInteraction),

    /// Взаимодействие с Сашей.
    SashaInteraction(npc::sasha::SashaInteraction),

    /// Взаимодействие с Кузьменко.
    KuzmenkoInteraction(npc::kuzmenko::KuzmenkoInteraction),

    /// Взаимодействие с Diamond.
    /// Третий аргумент означает, уходит ли Diamond после взаимодействия.
    DiamondInteraction(npc::diamond::DiamondInteraction, bool),

    /// Взаимодействие с Сержем.
    /// Третий аргумент означает, уходит ли Серж после взаимодействия.
    SerjInteraction(npc::serj::SerjInteraction, bool),

    /// Работа в ТЕРКОМе
    Terkom(terkom::Terkom),

    /// Экран "Идти к преподу"
    GoToProfessor,

    /// Опциональный экран с прелюдией к сдаче зачёта
    ExamIntro(scene_router::exams::ExamIntro),

    /// Экран сдачи зачёта.
    Exam(scene_router::exams::ExamScene),

    /// После сдачи зачёта в электричке оказался на Балтийском вокзале,
    /// дальше можно либо поехать в ПОМИ, либо обратно в ПУНК.
    BaltiyskiyRailwayStation(scene_router::train::BaltiyskiyRailwayStationScene),

    // TODO: Добавить больше параметров. Сейчас поддерживается только "не тянет поспать"
    /// Сон.
    Sleep,

    /// Посидеть в интернете. Если `found_program` `true`, это означает, что
    /// герой нашёл в интернете решение задачи по информатике.
    SurfInternet {
        found_program: bool,
    },

    /// Экран "ты серьёзно хочешь закончить игру?"
    IAmDone,

    /// Финальный экран с описанием причины смерти/отчисления, либо поздравлением.
    GameEnd,

    /// Пользователю предлагается либо повторить игру, либо выйти.
    WannaTryAgain,

    /// Экран, который отображается пользователю, если он решил выйти из игры.
    Disclaimer,

    /// Экран помощи с описанием цели игры.
    WhatToDo,

    /// Экран помощи с описанием главного экрана.
    AboutScreen,

    /// Экран помощи с описанием локаций.
    WhereToGoAndWhy,

    /// Экран помощи с описанием преподавателей.
    AboutProfessors,

    /// Экран помощи с описанием NPC-шек.
    AboutCharacters,

    /// Экран помощи с информацией о программе.
    AboutThisProgram,

    /// Терминальное состояние. Ему тоже соответствует никакой экран.
    /// Игра завершена безвозвратно.
    Terminal,
}
