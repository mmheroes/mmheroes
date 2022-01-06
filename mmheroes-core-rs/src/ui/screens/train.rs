use crate::logic::scene_router::train::TrainToPDMI;
use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_train_to_pdmi<R: RendererRequestConsumer>(
    r: &mut Renderer<R>,
    available_actions: &[Action],
    state: &GameState,
    interaction: TrainToPDMI,
) -> WaitingState {
    let gatecrash = |r: &mut Renderer<R>, caught_by_inspectors: bool| -> WaitingState {
        if caught_by_inspectors {
            writeln!(r, "Тебя заловили контролеры!");
            write!(r, "Высадили в Красных зорях, гады!");
        } else {
            write!(r, "Уф, доехал!")
        }
        wait_for_any_key(r)
    };
    match interaction {
        TrainToPDMI::NoPointToGoToPDMI => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            if state.location() == Location::Dorm {
                // В оригинальной реализации тоже очищается экран при попытке поехать
                // в ПОМИ из общежития.
                r.clear_screen();
            }
            r.set_color(Color::White, Color::Black);
            writeln!(r, "Здравый смысл подсказывает тебе, что в такое время");
            writeln!(r, "ты там никого уже не найдешь.");
            write!(r, "Не будем зря тратить здоровье на поездку в ПОМИ.");
            wait_for_any_key(r)
        }
        TrainToPDMI::GatecrashBecauseNoMoney {
            caught_by_inspectors,
        } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::White, Color::Black);
            writeln!(r, "Денег у тебя нет, пришлось ехать зайцем...");
            gatecrash(r, caught_by_inspectors)
        }
        TrainToPDMI::PromptToBuyTickets => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(11, 0);
            dialog(r, available_actions)
        }
        TrainToPDMI::GatecrashByChoice {
            caught_by_inspectors,
        } => {
            r.move_cursor_to(14, 0);
            r.set_color(Color::White, Color::Black);
            gatecrash(r, caught_by_inspectors)
        }
        TrainToPDMI::BoughtRoundtripTicket => wait_for_any_key(r),
    }
}
