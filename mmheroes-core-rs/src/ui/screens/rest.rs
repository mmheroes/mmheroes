use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

fn display_rest(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
    prompt: &'static str,
) -> WaitingState {
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    writeln_colored!(White, r, "{}", prompt);
    scene_router::display_short_today_timetable(r, 10, state);
    r.move_cursor_to(10, 0);
    dialog(r, available_actions)
}

pub(in crate::ui) fn display_rest_in_mausoleum(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
) -> WaitingState {
    display_rest(
        r,
        available_actions,
        state,
        "Выбери себе способ \"культурного отдыха\".",
    )
}

pub(in crate::ui) fn display_cafe(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
) -> WaitingState {
    display_rest(r, available_actions, state, "Что брать будем?")
}
