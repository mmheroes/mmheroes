//! # Устаревшие функции
//!
//! Пока мы в процессе переписывания игры в стиле async/await нам всё ещё нужны эти
//! функции. Когда вся игра будет переписана, их можно будет удалить.
//!

use crate::logic::actions::ActionVec;
use crate::logic::{entry_point, Action, InternalGameState};

#[deprecated]
pub(in crate::logic) fn start_game(g: &mut InternalGameState) -> ActionVec {
    if entry_point::should_select_game_style(g) {
        g.observable_state.borrow().available_actions.clone()
    } else {
        entry_point::ding(g, Action::RandomStudent)
    }
}
