mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{
    Action, GameMode, GameScreen, HealthLevel, Location, Money, Time,
};

#[test]
fn cafe_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет и убеждаемся что кафе ещё закрыто
    replay_game(game_ui, "4↓r");
    assert!(!state
        .borrow()
        .available_actions()
        .contains(&Action::GoToCafePUNK));

    // Отдыхаем до 9:00 и снова идём на факультет
    replay_game(game_ui, "2↓r2↓r4↓r");

    // Кафе закрыто
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
        }
    );
    assert!(!state
        .borrow()
        .available_actions()
        .contains(&Action::GoToCafePUNK));

    // Отдыхаем до 10:00 и снова идём на факультет. Заходим в кафе.
    replay_game(game_ui, "2↓r2↓r4↓r6↓r");
    assert_eq!(
        state.borrow().available_actions(),
        [Action::RestInCafePUNK, Action::ShouldntHaveComeToCafePUNK]
    );

    // Получаем деньги у Паши, снова идём в кафе
    replay_game(game_ui, "↓r3↑2r6↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::CafePUNK(state) => {
            assert_eq!(state.player().health(), HealthLevel(58));
            assert_eq!(state.player().money(), Money(50));
            assert_eq!(state.current_time(), Time(10));
        }
    );
    assert_eq!(
        state.borrow().available_actions(),
        [
            Action::OrderTea,
            Action::OrderCake,
            Action::OrderTeaWithCake,
            Action::RestInCafePUNK,
            Action::ShouldntHaveComeToCafePUNK
        ]
    );

    // Заказываем чай
    replay_game(game_ui, "r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(62));
            assert_eq!(state.player().money(), Money(48));
            assert_eq!(state.current_time(), Time(11));
        }
    );

    // Снова идём в кафе, заказываем кекс
    replay_game(game_ui, "6↓r↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(67));
            assert_eq!(state.player().money(), Money(44));
            assert_eq!(state.current_time(), Time(12));
        }
    );

    // Снова идём в кафе, заказываем чай и выпечку
    replay_game(game_ui, "6↓r2↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(78));
            assert_eq!(state.player().money(), Money(38));
            assert_eq!(state.current_time(), Time(13));
        }
    );

    // Снова идём в кафе, просто отдыхаем
    replay_game(game_ui, "6↓r3↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(81));
            assert_eq!(state.player().money(), Money(38));
            assert_eq!(state.current_time(), Time(14));
        }
    );

    // Снова идём в кафе, после чего сразу же выходим обратно
    replay_game(game_ui, "6↓r4↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(81));
            assert_eq!(state.player().money(), Money(38));
            assert_eq!(state.current_time(), Time(14));
        }
    );

    // Отдыхаем в кафе до 18:00.
    replay_game(game_ui, "6↓r3↓r6↓r3↓r6↓r3↓r6↓r3↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(93));
            assert_eq!(state.player().money(), Money(38));
            assert_eq!(state.current_time(), Time(18));
        }
    );
    assert!(state
        .borrow()
        .available_actions()
        .contains(&Action::GoToCafePUNK));

    // Отдыхаем в кафе до 19:00 и убеждаемся что оно закрывается
    replay_game(game_ui, "6↓r3↓r");
    assert_matches!(
        state.borrow().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(94));
            assert_eq!(state.player().money(), Money(38));
            assert_eq!(state.current_time(), Time(19));
        }
    );
    assert!(!state
        .borrow()
        .available_actions()
        .contains(&Action::GoToCafePUNK));
}

#[test]
fn cafe_punk_limited_menu() {
    {
        initialize_game!((0, GameMode::Normal) => state, game_ui);
        replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

        // Отдыхаем до 10:00, идём на факультет и получаем стипендию
        replay_game(game_ui, "2↓r2↓r4↓r3↑2r");

        // Идём в кафе, заказываем пока не останется 4 рубля
        for _ in 0..7 {
            replay_game(game_ui, "6↓r2↓r");
        }
        replay_game(game_ui, "6↓r↓r");
        assert_matches!(
            state.borrow().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.location(), Location::PUNK);
                assert_eq!(state.player().health(), HealthLevel(132));
                assert_eq!(state.player().money(), Money(4));
                assert_eq!(state.current_time(), Time(18));
            }
        );

        // Снова идём в кафе. Меню должно быть меньше, т.к. у нас мало денег.
        replay_game(game_ui, "6↓r");
        assert_eq!(
            state.borrow().available_actions(),
            [
                Action::OrderTea,
                Action::OrderCake,
                Action::RestInCafePUNK,
                Action::ShouldntHaveComeToCafePUNK
            ],
        );
    }
    {
        initialize_game!((0, GameMode::Normal) => state, game_ui);
        replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

        // Отдыхаем до 10:00, идём на факультет и получаем стипендию
        replay_game(game_ui, "2↓r2↓r4↓r3↑2r");

        // Идём в кафе, заказываем пока не останется 2 рубля
        for _ in 0..8 {
            replay_game(game_ui, "6↓r2↓r");
        }
        assert_matches!(
            state.borrow().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.location(), Location::PUNK);
                assert_eq!(state.player().health(), HealthLevel(135));
                assert_eq!(state.player().money(), Money(2));
                assert_eq!(state.current_time(), Time(18));
            }
        );

        // Снова идём в кафе. Меню должно быть меньше, т.к. у нас мало денег.
        replay_game(game_ui, "6↓r");
        assert_eq!(
            state.borrow().available_actions(),
            [
                Action::OrderTea,
                Action::RestInCafePUNK,
                Action::ShouldntHaveComeToCafePUNK
            ],
        );
    }
}
