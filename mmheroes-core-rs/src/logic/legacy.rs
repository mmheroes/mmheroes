//! # Устаревшие функции
//!
//! Пока мы в процессе переписывания игры в стиле async/await нам всё ещё нужны эти
//! функции. Когда вся игра будет переписана, их можно будет удалить.
//!

#![allow(deprecated)]

use crate::logic::actions::{wait_for_any_key, ActionVec};
use crate::logic::*;

#[deprecated]
pub(in crate::logic) fn start_game(g: &mut InternalGameState) -> ActionVec {
    if entry_point::should_select_game_style(g) {
        g.observable_state.borrow().available_actions.clone()
    } else {
        ding(g, Action::RandomStudent)
    }
}

#[deprecated]
pub(in crate::logic) fn ding(g: &mut InternalGameState, action: Action) -> ActionVec {
    let player = g.initialize_player(action);
    g.set_screen(GameScreen::Ding(player));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn view_timetable(
    g: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    g.set_screen(GameScreen::Timetable(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn scene_router_run(
    game: &mut InternalGameState,
    state: &GameState,
) -> ActionVec {
    let available_actions = scene_router::available_actions(state);
    game.set_screen(GameScreen::SceneRouter(state.clone()));
    available_actions
}

#[deprecated]
pub(in crate::logic) fn handle_action_sync(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    use scene_router::*;
    use Location::*;
    match state.location() {
        PUNK => punk::handle_action(game, state, action),
        PDMI => pdmi::handle_action(game, state, action),
        ComputerClass => computer_class::handle_action(game, state, action),
        Dorm => dorm::handle_action(game, state, action),
        Mausoleum => mausoleum::handle_action(game, state, action),
    }
}

#[deprecated]
pub(in crate::logic) fn game_end(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    game.set_screen(GameScreen::GameEnd(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn wanna_try_again(game: &mut InternalGameState) -> ActionVec {
    game.set_screen(GameScreen::WannaTryAgain);
    // Хочешь попробовать снова? Да или нет.
    ActionVec::from([Action::WantToTryAgain, Action::DontWantToTryAgain])
}

#[deprecated]
pub(in crate::logic) fn handle_wanna_try_again(
    game: &mut InternalGameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::WantToTryAgain => start_game(game),
        Action::DontWantToTryAgain => {
            game.set_screen(GameScreen::Disclaimer);
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}
