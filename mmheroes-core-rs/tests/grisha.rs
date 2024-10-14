mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::grisha::GrishaInteraction;
use mmheroes_core::logic::{Action, CauseOfDeath, Classmate, GameMode, GameScreen, Time};

#[test]
fn grisha() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Ждём когда в мавзолее появится Гриша, идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r6↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 64,
                money: 0,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Grisha)));

    // Подходим к Грише. Он поит пивом, проходит час.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::ThirdYearStudentsDontAttendLectures {
                drink_beer: true,
                hour_pass: true,
            }
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 64,
                money: 0,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(11));
            assert_characteristics!(
                state,
                health: 64,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    assert!(!state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Grisha)));

    // Гриша ушёл. Отдыхаем шесть часов, пока он не появится.
    replay_game(game_ui, "3↓2r3↓2r3↓2r3↓2r3↓2r3↓2r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Grisha)));

    // Снова подходим к Грише. Он предлагает устроиться в ТЕРКОМ
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::PromptEmploymentAtTerkom
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    // Соглашаемся
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::CongratulationsYouAreNowEmployed
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он подсказывает адрес прокси-сервера.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::ProxyAddress
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он просто выкидывает реплику.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::TakeExampleFromKolya {
                drink_beer: false,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NamesOfFreebieLovers {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он просто выкидывает реплику.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NoNeedToStudyToGetDiploma {
                drink_beer: false,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::SitHereAndChill {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он просто выкидывает реплику.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::YouStudiedDidItHelp {
                drink_beer: false,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::HateLevTolstoy {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 5,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 6,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он просто выкидывает реплику.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NoNeedToStudyToGetDiploma {
                drink_beer: false,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 6,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 6,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::MechanicsHaveFreebie {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 6,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он просто выкидывает реплику.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::ThirdYearStudentsDontAttendLectures {
                drink_beer: false,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::DontGoToPDMI {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 3,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 2,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NoNeedToStudyToGetDiploma {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 2,
                stamina: 4,
                charisma: 7,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 2,
                stamina: 4,
                charisma: 8,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом, проходит час
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::FreebieComeToMe {
                drink_beer: true,
                hour_pass: true,
            }
        ) => {
            assert_eq!(state.current_time(), Time(17));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 2,
                stamina: 4,
                charisma: 8,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(18));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 1,
                stamina: 4,
                charisma: 8,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он поит пивом, проходит час. Умираем от пива.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NamesOfFreebieLovers {
                drink_beer: true,
                hour_pass: true,
            }
        ) => {
            assert_eq!(state.current_time(), Time(18));
            assert_characteristics!(
                state,
                health: 70,
                money: 0,
                brain: 1,
                stamina: 4,
                charisma: 8,
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.current_time(), Time(19));
            assert_characteristics!(
                state,
                health: 0,
                money: 0,
                brain: 0,
                stamina: 4,
                charisma: 8,
            );
            assert_matches!(
                state.player().cause_of_death(),
                Some(CauseOfDeath::DrankTooMuchBeer)
            );
            assert!(state.player().has_internet());
            assert!(state.player().is_employed_at_terkom());
        }
    );
}

#[test]
fn grisha_refuse_terkom() {
    initialize_game!((1, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Идём в Мавзолей, ждём когда там появится Гриша
    replay_game(game_ui, "6↓r2↑2r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Grisha)));

    // Подходим к Грише. Он поит пивом.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::CantBeExpelledInFourthYear {
                drink_beer: true,
                hour_pass: false,
            }
        ) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Он предлагает устроиться в ТЕРКОМ.
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(_, GrishaInteraction::PromptEmploymentAtTerkom)
    );

    // Отказываемся
    replay_game(game_ui, "↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::AsYouWantButDontOverstudy
        ) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );

    // Снова подходим к Грише. Проходит час
    replay_game(game_ui, "2↑r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GrishaInteraction(
            state,
            GrishaInteraction::NamesOfFreebieLovers {
                drink_beer: false,
                hour_pass: true,
            }
        ) => {
            assert_eq!(state.current_time(), Time(9));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 46,
                money: 0,
                brain: 4,
                stamina: 3,
                charisma: 5,
            );
            assert!(!state.player().has_internet());
            assert!(!state.player().is_employed_at_terkom());
        }
    );
}
