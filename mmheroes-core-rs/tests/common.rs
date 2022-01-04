use mmheroes_core::logic::Game;
use mmheroes_core::ui::*;
use mmheroes_core::ui::recording::{InputRecordingParser, InputRecordingParserError};
use mmheroes_core::ui::renderer::RendererRequestConsumer;

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
        Self { requests: Vec::new() }
    }

    fn requests(&self) -> &[OwningRendererRequest] {
        &*self.requests
    }
}

impl RendererRequestConsumer for TestRendererRequestConsumer {
    fn consume_request(&mut self, request: RendererRequest) {
        let owning_request = match request {
            RendererRequest::ClearScreen => OwningRendererRequest::ClearScreen,
            RendererRequest::Flush => OwningRendererRequest::Flush,
            RendererRequest::WriteStr(s) => OwningRendererRequest::WriteString(String::from(s)),
            RendererRequest::MoveCursor { line, column } => OwningRendererRequest::MoveCursor { line, column },
            RendererRequest::SetColor { foreground, background } => OwningRendererRequest::SetColor { foreground, background },
            RendererRequest::Sleep(ms) => OwningRendererRequest::Sleep(ms),
        };
        self.requests.push(owning_request)
    }
}

pub type TestGameUI<'game> = GameUI<'game, TestRendererRequestConsumer>;

pub fn replay_game(game_ui: &mut TestGameUI, steps: &str) {
    let mut parser = InputRecordingParser::new(steps);
    match parser.parse_all(|input| game_ui.continue_game(input)) {
        Ok(()) => {}
        Err(InputRecordingParserError::Interrupted) => {}
        Err(error) => panic!("{:?}", error)
    }
}