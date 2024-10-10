mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::scene_router::train::TrainToPDMI;
use mmheroes_core::logic::{
    Action, CauseOfDeath, GameMode, GameScreen, HealthLevel, Location, Money, Time,
};

#[test]
fn go_from_dorm_to_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    replay_game(game_ui, "4↓r");
    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.location(), Location::PUNK);
    });
}

#[test]
fn death_on_the_way_from_dorm_to_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "10r");

    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.player().health(), HealthLevel(2));
    });

    // Идём на факультет
    replay_game(game_ui, "4↓r");

    assert_matches!(state.observable_state().screen(), GameScreen::GameEnd(state) => {
        assert_matches!(state.player().cause_of_death(), Some(CauseOfDeath::OnTheWayToPUNK));
    });
}

#[test]
fn go_from_dorm_to_mausoleum() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    replay_game(game_ui, "6↓r");
    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.location(), Location::Mausoleum);
    });
}

#[test]
fn death_on_the_way_from_dorm_to_mausoleum() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "10r");

    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.player().health(), HealthLevel(2));
    });

    // Идём в мавзолей
    replay_game(game_ui, "6↓r");

    assert_matches!(state.observable_state().screen(), GameScreen::GameEnd(state) => {
        assert_matches!(state.player().cause_of_death(), Some(CauseOfDeath::OnTheWayToMausoleum));
    });
}

#[test]
fn no_point_to_go_to_pdmi() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 22:00
    for _ in 0..13 {
        replay_game(game_ui, "2↓r");
    }

    // Едем в ПОМИ
    replay_game(game_ui, "5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(_, TrainToPDMI::NoPointToGoToPDMI)
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(_)
    );
}

#[test]
fn go_to_pdmi_from_dorm_without_money_not_caught_by_inspectors() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    assert_matches!(state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.player().health(), HealthLevel(44))
        }
    );
    replay_game(game_ui, "5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashBecauseNoMoney {
                caught_by_inspectors: false
            }
        ) => {
            assert_eq!(state.current_time(), Time(8));
            assert_eq!(state.player().health(), HealthLevel(43))
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(9));
            assert_eq!(state.location(), Location::PDMI)
        }
    );
}

#[test]
fn go_to_pdmi_from_dorm_without_money_caught_by_inspectors() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);
    assert_matches!(state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.player().health(), HealthLevel(45));
            assert_eq!(state.current_time(), Time(8));
        }
    );
    replay_game(game_ui, "5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashBecauseNoMoney {
                caught_by_inspectors: true
            }
        ) => {
            assert_eq!(state.current_time(), Time(8));
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(26))
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(26));
        }
    );
}

#[test]
fn death_on_the_way_to_pdmi() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "10r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(13));
            assert_eq!(state.player().health(), HealthLevel(2));
        }
    );

    // Едем в ПОМИ
    replay_game(game_ui, "5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashBecauseNoMoney { caught_by_inspectors: false },
        ) => {
            assert_matches!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::CorpseFoundInTheTrain)
            );
            assert_eq!(state.current_time(), Time(13));
            assert_eq!(state.player().health(), HealthLevel(2));
            assert_eq!(state.location(), Location::PDMI)
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.current_time(), Time(14));
            assert_matches!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::CorpseFoundInTheTrain)
            );
            assert_eq!(state.location(), Location::PDMI)
        }
    );
}

#[test]
fn killed_by_inspectors_no_money_from_dorm_to_pdmi() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "8r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.player().health(), HealthLevel(13));
            assert_eq!(state.current_time(), Time(12))
        }
    );

    // Едем в ПОМИ
    replay_game(game_ui, "5↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashBecauseNoMoney { caught_by_inspectors: true },
        ) => {
            assert_matches!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::KilledByInspectors)
            );
            assert_eq!(state.current_time(), Time(12));
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(8));
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.current_time(), Time(14));
            assert_matches!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::KilledByInspectors)
            );
            assert_eq!(state.player().health(), HealthLevel(8));
            assert_eq!(state.location(), Location::PDMI)
        }
    );
}

#[test]
fn go_to_pdmi_with_money_but_without_ticket_not_caught_by_inspectors() {
    initialize_game!((2, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем пока на факультет не придёт Паша
    replay_game(game_ui, "2↓r2↓r");

    // Получаем у Паши стипендию
    replay_game(game_ui, "4↓r3↑2r");

    // Едем в ПОМИ
    replay_game(game_ui, "3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(_, TrainToPDMI::PromptToBuyTickets)
    );

    // Едем зайцем
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashByChoice {
                caught_by_inspectors: false
            }
        ) => {
            assert_eq!(state.player().health(), HealthLevel(52))
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(52));
            assert_eq!(state.player().money(), Money(50));
            assert!(!state.player().has_roundtrip_train_ticket());
        }
    );
}

#[test]
fn go_to_pdmi_with_money_but_without_ticket_caught_by_inspectors() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем пока на факультет не придёт Паша
    replay_game(game_ui, "2↓r2↓r");

    // Получаем у Паши стипендию
    replay_game(game_ui, "4↓r3↑2r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(state.player().health(), HealthLevel(64))
        }
    );

    // Едем в ПОМИ
    replay_game(game_ui, "3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(state, TrainToPDMI::PromptToBuyTickets) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(state.player().health(), HealthLevel(59))
        }
    );

    // Едем зайцем
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::GatecrashByChoice {
                caught_by_inspectors: true
            }
        ) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(state.player().health(), HealthLevel(59))
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.current_time(), Time(12));
            assert_eq!(state.player().health(), HealthLevel(59))
        }
    );
}

#[test]
fn go_to_pdmi_with_ticket() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем пока на факультет не придёт Паша
    replay_game(game_ui, "2↓r2↓r");

    // Получаем у Паши стипендию
    replay_game(game_ui, "4↓r3↑2r");

    // Едем в ПОМИ
    replay_game(game_ui, "3↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(_, TrainToPDMI::PromptToBuyTickets)
    );

    // Покупаем билет
    replay_game(game_ui, "↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::TrainToPDMI(
            state,
            TrainToPDMI::BoughtRoundtripTicket
        ) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(59));
            assert_eq!(state.player().money(), Money(50));
            assert!(!state.player().has_roundtrip_train_ticket());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PDMI);
            assert_eq!(state.player().health(), HealthLevel(59));
            assert_eq!(state.player().money(), Money(40));
            assert!(state.player().has_roundtrip_train_ticket());
        }
    );
}

#[test]
fn go_from_punk_to_dorm() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );

    // Идём обратно в общагу факультет и проверяем что здоровье не изменилось
    replay_game(game_ui, "2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Dorm);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );
}

#[test]
fn go_from_punk_to_computer_class() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );

    // Идём в компьютерный класс
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::ComputerClass);
            assert_eq!(state.player().health(), HealthLevel(39));
        }
    );

    // Возвращаемся в общагу
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Dorm);
            assert_eq!(state.player().health(), HealthLevel(39));
        }
    );

    // Отдыхаем до 19:00
    replay_game(game_ui, "2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.player().health(), HealthLevel(170));
        }
    );

    // Снова идём в компьютерный класс
    replay_game(game_ui, "4↓r4↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::ComputerClass);
            assert_eq!(state.player().health(), HealthLevel(165));
        }
    );

    // Возвращаемся в общагу и отдыхаем до 20:00
    replay_game(game_ui, "r2↓r");

    // Идём на факультет, убеждаемся что компьютерный класс уже закрыт
    replay_game(game_ui, "4↓r");
    assert!(!state
        .observable_state()
        .available_actions()
        .contains(&Action::GoToComputerClass));
}

#[test]
fn death_on_the_way_to_computer_class() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "10r");

    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.player().health(), HealthLevel(4));
    });

    // Идём в компьютерный класс
    replay_game(game_ui, "4↓r5↓r");

    assert_matches!(state.observable_state().screen(), GameScreen::GameEnd(state) => {
        assert_eq!(state.player().cause_of_death(), Some(CauseOfDeath::FellFromStairs));
    });
}

#[test]
fn go_from_punk_to_mausoleum() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём на факультет
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );

    // Идём в мавзолей
    replay_game(game_ui, "3↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.player().health(), HealthLevel(38));
        }
    );
}

#[test]
fn go_from_mausoleum_to_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём в мавзолей
    replay_game(game_ui, "6↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );

    // Идём в ПУНК
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().health(), HealthLevel(38));
        }
    );
}

#[test]
fn death_on_the_way_from_mausoleum_to_punk() {
    initialize_game!((1, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Учим алгебру пока уровень здоровья не упадёт до почти нуля
    replay_game(game_ui, "10r");

    assert_matches!(state.observable_state().screen(), GameScreen::SceneRouter(state) => {
        assert_eq!(state.player().health(), HealthLevel(4));
    });

    // Идём в мавзолей
    replay_game(game_ui, "6↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.player().health(), HealthLevel(1));
        }
    );

    // Идём в ПУНК
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.location(), Location::PUNK);
            assert_eq!(state.player().cause_of_death(), Some(CauseOfDeath::OnTheWayToPUNK));
        }
    );
}

#[test]
fn go_from_mausoleum_to_dorm() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Идём в мавзолей
    replay_game(game_ui, "6↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Mausoleum);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );

    // Идём в общагу
    replay_game(game_ui, "2↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.location(), Location::Dorm);
            assert_eq!(state.player().health(), HealthLevel(41));
        }
    );
}
