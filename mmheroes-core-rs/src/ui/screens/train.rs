use crate::logic::scene_router::train::{BaltiyskiyRailwayStationScene, TrainScene};
use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_train_to_pdmi(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
    interaction: TrainScene,
) -> WaitingState {
    match interaction {
        TrainScene::NoPointToGoToPDMI
        | TrainScene::GatecrashBecauseNoMoney { .. }
        | TrainScene::PromptToBuyTickets => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
        }
        TrainScene::GatecrashByChoice { .. } | TrainScene::BoughtRoundtripTicket => (),
    }
    display_train(
        r,
        available_actions,
        state,
        interaction,
        7,
        11,
        Color::White,
        &|r, caught_by_inspectors| {
            r.set_color(Color::White, Color::Black);
            if caught_by_inspectors {
                writeln!(r, "Тебя заловили контролеры!");
                write!(r, "Высадили в Красных зорях, гады!");
            } else {
                writeln!(r, "Уф, доехал!");
            }
        },
    )
}

pub(in crate::ui) fn display_train_algebra_exam(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    state: &GameState,
    interaction: TrainScene,
) -> WaitingState {
    match interaction {
        TrainScene::GatecrashBecauseNoMoney { .. } | TrainScene::PromptToBuyTickets => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(11, 0);
            writeln_colored!(
                White,
                r,
                "Есть надежда, что в электричке удастся что-то еще решить."
            );
            writeln!(r, "Правда, зачетной ведомости с собой туда не взять...");
        }
        TrainScene::GatecrashByChoice { .. } | TrainScene::BoughtRoundtripTicket => (),
        TrainScene::NoPointToGoToPDMI => unreachable!(),
    }
    display_train(
        r,
        available_actions,
        state,
        interaction,
        13,
        16,
        Color::RedBright,
        &|r, caught_by_inspectors| {
            if caught_by_inspectors {
                writeln_colored!(MagentaBright, r, "Тебя заловили контролеры!");
                match interaction {
                    TrainScene::GatecrashBecauseNoMoney { .. } => {
                        writeln!(r, "Высадили в Красных зорях, гады!");
                    }
                    TrainScene::GatecrashByChoice { .. } => {
                        writeln!(r, "И со Всемирновым ты ничего не успел...");
                    }
                    _ => (),
                }
            }
        },
    )
}

fn display_train<R: RendererRequestConsumer>(
    r: &mut Renderer<R>,
    available_actions: &[Action],
    state: &GameState,
    interaction: TrainScene,
    start_line: Line,
    prompt_line: Line,
    no_money_color: Color,
    gatecrash: &dyn Fn(&mut Renderer<R>, bool) -> (),
) -> WaitingState {
    match interaction {
        TrainScene::NoPointToGoToPDMI => {
            r.move_cursor_to(start_line, 0);
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
        TrainScene::GatecrashBecauseNoMoney {
            caught_by_inspectors,
        } => {
            r.move_cursor_to(start_line, 0);
            r.set_color(no_money_color, Color::Black);
            writeln!(r, "Денег у тебя нет, пришлось ехать зайцем...");
            gatecrash(r, caught_by_inspectors);
            wait_for_any_key(r)
        }
        TrainScene::PromptToBuyTickets => {
            r.move_cursor_to(prompt_line, 0);
            dialog(r, available_actions)
        }
        TrainScene::GatecrashByChoice {
            caught_by_inspectors,
        } => {
            r.move_cursor_to(start_line + 7, 0);
            gatecrash(r, caught_by_inspectors);
            wait_for_any_key(r)
        }
        TrainScene::BoughtRoundtripTicket => wait_for_any_key(r),
    }
}

pub(in crate::ui) fn display_baltiyskiy_railway_station(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    scene: &BaltiyskiyRailwayStationScene,
) -> WaitingState {
    match scene {
        BaltiyskiyRailwayStationScene::Prompt(state) => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(11, 0);
            writeln_colored!(CyanBright, r, "Ты в Питере, на Балтийском вокзале.");
            writeln!(r, "Куда направляемся?");
            r.move_cursor_to(14, 0);
            dialog(r, available_actions)
        }
        BaltiyskiyRailwayStationScene::CaughtByInspectors => {
            r.move_cursor_to(19, 0);
            writeln_colored!(White, r, "Тебя заловили контролеры!");
            writeln!(r, "Высадили в Красных зорях, гады!");
            wait_for_any_key(r)
        }
    }
}
