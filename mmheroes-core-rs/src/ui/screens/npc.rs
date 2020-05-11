use crate::logic::{npc::KolyaInteraction, *};
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_kolya_interaction(
    r: &mut Renderer,
    state: &GameState,
    interaction: npc::KolyaInteraction,
) -> WaitingState {
    scene_router::display_header_stats(r, state);

    let today_timetable = |r: &mut Renderer| {
        scene_router::display_short_today_timetable(
            r,
            state.current_day(),
            state.player(),
        )
    };

    let oat_tincture_is_better = |r: &mut Renderer| {
        writeln_colored!(
            WhiteBright,
            r,
            "\"Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!\""
        );
    };

    let brake_fluid = |r: &mut Renderer| {
        writeln_colored!(
            MagentaBright,
            r,
            "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        );
    };

    let solved_algebra_problems = |r: &mut Renderer| {
        writeln_colored!(
            WhiteBright,
            r,
            "\"У тебя остались нерешенные задачи по Всемирнову? Давай сюда!\""
        );
        write_colored!(White, r, "Коля решил тебе еще ");
        write_colored!(WhiteBright, r, "{}", 2);
        writeln_colored!(White, r, " задачи по алгебре!");
        wait_for_any_key(r)
    };

    r.move_cursor_to(7, 0);
    writeln_colored!(White, r, "Коля смотрит на тебя немного окосевшими глазами.");

    match interaction {
        KolyaInteraction::SolvedAlgebraProblemsForFree => solved_algebra_problems(r),
        KolyaInteraction::PromptOatTincture => {
            today_timetable(r);
            oat_tincture_is_better(r);
            writeln_colored!(White, r, "Заказать Коле настойку овса?");
            let options = tiny_vec!(capacity: 16, [
                ("Да", Color::CyanBright, Action::Yes),
                ("Нет", Color::CyanBright, Action::No),
            ]);
            r.move_cursor_to(14, 0);
            dialog(r, options)
        }
        KolyaInteraction::SolvedAlgebraProblemsForOatTincture => {
            today_timetable(r);
            r.move_cursor_to(18, 0);
            solved_algebra_problems(r)
        }
        KolyaInteraction::BrakeFluidNoMoney => {
            oat_tincture_is_better(r);
            r.move_cursor_to(14, 0);
            brake_fluid(r);
            wait_for_any_key(r)
        }
        KolyaInteraction::BrakeFluidBecauseRefused => {
            today_timetable(r);
            r.move_cursor_to(18, 0);
            writeln_colored!(WhiteBright, r, "\"Зря, ой, зря ...\"");
            brake_fluid(r);
            wait_for_any_key(r)
        }
        KolyaInteraction::Altruism => {
            today_timetable(r);
            r.move_cursor_to(18, 0);
            writeln_colored!(
                White,
                r,
                "Твой альтруизм навсегда останется в памяти потомков."
            );
            wait_for_any_key(r)
        }
    }
}