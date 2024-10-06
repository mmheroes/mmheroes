use super::*;
use crate::logic::{Action, GameMode, GameScreen, GameState, InternalGameState};

pub(in crate::logic) enum GameEnd {
    Exit,
    Restart,
}

/// Точка входа
pub(super) async fn run(g: &mut InternalGameState<'_>) {
    loop {
        let game_style = select_game_style(g).await;
        let player = g.initialize_player(game_style);
        g.set_screen(GameScreen::Ding(player.clone()));
        g.wait_for_any_key().await;
        let state = GameState::new(
            player.clone(),
            Timetable::random(&mut g.rng),
            Location::Dorm,
        );
        timetable::show(g, state.clone()).await;
        if let GameEnd::Exit = scene_router::run(g, state.clone()).await {
            return;
        }
    }
}

pub(super) fn should_select_game_style(g: &mut InternalGameState) -> bool {
    let mode = g.observable_state.borrow().mode;
    match mode {
        GameMode::SelectInitialParameters => {
            // Можно выбрать 4 стиля игры:
            // - Случайный студент
            // - Шибко умный
            // - Шибко наглый
            // - Шибко общительный
            g.set_screen_and_available_actions(
                GameScreen::InitialParameters,
                [
                    Action::RandomStudent,
                    Action::CleverStudent,
                    Action::ImpudentStudent,
                    Action::SociableStudent,
                ],
            );
            true
        }
        GameMode::God => {
            // Можно выбрать 5 стилей игры:
            // - Случайный студент
            // - Шибко умный
            // - Шибко наглый
            // - Шибко общительный
            // - GOD-режим
            g.set_screen_and_available_actions(
                GameScreen::InitialParameters,
                [
                    Action::RandomStudent,
                    Action::CleverStudent,
                    Action::ImpudentStudent,
                    Action::SociableStudent,
                    Action::GodMode,
                ],
            );
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
