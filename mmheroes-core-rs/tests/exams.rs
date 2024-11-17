mod common;

use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::GameMode;

#[test]
fn exam_list_in_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет. Утром на факультете никого нет.
    // Список экзаменов в первый день:
    // - Алгебра 13:00–15:00
    // - English 14:00-16:00
    replay_game(game_ui, "4↓2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 8:00    Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (41)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты сейчас на факультете. К кому идти?

Ни к кому▁
"
    );

    // Идём в общагу и отдыхаем до 13:00
    replay_game(game_ui, "r2↓r2↓r2↓r2↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(game_ui, "4↓2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 13:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (96)                 Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты сейчас на факультете. К кому идти?

Всемирнов М.А.▁
Ни к кому
"
    );

    // Идём в общагу и отдыхаем до 14:00
    replay_game(game_ui, "↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(game_ui, "4↓2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 14:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (106)                Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты сейчас на факультете. К кому идти?

Всемирнов М.А.▁
Влащенко Н.П.
Ни к кому
"
    );

    // Идём в общагу и отдыхаем до 15:00
    replay_game(game_ui, "2↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(game_ui, "4↓2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 15:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (116)                Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты сейчас на факультете. К кому идти?

Влащенко Н.П.▁
Ни к кому
"
    );

    // Идём в общагу и отдыхаем до 16:00
    replay_game(game_ui, "↓r2↓r2↓r");

    // Снова идём на факультет.
    replay_game(game_ui, "4↓2r");
    assert_ui!(
        game_ui,
        "
Сегодня 22е мая; 16:00   Версия gamma3.14   Алгебра и Т.Ч.        2   Плохо
Самочувствие: отличное (124)                Мат. Анализ           0   Плохо
Финансы: Надо получить деньги за май...     Геометрия и Топология 3   Плохо
Голова свежая (5)                           Информатика           0   Плохо
Немного устал (4)                           English               4   Плохо
У тебя много друзей (5)                     Физ-ра                0   Плохо

Ты сейчас на факультете. К кому идти?

Ни к кому▁
"
    );
}
