use super::subjects::Subject;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CauseOfDeath {
    /// Умер по пути на факультет.
    OnTheWayToPUNK,

    /// Умер по пути в мавзолей.
    OnTheWayToMausoleum,

    /// Умер по пути домой. Бывает.
    OnTheWayToDorm,

    /// Упал с лестницы у главного входа.
    FellFromStairs,

    /// Сгорел на работе.
    Burnout,

    /// Заучился.
    Overstudied,

    /// Зубрежка до добра не доводит!
    StudiedTooWell,

    /// Не смог расстаться с компьютером.
    CouldntLeaveTheComputer,

    /// В электричке нашли бездыханное тело.
    CorpseFoundInTheTrain,

    /// Контролеры жизни лишили.
    KilledByInspectors,

    /// Заснул в электричке и не проснулся.
    FellAsleepInTheTrain,

    /// Раздвоение ложной личности.
    SplitPersonality,

    /// Пивной алкоголизм, батенька...
    BeerAlcoholism,

    /// Спился.
    DrankTooMuch,

    /// Губит людей не пиво, а избыток пива.
    DrankTooMuchBeer,

    /// Альтруизм не довел до добра.
    Altruism,

    /// Превратился в овощ.
    TurnedToVegetable,

    /// <препод> замучил.
    TorturedByProfessor(Subject),

    /// Бурно прогрессирующая паранойя
    Paranoia,

    /// Время вышло.
    TimeOut,

    /// Вышел сам.
    Suicide,

    /// Раздавлен безжалостной ошибкой в программе.
    SoftwareBug,
}
