use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_cafe(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
) -> WaitingState {
    r.clear_screen();
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);

    let (line, prompt) = match state.location() {
        Location::PUNK => (10, "Что брать будем?"),
        Location::PDMI => (9, "Что брать будем?"),
        Location::Mausoleum => (10, "Выбери себе способ \"культурного отдыха\"."),
        _ => unreachable!(),
    };
    writeln_colored!(White, r, "{}", prompt);
    scene_router::display_short_today_timetable(r, line, state);
    r.move_cursor_to(line, 0);
    dialog(r, available_actions)
}
