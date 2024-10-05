use mmheroes_core::logic::Game;
use mmheroes_core::ui::recording::{InputRecordingParser, InputRecordingParserError};
use mmheroes_core::ui::renderer::RendererRequestConsumer;
use mmheroes_core::ui::*;

#[allow(dead_code)]
enum OwningRendererRequest {
    ClearScreen,
    Flush,
    WriteString(String),
    MoveCursor {
        line: u8,
        column: u8,
    },
    SetColor {
        foreground: Color,
        background: Color,
    },
    Sleep(Milliseconds),
}

pub struct TestRendererRequestConsumer {
    requests: Vec<OwningRendererRequest>,
}

impl TestRendererRequestConsumer {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn requests(&self) -> &[OwningRendererRequest] {
        &*self.requests
    }
}

impl RendererRequestConsumer for TestRendererRequestConsumer {
    fn consume_request(&mut self, request: RendererRequest) {
        let owning_request = match request {
            RendererRequest::ClearScreen => OwningRendererRequest::ClearScreen,
            RendererRequest::Flush => OwningRendererRequest::Flush,
            RendererRequest::WriteStr(s) => {
                OwningRendererRequest::WriteString(String::from(s))
            }
            RendererRequest::MoveCursor { line, column } => {
                OwningRendererRequest::MoveCursor { line, column }
            }
            RendererRequest::SetColor {
                foreground,
                background,
            } => OwningRendererRequest::SetColor {
                foreground,
                background,
            },
            RendererRequest::Sleep(ms) => OwningRendererRequest::Sleep(ms),
        };
        self.requests.push(owning_request)
    }
}

pub type TestGameUI<'game, G> = GameUI<'game, G, TestRendererRequestConsumer>;

pub fn replay_game<G: Game>(game_ui: &mut TestGameUI<G>, steps: &str) {
    let mut parser = InputRecordingParser::new(steps);
    match parser.parse_all(|input| game_ui.continue_game(input)) {
        Ok(()) => {}
        Err(InputRecordingParserError::Interrupted) => {}
        Err(error) => panic!("{:?}", error),
    }
}

#[macro_export]
macro_rules! initialize_game {
    (($seed:expr, $mode:expr) => $state:ident, $game_ui:ident) => {
        let $state = core::cell::RefCell::new(
            mmheroes_core::logic::ObservableGameState::new($mode),
        );
        let mut game = mmheroes_core::logic::create_game($seed, &$state);
        let game = pin!(game);
        let mut $game_ui = $crate::TestGameUI::new(
            &$state,
            game,
            None,
            TestRendererRequestConsumer::new(),
        );
    };
}
