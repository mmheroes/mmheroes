use mmheroes_core::{
    logic::{Game, GameMode},
    ui::{self, *},
};
use pancurses::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

mod screen {
    use super::{endwin, initscr, Window};

    /// A RAII object responsible for initializing and cleaning up the curses
    /// window.
    pub(crate) struct ScreenRAII {
        window: Window,
    }

    impl ScreenRAII {
        pub(crate) fn new() -> ScreenRAII {
            ScreenRAII { window: initscr() }
        }
    }

    impl Drop for ScreenRAII {
        fn drop(&mut self) {
            endwin();
        }
    }

    impl std::ops::Deref for ScreenRAII {
        type Target = Window;

        fn deref(&self) -> &Self::Target {
            &self.window
        }
    }
}

use screen::ScreenRAII;

struct PancursesRenderer<'a> {
    log: &'static Mutex<RefCell<Vec<ui::Input>>>,
    color_pairs: &'a HashMap<(Color, Color), i16>,
    window: &'a ScreenRAII,
}

#[derive(Debug)]
struct CursesError(&'static str);

macro_rules! handle_curses_error {
    ($call:expr) => {{
        let rc = $call;
        if rc == pancurses::OK {
            Ok(())
        } else {
            Err(CursesError(stringify!($call)))
        }
    }};
}

impl Renderer for PancursesRenderer<'_> {
    type Error = CursesError;

    fn clear_screen(&mut self) -> Result<(), Self::Error> {
        handle_curses_error!(self.window.clear())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        handle_curses_error!(self.window.refresh())
    }

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        handle_curses_error!(self.window.addnstr(s, s.len()))
    }

    fn move_cursor_to(&mut self, line: i32, column: i32) -> Result<(), Self::Error> {
        handle_curses_error!(self.window.mv(line, column))
    }

    fn get_cursor_position(&mut self) -> Result<(i32, i32), Self::Error> {
        Ok(self.window.get_cur_yx())
    }

    fn set_color(&mut self, foreground: Color, background: Color) -> Result<(), Self::Error> {
        handle_curses_error!(self.window.color_set(
            *self
                .color_pairs
                .get(&(foreground, background))
                .unwrap_or_else(|| panic!(
                    "Unknown color pair: ({:?}, {:?})",
                    foreground, background
                )),
        ))
    }

    fn getch(&mut self) -> Result<ui::Input, Self::Error> {
        loop {
            let ui_input = match self.window.getch() {
                None | Some(pancurses::Input::KeyResize) => continue,
                Some(pancurses::Input::KeyUp) => ui::Input::KeyUp,
                Some(pancurses::Input::KeyDown) => ui::Input::KeyDown,
                Some(pancurses::Input::Character('\n')) => ui::Input::Enter,
                Some(_) => ui::Input::Other,
            };
            {
                let log = self.log.lock().unwrap();
                log.borrow_mut().push(ui_input);
            }
            break Ok(ui_input);
        }
    }

    fn sleep_ms(&mut self, ms: Milliseconds) -> Result<(), Self::Error> {
        handle_curses_error!(napms(ms.0))
    }
}

fn main() {
    use std::io::Write;

    let window = ScreenRAII::new();
    start_color();
    set_blink(true);
    curs_set(1);

    cbreak();
    noecho();

    window.keypad(true);
    window.nodelay(false);

    // Resize the terminal. We want 24 lines and 80 columns.
    print!("\x1B[8;24;80t");
    std::io::stdout().flush().unwrap();
    resize_term(24, 80);

    window.clear();
    window.refresh();

    let color_pairs = [
        (Color::White, Color::Black),
        (Color::Gray, Color::Black),
        (Color::Red, Color::Black),
        (Color::Green, Color::Black),
        (Color::YellowBright, Color::Black),
        (Color::Cyan, Color::Black),
        (Color::CyanBright, Color::Black),
        (Color::WhiteBright, Color::Black),
        (Color::Black, Color::White),
        (Color::Black, Color::Yellow),
        (Color::Black, Color::Gray),
        (Color::Magenta, Color::Black),
        (Color::MagentaBright, Color::Black),
        (Color::BlueBright, Color::Black),
        (Color::Blue, Color::Black),
    ];

    let mut color_pairs_map = std::collections::HashMap::<(Color, Color), i16>::new();

    for (i, &(foreground, background)) in color_pairs.iter().enumerate() {
        init_pair(i as i16, foreground as i16, background as i16);
        color_pairs_map.insert((foreground, background), i as i16);
    }

    window.bkgd(COLOR_PAIR(
        *color_pairs_map.get(&(Color::White, Color::Black)).unwrap() as chtype,
    ));

    // We save each pressed key to this log, so that if a panic occurs,
    // we could print it and the player could send a useful bug report.
    let log = {
        let log = Box::new(Mutex::new(RefCell::new(Vec::new())));

        // Leak the log object so that we could obtain a reference with static lifetime.
        // This is needed for accessing it in the panic handler.
        &*Box::leak(log)
    };

    let mut renderer = PancursesRenderer {
        log: log,
        color_pairs: &color_pairs_map,
        window: &window,
    };

    let mode = match std::env::args().nth(1).as_deref() {
        Some("-3dec-happy-birthday-Diamond") => GameMode::God,
        Some(_) => GameMode::SelectInitialParameters,
        None => GameMode::Normal,
    };

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut game_ui = GameUI::new(&mut renderer, Game::new(mode, seed));

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        endwin(); // Switch back to normal terminal I/O.
        default_hook(panic_info); // Print panic message and optionally a backtrace.
        let log = log.lock().unwrap();
        eprintln!("Game seed: {}", seed);
        eprintln!("Key presses to reproduce: {:?}", log.borrow());
    }));

    game_ui.run().unwrap()
}
