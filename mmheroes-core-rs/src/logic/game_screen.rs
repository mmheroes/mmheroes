use super::*;

#[derive(Debug)]
pub enum GameScreen {
    /// Самый первый экран, который видит пользователь.
    Intro,

    /// Экран, который видит пользователь, если запускает игру с каким-то аргументом.
    /// Предлагает выбрать стиль игры.
    InitialParameters,

    /// Экран с предысторией ("ты просыпаешься от звонка будильника...")
    Ding(Player),

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

    /// Кафе в ПУНКе
    CafePUNK(GameState),

    /// Взаимодействие с Колей.
    KolyaInteraction(GameState, npc::KolyaInteraction),

    /// Взаимодействие с Пашей.
    PashaInteraction(GameState, npc::PashaInteraction),

    /// Взаимодействие с Гришей.
    GrishaInteraction(GameState, npc::GrishaInteraction),

    /// Взаимодействие с Сашей.
    SashaInteraction(GameState, npc::SashaInteraction),

    /// Взаимодействие с Кузьменко.
    KuzmenkoInteraction(GameState, npc::KuzmenkoInteraction),

    /// Экран "Идти к преподу"
    GoToProfessor(GameState),

    /// Экран сдачи зачёта.
    Exam(GameState, Subject),

    // TODO: Добавить больше параметров. Сейчас поддерживается только "не тянет поспать"
    /// Сон.
    Sleep(GameState),

    /// Посидеть в интернете. Если второй аргумент `true`, это означает, что
    /// герой нашёл в интернете решение задачи по информатике.
    SurfInternet(GameState, bool),

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
