use crate::logic::actions::{wait_for_any_key, ActionVec};
use crate::logic::{Action, GameMode, GameScreen, InternalGameState};

/// Точка входа
pub(super) async fn run(g: &mut InternalGameState<'_>) {
    let game_style = select_game_style(g).await;
    let available_actions = ding(g, game_style);
    g.set_available_actions_from_vec(available_actions);
    loop {
        let action = g.wait_for_action().await;
        let new_actions = g.perform_action(action);
        g.set_available_actions_from_vec(new_actions);
    }
}

pub(super) fn should_select_game_style(g: &mut InternalGameState) -> bool {
    let mode = g.observable_state.borrow().mode;
    match mode {
        GameMode::SelectInitialParameters => {
            g.set_screen(GameScreen::InitialParameters);
            // Можно выбрать 4 стиля игры:
            // - Случайный студент
            // - Шибко умный
            // - Шибко наглый
            // - Шибко общительный
            g.set_available_actions([
                Action::RandomStudent,
                Action::CleverStudent,
                Action::ImpudentStudent,
                Action::SociableStudent,
            ]);
            true
        }
        GameMode::God => {
            g.set_screen(GameScreen::InitialParameters);
            // Можно выбрать 5 стилей игры:
            // - Случайный студент
            // - Шибко умный
            // - Шибко наглый
            // - Шибко общительный
            // - GOD-режим
            g.set_available_actions([
                Action::RandomStudent,
                Action::CleverStudent,
                Action::ImpudentStudent,
                Action::SociableStudent,
                Action::GodMode,
            ]);
            true
        }
        GameMode::Normal => false,
    }
}

async fn select_game_style(g: &mut InternalGameState<'_>) -> Action {
    if should_select_game_style(g) {
        g.wait_for_action().await
    } else {
        Action::RandomStudent
    }
}

pub(super) fn ding(g: &mut InternalGameState, action: Action) -> ActionVec {
    let player = g.initialize_player(action);
    g.set_screen(GameScreen::Ding(player));
    wait_for_any_key()
}
