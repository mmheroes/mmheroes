mod common;

use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::GameMode;

#[test]
fn try_to_sleep() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (44)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  13-15    0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  14-16    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );

    replay_game(game_ui, "3↓r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (44)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться                                       АиТЧ    ПУНК  13-15    0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  14-16    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???



Тебя чего-то не тянет по-спать...

Нажми любую клавишу ...▁
"
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (44)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  13-15    0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  14-16    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn rest_until_midnight() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 20:00
    for _ in 0..12 {
        replay_game(game_ui, "2↓r");
    }

    // Отказываемся от приглашения соседа
    replay_game(game_ui, "↓2r");

    // Отдыхаем до 24:00
    for _ in 0..3 {
        replay_game(game_ui, "2↓r");
    }
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 23:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (209)                Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
Тебе непросто общаться с людьми (3)         Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  13-15    0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  14-16    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );

    replay_game(game_ui, "2↓r");
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

#[test]
fn cant_stay_awake() {
    initialize_game!((5, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    replay_game(
        game_ui,
        "2r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓7r2↓5r2↓3r2↓5r2↓3r2↓3r2↓2r↓r",
    );
    assert_ui!(
        game_ui,
        "
Сегодня 23е мая; 22:00   Версия gamma3.14   Алгебра и Т.Ч.        55  Отлично
Самочувствие: плохое (9)                    Мат. Анализ           8   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 7   Удовл.
Легкость в мыслях необыкновенная (6)        Информатика           3   Плохо
Готов к труду и обороне (5)                 English               3   Плохо
У тебя очень много друзей (7)               Физ-ра                3   Плохо

Тебя неумолимо клонит ко сну ...















Нажми любую клавишу ...▁
"
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 24е мая; 7:00    Версия gamma3.14   Алгебра и Т.Ч.        55  Отлично
Самочувствие: среднее (26)                  Мат. Анализ           8   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 7   Удовл.
Легкость в мыслях необыкновенная (6)        Информатика           3   Плохо
Готов к труду и обороне (5)                 English               3   Плохо
У тебя очень много друзей (7)               Физ-ра                3   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ----           0/12
Посмотреть расписание                            МатАн   ПУНК  15-18    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     Компы 9-10     0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn stupid_dream() {
    initialize_game!((3, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Отдыхаем до 18:00
    for _ in 0..10 {
        replay_game(game_ui, "2↓r");
    }

    // Отказываемся от приглашения соседа, отдыхаем до 20:00 и идём спать
    replay_game(game_ui, "↓2r2↓r2↓r3↓r");
    assert_ui!(
        game_ui,
        "
Розовые слоники с блестящими крылышками
сидят с окосевшими глазами в Мавзолее
и решают задачи по математической болтологии
...
▁
"
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Розовые слоники с блестящими крылышками
сидят с окосевшими глазами в Мавзолее
и решают задачи по математической болтологии
...

Господи! Ну и присниться же такое!
За то теперь ты точно знаешь,
что снится студентам-математикам,
когда они вне кондиции
...
▁
"
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        "
Сегодня 23е мая; 5:00    Версия gamma3.14   Алгебра и Т.Ч.        0   Плохо
Самочувствие: так себе (18)                 Мат. Анализ           1   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 0   Плохо
Думать трудно (2)                           Информатика           0   Плохо
Скорее бы все это кончилось... (2)          English               0   Плохо
У тебя очень много друзей (7)               Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ----           0/12
Посмотреть расписание                            МатАн   ПУНК  10-13    0/10
Отдыхать                                         ГиТ     ПОМИ  15-17    0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn djug_dream() {
    initialize_game!((2, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 14:00 и едем в ПОМИ на зачёт по геометрии
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r5↓4r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 16:00   Версия gamma3.14   Алгебра и Т.Ч.        4   Плохо
Самочувствие: отличное (85)                 Мат. Анализ           4   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 2   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
Ты нормально относишься к окружающим (4)    Физ-ра                2   Плохо
У тебя еще ничего не зачтено.
Сейчас тебя истязает Подкорытов С.С..
Кроме тебя, здесь еще сидят Паша, Миша, Серж, NiL и DJuG.


Мучаться дальше▁                                 АиТЧ    ----           0/12
Паша                                             МатАн   ----           0/10
Миша                                             ГиТ     ПОМИ  15-18    0/3
Серж                                             Инф     ----           0/2
NiL                                              ИнЯз    ----           0/3
DJuG                                             Физ-ра  ПУНК  16-17    0/1
Бросить это дело
"
    );

    // Играем с Мишей в клоподавку чтобы прошёл час и DJuG подействовал на нас
    replay_game(game_ui, "2↓3r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 17:00   Версия gamma3.14   Алгебра и Т.Ч.        4   Плохо
Самочувствие: отличное (79)                 Мат. Анализ           4   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 2   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
У тебя много друзей (5)                     Физ-ра                2   Плохо

Сейчас тебя истязает Подкорытов С.С..
Кроме тебя, здесь еще сидят Паша, Diamond, Миша, Серж, NiL, DJuG и Эндрю
.

К тебе пристает NiL. Что будешь делать?


Пытаться игнорировать▁                           АиТЧ    ----           0/12
NiL                                              МатАн   ----           0/10
                                                 ГиТ     ПОМИ  15-18    0/3
                                                 Инф     ----           0/2
                                                 ИнЯз    ----           0/3
                                                 Физ-ра  ПУНК  16-17    0/1
"
    );

    // Игнорируем NiL и едем обратно в ПУНК
    replay_game(game_ui, "2r↑r2↑2r");

    // Идём в общагу и отдыхаем до полуночи
    replay_game(game_ui, "2↓r2↓r2↓r↓2r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
Оно большое ...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
Оно большое ...
Оно пыхтит! ...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
Оно большое ...
Оно пыхтит! ...
Оно медленно ползет прямо на тебя!!! ...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
Оно большое ...
Оно пыхтит! ...
Оно медленно ползет прямо на тебя!!! ...
Оно говорит человеческим голосом:
"Это в средневековье ученые спорили,
сколько чертей может поместиться
на кончике иглы..."
...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
"Здравствуйте!" ...
Оно большое ...
Оно пыхтит! ...
Оно медленно ползет прямо на тебя!!! ...
Оно говорит человеческим голосом:
"Это в средневековье ученые спорили,
сколько чертей может поместиться
на кончике иглы..."
...

Уффф... Что-то сегодня опять какие-то гадости снятся.
Все, пора завязывать с этим. Нельзя так много учиться.
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        4   Плохо
Самочувствие: так себе (18)                 Мат. Анализ           4   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 2   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Нас ждут великие дела (6)                   English               4   Плохо
Ты нормально относишься к окружающим (4)    Физ-ра                2   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  12-16    0/12
Посмотреть расписание                            МатАн   ПУНК  11-13    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     Компы 15-17    0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn dream_about_algebra() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Просто отдыхаем, не ходя ни на какие зачёты.
    // По умолчанию должна присниться алгебра.
    for _ in 0..11 {
        replay_game(game_ui, "2↓r");
    }
    replay_game(game_ui, "2r");
    assert_ui!(
        game_ui,
        r#"
Ты слышишь мягкий, ненавязчивый голос:
"А Вы действительно правильно выбрали
 себе специальность?"

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r#"
Сегодня 23е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        3   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           1   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 1   Плохо
Легкость в мыслях необыкновенная (6)        Информатика           2   Плохо
Готов к труду и обороне (5)                 English               0   Плохо
У тебя много друзей (5)                     Физ-ра                4   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  10-14    0/12
Посмотреть расписание                            МатАн   ПУНК  10-12    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  12-14    0/3
Поехать в ПОМИ                                   Физ-ра  ПУНК  15-16    0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"#
    );
}

#[test]
fn dream_about_calculus() {
    initialize_game!((33, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем и идём на зачёт по анализу, после чего сразу уходим в общагу
    replay_game(game_ui, "2↓r2↓r4↓3r↑r2↓r");

    // Отдыхаем до полуночи, отказываясь от приглашения соседа
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r↓2r2↓r2↓r2↓r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"Интеграл..."
"Какой интеграл?"
"Да вот же он, мы его только что стерли!"

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        1   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова в норме (4)                          Информатика           1   Плохо
Нас ждут великие дела (6)                   English               0   Плохо
Тебе непросто общаться с людьми (3)         Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ----           0/12
Посмотреть расписание                            МатАн   ----           0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  12-14    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn dream_about_geometry() {
    initialize_game!((4, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем и идём на зачёт по геометрии, после чего сразу уходим в общагу
    replay_game(game_ui, "2↓r2↓r4↓4r↓r2↓r");

    // Отдыхаем до полуночи, отказываясь от приглашения соседа
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r↓2r2↓r2↓r2↓r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"Вы, конечно, великий парильщик.
 Но эту задачу я Вам засчитаю."

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 7:00    Версия gamma3.14   Алгебра и Т.Ч.        1   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 4   Плохо
Голова свежая (5)                           Информатика           1   Плохо
Готов к труду и обороне (5)                 English               4   Плохо
Тебе трудно общаться с людьми (2)           Физ-ра                4   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПОМИ  10-12    0/12
Посмотреть расписание                            МатАн   ПУНК  10-12    0/10
Отдыхать                                         ГиТ     ПОМИ  10-13    0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ----           0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn dream_about_computer_science() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем и идём на зачёт по информатике, после чего сразу уходим в общагу
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r4↓r5↓3r↑r↓r");

    // Отдыхаем, соглашаемся на приглашение соседа и отдыхаем до полуночи
    replay_game(game_ui, "2↓r2↓r2↓2r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"А что, у нас сегодня разве аудиторное занятие?"

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 7:00    Версия gamma3.14   Алгебра и Т.Ч.        3   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           2   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Легкость в мыслях необыкновенная (6)        Информатика           3   Плохо
Готов к труду и обороне (5)                 English               0   Плохо
У тебя много друзей (5)                     Физ-ра                3   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  10-14    0/12
Посмотреть расписание                            МатАн   ПУНК  10-12    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  12-14    0/3
Поехать в ПОМИ                                   Физ-ра  ПУНК  15-16    0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn dream_about_english() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем и идём на зачёт по английскому языку, после чего сразу уходим в общагу
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r4↓4r↑r2↓r");

    // Отдыхаем до полуночи
    replay_game(game_ui, "2↓r2↓r2↓2r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"Well, last time I found a pencil left by one of you.
 I will return it to the owner, if he or she
 can tell me some nice and pleasant words.
 I am a lady, not your computer!"

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        3   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           2   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 13  Хорошо
Легкость в мыслях необыкновенная (6)        Информатика           3   Плохо
Готов к труду и обороне (5)                 English               0   Плохо
Ты нормально относишься к окружающим (4)    Физ-ра                4   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ПУНК  10-14    0/12
Посмотреть расписание                            МатАн   ПУНК  10-12    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  12-14    0/3
Поехать в ПОМИ                                   Физ-ра  ПУНК  15-16    0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}

#[test]
fn dream_about_physical_education() {
    initialize_game!((12, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем и идём на зачёт по физкультуре, после чего сразу уходим в общагу
    replay_game(game_ui, "2↓r2↓r2↓r2↓r4↓3r↓r2↓r");

    // Отдыхаем до полуночи, отказываясь от приглашения соседа
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r↓2r2↓r2↓r2↓r2↓r");
    assert_ui!(
        game_ui,
        r#"
"В следующем семестре вы должны будете написать реферат
 на тему "Бег в мировой литературе". В качестве первоисточника
 можете взять одноименный роман Булгакова."

Ну все, похоже, заучился - если преподы по ночам снятся...
▁
"#
    );

    replay_game(game_ui, "r");
    assert_ui!(
        game_ui,
        r"
Сегодня 23е мая; 7:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (50)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 1   Плохо
Голова в норме (4)                          Информатика           3   Плохо
Немного устал (4)                           English               2   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты в общаге. Что делать?

Готовиться▁                                      АиТЧ    ----           0/12
Посмотреть расписание                            МатАн   ПУНК  12-14    0/10
Отдыхать                                         ГиТ     ----           0/3
Лечь спать                                       Инф     ----           0/2
Пойти на факультет                               ИнЯз    ПУНК  11-13    0/3
Поехать в ПОМИ                                   Физ-ра  ----           0/1
Пойти в мавзолей
С меня хватит!
ЧТО ДЕЛАТЬ ???
"
    );
}
