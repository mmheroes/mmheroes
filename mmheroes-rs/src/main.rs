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

    /// A RAII object repsonsible for initializing and cleaning up the curses
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
    log: &'static Mutex<RefCell<Vec<Option<pancurses::Input>>>>,
    color_pairs: &'a HashMap<(Color, Color), i16>,
    window: &'a ScreenRAII,
}

impl Renderer for PancursesRenderer<'_> {
    fn clear_screen(&mut self) {
        self.window.clear();
    }

    fn flush(&mut self) {
        self.window.refresh();
    }

    fn move_cursor_to(&mut self, line: i32, column: i32) {
        self.window.mv(line, column);
    }

    fn get_cursor_position(&mut self) -> (i32, i32) {
        self.window.get_cur_yx()
    }


    fn set_color(&mut self, foreground: Color, background: Color) {
        self.window.color_set(
            *self
                .color_pairs
                .get(&(foreground, background))
                .expect("Unknown color pair"),
        );
    }

    fn write_str<S: AsRef<str>>(&mut self, string: S) {
        let s = string.as_ref();
        self.window.addnstr(s, s.len());
    }

    fn getch(&mut self) -> ui::Input {
        let input = self.window.getch();
        {
            let log = self.log.lock().unwrap();
            log.borrow_mut().push(input);
        }
        match input {
            Some(pancurses::Input::KeyUp) => ui::Input::KeyUp,
            Some(pancurses::Input::KeyDown) => ui::Input::KeyDown,
            Some(pancurses::Input::Character('\n')) => ui::Input::Enter,
            Some(_) => ui::Input::Other,
            None => ui::Input::EOF,
        }
    }

    fn sleep_ms(&mut self, ms: Milliseconds) {
        napms(ms.0);
    }
}

fn main() {
    let window = ScreenRAII::new();
    start_color();
    use_default_colors();
    set_blink(true);
    curs_set(1);

    cbreak();
    noecho();

    window.clear();
    window.refresh();

    window.keypad(true);
    window.nodelay(false);

    resize_term(24, 80);

    let color_pairs = [
        (Color::White, Color::Black),
        (Color::Gray, Color::Black),
        (Color::Red, Color::Black),
        (Color::Green, Color::Black),
        (Color::Yellow, Color::Black),
        (Color::Cyan, Color::Black),
        (Color::WhiteBright, Color::Black),
        (Color::Black, Color::White),
    ];

    let mut color_pairs_map = std::collections::HashMap::<(Color, Color), i16>::new();

    for (i, &(foreground, background)) in color_pairs.iter().enumerate() {
        init_pair(i as i16, foreground as i16, background as i16);
        color_pairs_map.insert((foreground, background), i as i16);
    }

    window.bkgd(COLOR_PAIR(
        *color_pairs_map
            .get(&(Color::WhiteBright, Color::Black))
            .unwrap() as chtype,
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

    game_ui.run()
}
