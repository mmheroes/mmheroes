mod common;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::GameMode;

#[test]
fn mausoleum_closing() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 23:00
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r↓2r2↓r2↓r2↓r");

    // Идём в мавзолей и отдыхаем там
    replay_game(game_ui, "6↓r3↓2r");
    assert_ui!(
        game_ui,
        "
Мавзолей закрывается.
Пора домой!





















Нажми любую клавишу ...▁
    "
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 23е мая; 0:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (207)                Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
Тебе непросто общаться с людьми (3)         Физ-ра                0   Плохо

Тебя неумолимо клонит ко сну ...















Нажми любую клавишу ...▁
    "
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 23е мая; 7:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
Тебе непросто общаться с людьми (3)         Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  13-15    0/12
Посмотреть расписание                            МатАн   ПУНК  11-14    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
    "
    );
}
