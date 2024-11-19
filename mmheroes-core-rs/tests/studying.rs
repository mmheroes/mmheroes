mod common;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{GameMode, Subject};

#[test]
fn overstudy_to_zero_health() {
    initialize_game!((1641333345581, GameMode::Normal) => state, game_ui);
    replay_game(game_ui, "13r");
    assert_ui!(game_ui, "asd;");

    replay_game(game_ui, "2r");
    assert_ui!(game_ui, "asd;");
    replay_game(game_ui, "2r");
    assert_ui!(game_ui, "asd;");
}

/// Проверяем, что в случае отрицательного brain level попытка подготовиться к зачёту
/// ни к чему не приводит: знание предмета не увеличивается, время не тратится.
#[test]
fn study_with_negative_brain_level() {
    initialize_game!((1641336778475, GameMode::Normal) => state, game_ui);
    replay_game(game_ui, "3r2↓r2↓r4↓r7↓2r4↓r3↑r↓2r3↑r↓2r4↓r↓2r3↑r↑2r3↑r↑2r3↑r↑2r3↑2r3↑2r2↑2r3↑2r3↑2r2↑r↓3r2↓2r");
    assert_ui!(game_ui, "asd;");
    replay_game(game_ui, "r");
    assert_ui!(game_ui, "asd;");
}

#[test]
fn died_of_studying_to_well() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Ждём когда на факультет приходит Саша
    replay_game(game_ui, "2↓r2↓r");

    // Идём на факультет, обращаемся к Саше
    replay_game(game_ui, "4↓r2↑r");

    // С трёх попыток Саша соглашается дать нам конспект по геометрии
    replay_game(game_ui, "2r2↑r↓2r2↑r2↓2r");

    assert!(state
        .game_state()
        .unwrap()
        .player()
        .status_for_subject(Subject::GeometryAndTopology)
        .has_lecture_notes());

    // Идём в общагу
    replay_game(game_ui, "2↓r");

    // Готовимся к геометрии 9 раз
    for _ in 0..9 {
        replay_game(game_ui, "r2↓2r");
    }

    // Умираем от зубрёжки
    assert_ui!(game_ui, "asd;");
}
