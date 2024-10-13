mod common;

use assert_matches::assert_matches;
use common::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::kolya::KolyaInteraction;
use mmheroes_core::logic::Subject::AlgebraAndNumberTheory;
use mmheroes_core::logic::{Action, CauseOfDeath, Classmate, GameMode, GameScreen, Time};

#[test]
fn no_money_brake_fluid() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 10:00 и идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r6↓r");
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Kolya)));
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
        }
    );

    for i in 0..4 {
        // Идём к Коле. Он заставляет нас выпить тормозную жидкость, мы тупеем.
        replay_game(game_ui, "4↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(state, KolyaInteraction::BrakeFluidNoMoney) => {
                assert_eq!(state.current_time(), Time(10));
                assert_characteristics!(
                    state,
                    health: 64,
                    money: 0,
                    brain: 5 - i,
                    stamina: 4,
                    charisma: 5,
                );
            }
        );

        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10));
                assert_characteristics!(
                    state,
                    health: 64,
                    money: 0,
                    brain: 5 - i - 1,
                    stamina: 4,
                    charisma: 5,
                );
            }
        );
    }

    // Снова идём к Коле, снова пьём тормозную жидкость.
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KolyaInteraction(state, KolyaInteraction::BrakeFluidNoMoney) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 64,
                money: 0,
                brain: 1,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    // Умираем
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::GameEnd(state) => {
            assert_eq!(state.player().cause_of_death(), Some(CauseOfDeath::DrankTooMuch));
            assert_characteristics!(
                state,
                health: 0,
                money: 0,
                brain: 0,
                stamina: 4,
                charisma: 5,
            );
        }
    );
}

#[test]
fn no_money_solves_algebra_problems() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::SociableStudent);

    // Отдыхаем до 10:00 и идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r6↓r");
    assert!(state
        .observable_state()
        .available_actions()
        .contains(&Action::InteractWithClassmate(Classmate::Kolya)));
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                0,
            );
            assert_characteristics!(
                state,
                health: 60,
                money: 0,
                brain: 3,
                stamina: 2,
                charisma: 9,
            );
        }
    );

    for i in 0..6 {
        // Идём к Коле. Он решает нам задачи.
        replay_game(game_ui, "4↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(state, KolyaInteraction::SolvedAlgebraProblemsForFree) => {
                assert_eq!(state.current_time(), Time(10 + i));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    i * 2,
                );
                assert_characteristics!(
                    state,
                    health: 60,
                    money: 0,
                    brain: 3,
                    stamina: 2,
                    charisma: 9,
                );
            }
        );
        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10 + i + 1));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    i * 2 + 2,
                );
                assert_characteristics!(
                    state,
                    health: 60,
                    money: 0,
                    brain: 3,
                    stamina: 2,
                    charisma: 9,
                );
            }
        );
    }

    // Снова идём к Коле. Все задачи решены, поэтому пьём тормозную жидкость.
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KolyaInteraction(state, KolyaInteraction::BrakeFluidNoMoney) => {
            assert_eq!(state.current_time(), Time(16));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                12,
            );
            assert_characteristics!(
                state,
                health: 60,
                money: 0,
                brain: 3,
                stamina: 2,
                charisma: 9,
            );
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(16));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                12,
            );
            assert_characteristics!(
                state,
                health: 60,
                money: 0,
                brain: 2,
                stamina: 2,
                charisma: 9,
            );
        }
    );
}

#[test]
fn altruism() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 10:00 и идём в ПУНК, забираем стипендию и идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r4↓r3↑2r4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                0,
            );
            assert_characteristics!(
                state,
                health: 61,
                money: 50,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );

    for i in 0..3 {
        // Подходим к Коле, он предлагает купить ему настойку овса
        replay_game(game_ui, "4↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(
                state,
                KolyaInteraction::PromptOatTincture,
            ) => {
                assert_eq!(state.current_time(), Time(10));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    0,
                );
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50 - 15 * i,
                    brain: 5,
                    stamina: 4,
                    charisma: 5,
                );
            }
        );

        // Соглашаемся, но задачи по алгебре он не решает.
        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(
                state,
                KolyaInteraction::Altruism,
            ) => {
                assert_eq!(state.current_time(), Time(10));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    0,
                );
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50 - 15 * i,
                    brain: 5,
                    stamina: 4,
                    charisma: 5,
                );
            }
        );

        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    0,
                );
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50 - 15 * (i + 1),
                    brain: 5,
                    stamina: 4,
                    charisma: 5,
                );
            }
        );
    }

    // Денег у нас больше нет, поэтому пьём тормозную жидкость
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KolyaInteraction(
            state,
            KolyaInteraction::BrakeFluidNoMoney,
        ) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                0,
            );
            assert_characteristics!(
                state,
                health: 61,
                money: 5,
                brain: 5,
                stamina: 4,
                charisma: 5,
            );
        }
    );
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                0,
            );
            assert_characteristics!(
                state,
                health: 61,
                money: 5,
                brain: 4,
                stamina: 4,
                charisma: 5,
            );
        }
    );
}

#[test]
fn refuse_to_buy_oat_tincture() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::ImpudentStudent);

    // Отдыхаем до 10:00 и идём в ПУНК, забираем стипендию и идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r4↓r3↑2r4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_characteristics!(
                state,
                health: 61,
                money: 50,
                brain: 3,
                stamina: 5,
                charisma: 3,
            );
        }
    );

    // Идём к Коле и несколько раз отказываемся покупать ему настойку овса.
    // Мозг уходит в минус, но смерть не наступает.
    for i in 0..6 {
        replay_game(game_ui, "4↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(
                state,
                KolyaInteraction::PromptOatTincture,
            ) => {
                assert_eq!(state.current_time(), Time(10));
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50,
                    brain: 3 - i,
                    stamina: 5,
                    charisma: 3,
                );
            }
        );
        replay_game(game_ui, "↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(
                state,
                KolyaInteraction::BrakeFluidBecauseRefused,
            ) => {
                assert_eq!(state.current_time(), Time(10));
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50,
                    brain: 3 - i,
                    stamina: 5,
                    charisma: 3,
                );
            }
        );
        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10));
                assert_characteristics!(
                    state,
                    health: 61,
                    money: 50,
                    brain: 3 - i - 1,
                    stamina: 5,
                    charisma: 3,
                );
            }
        );
    }
}

#[test]
fn solves_algebra_problems_for_oat_tincture() {
    initialize_game!((5, GameMode::Normal) => state, game_ui);
    replay_until_dorm(state, game_ui, PlayStyle::RandomStudent);

    // Отдыхаем до 10:00 и идём в ПУНК, забираем стипендию и идём в Мавзолей
    replay_game(game_ui, "2↓r2↓r4↓r2↑2r4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(10));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                0,
            );
            assert_characteristics!(
                state,
                health: 59,
                money: 50,
                brain: 6,
                stamina: 5,
                charisma: 6,
            );
        }
    );

    // Подходим к Коле, он решает нам четыре задачи по алгебре бесплатно
    for i in 0..2 {
        replay_game(game_ui, "4↓r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::KolyaInteraction(
                state,
                KolyaInteraction::SolvedAlgebraProblemsForFree,
            ) => {
                assert_eq!(state.current_time(), Time(10 + i));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    i * 2,
                );
                assert_characteristics!(
                    state,
                    health: 59,
                    money: 50,
                    brain: 6,
                    stamina: 5,
                    charisma: 6,
                );
            }
        );

        replay_game(game_ui, "r");
        assert_matches!(
            state.observable_state().screen(),
            GameScreen::SceneRouter(state) => {
                assert_eq!(state.current_time(), Time(10 + i + 1));
                assert_eq!(
                    state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                    i * 2 + 2,
                );
                assert_characteristics!(
                    state,
                    health: 59,
                    money: 50,
                    brain: 6,
                    stamina: 5,
                    charisma: 6,
                );
            }
        );
    }

    // Снова подходим к Коле, он предлагает купить ему настойку овса
    replay_game(game_ui, "4↓r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KolyaInteraction(
            state,
            KolyaInteraction::PromptOatTincture,
        ) => {
            assert_eq!(state.current_time(), Time(12));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                4,
            );
            assert_characteristics!(
                state,
                health: 59,
                money: 50,
                brain: 6,
                stamina: 5,
                charisma: 6,
            );
        }
    );

    // Соглашаемся, он решает нам ещё две задачи
    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::KolyaInteraction(
            state,
            KolyaInteraction::SolvedAlgebraProblemsForOatTincture,
        ) => {
            assert_eq!(state.current_time(), Time(12));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                4,
            );
            assert_characteristics!(
                state,
                health: 59,
                money: 50,
                brain: 6,
                stamina: 5,
                charisma: 6,
            );
        }
    );

    replay_game(game_ui, "r");
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(state) => {
            assert_eq!(state.current_time(), Time(13));
            assert_eq!(
                state.player().status_for_subject(AlgebraAndNumberTheory).problems_done(),
                6,
            );
            assert_characteristics!(
                state,
                health: 59,
                money: 35,
                brain: 6,
                stamina: 5,
                charisma: 6,
            );
        }
    );
}
