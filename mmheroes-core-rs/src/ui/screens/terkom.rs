use crate::logic::scene_router::terkom::Terkom;
use crate::logic::{Action, GameState, Money};
use crate::random;
use crate::ui::dialog::dialog;
use crate::ui::renderer::{Renderer, RendererRequestConsumer};
use crate::ui::screens::scene_router;
use crate::ui::{wait_for_any_key, Color, WaitingState};

pub(in crate::ui) fn display_terkom(
    r: &mut Renderer<impl RendererRequestConsumer>,
    available_actions: &[Action],
    rng: &mut random::Rng,
    state: &GameState,
    screen: Terkom,
) -> WaitingState {
    match screen {
        Terkom::SorryNoFreeComputers { hiccup } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::Green, Color::Black);
            write_with_hiccup(
                r,
                rng,
                hiccup,
                "\"Извини, парень, свободных кумпутеров нет.",
            );
            writeln!(r);
            write_with_hiccup(r, rng, hiccup, "Пойди поучись пока.\"");
            writeln!(r);
            wait_for_any_key(r)
        }
        Terkom::AgainNoFreeComputers { hiccup } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::CyanBright, Color::Black);
            write_with_hiccup(r, rng, hiccup, "\"Сказано же, нет свободных компов!\"");
            writeln!(r);
            wait_for_any_key(r)
        }
        Terkom::Prompt => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            writeln_colored!(White, r, "Ты сидишь за свободным компом");
            writeln!(r, "в тереховской \"конторе\".");
            writeln!(r, "Что делать будем?");
            scene_router::display_short_today_timetable(r, 7, state);
            r.move_cursor_to(11, 0);
            dialog(r, available_actions)
        }
        Terkom::YouEarnedByWorking { income, hiccup } => {
            r.move_cursor_to(18, 0);
            write_income(r, rng, hiccup, income);
            wait_for_any_key(r)
        }
        Terkom::YouEarnedBySurfingInternet { income, hiccup } => {
            r.move_cursor_to(18, 0);
            writeln_colored!(White, r, "Вот здорово - мы сидим, а денежки-то идут!");
            write_income(r, rng, hiccup, income);
            wait_for_any_key(r)
        }
        Terkom::MMHEROES { hiccup } => {
            todo!("Поиграть в MMHEROES")
        }
        Terkom::Leaving { hiccup } => {
            r.move_cursor_to(16, 0);
            r.set_color(Color::White, Color::Black);
            write_with_hiccup(r, rng, hiccup, "Уходим ...");
            writeln!(r);
            wait_for_any_key(r)
        }
        Terkom::EndOfWorkDay { hiccup } => {
            r.move_cursor_to(19, 0);
            r.set_color(Color::White, Color::Black);
            write_with_hiccup(r, rng, hiccup, "Рабочий день закончился, все по домам.");
            writeln!(r);
            wait_for_any_key(r)
        }
    }
}

fn write_with_hiccup(
    r: &mut Renderer<impl RendererRequestConsumer>,
    rng: &mut random::Rng,
    hiccup: u8,
    s: &str,
) {
    let mut hiccup = hiccup;
    let mut is_first = true;
    for word in s.split(' ') {
        if is_first {
            is_first = false;
        } else if rng.roll_dice(hiccup) {
            write!(r, " <йк> ");
            hiccup -= 1;
            if hiccup < 1 {
                hiccup = 2;
            }
        } else {
            write!(r, " ");
        }
        write!(r, "{}", word);
    }
}

fn write_income(
    r: &mut Renderer<impl RendererRequestConsumer>,
    rng: &mut random::Rng,
    hiccup: u8,
    income: Money,
) {
    r.set_color(Color::White, Color::Black);
    write_with_hiccup(r, rng, hiccup, "Тебе накапало ");
    write_colored!(WhiteBright, r, "{}", income);
    writeln_colored!(White, r, " руб.");
}
