use super::*;
use crate::logic::{Action, GameMode, GameScreen, GameState, InternalGameState};

pub(in crate::logic) enum GameEnd {
    Exit,
    Restart,
}

/// Точка входа
pub(super) async fn run(g: &mut InternalGameState<'_>) {
    loop {
        let play_style = select_play_style(g).await;
        let player = g.initialize_player(play_style);
        g.set_screen(GameScreen::Ding(player.clone()));
        g.wait_for_any_key().await;
        let state = GameState::new(
            player.clone(),
            Timetable::random(&mut g.rng),
            Location::Dorm,
        );
        timetable::show(g, &state).await;
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
                    Action::SelectPlayStyle(actions::PlayStyle::RandomStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::CleverStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::ImpudentStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::SociableStudent),
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
                    Action::SelectPlayStyle(actions::PlayStyle::RandomStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::CleverStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::ImpudentStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::SociableStudent),
                    Action::SelectPlayStyle(actions::PlayStyle::GodMode),
                ],
            );
            true
        }
        GameMode::Normal => false,
    }
}

async fn select_play_style(g: &mut InternalGameState<'_>) -> actions::PlayStyle {
    if should_select_game_style(g) {
        match g.wait_for_action().await {
            Action::SelectPlayStyle(style) => style,
            action => illegal_action!(action),
        }
    } else {
        actions::PlayStyle::RandomStudent
    }
}
