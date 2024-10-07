use assert_matches::assert_matches;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Game, GameMode, GameScreen, ObservableGameState};
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

/// Возвращает `true` как только [GameUI::continue_game] возвращает `false`,
/// то есть, когда игра закончилась. Пока игра не закончилась, возвращает `false`.
pub fn replay_game<G: Game>(game_ui: &mut TestGameUI<G>, steps: &str) -> bool {
    let mut parser = InputRecordingParser::new(steps);
    match parser.parse_all(|input| game_ui.continue_game(input)) {
        Ok(()) => false,
        Err(InputRecordingParserError::Interrupted) => true,
        Err(error) => panic!("{:?}", error),
    }
}

pub fn replay_until_dorm<G: Game>(
    state: &core::cell::RefCell<ObservableGameState>,
    game_ui: &mut TestGameUI<G>,
    style: PlayStyle,
) {
    replay_game(game_ui, "r");
    let mode = state.borrow().mode();
    if mode != GameMode::Normal {
        match style {
            PlayStyle::RandomStudent => {}
            PlayStyle::CleverStudent => {
                replay_game(game_ui, "↓");
            }
            PlayStyle::ImpudentStudent => {
                replay_game(game_ui, "2↓");
            }
            PlayStyle::SociableStudent => {
                replay_game(game_ui, "3↓");
            }
            PlayStyle::GodMode => {
                if mode == GameMode::God {
                    replay_game(game_ui, "4↓");
                }
            }
        };
        replay_game(game_ui, "r");
    }
    // Дзинь!
    replay_game(game_ui, "2r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(_));
}

#[macro_export]
macro_rules! initialize_game {
    (($seed:expr, $mode:expr, $high_scores:expr) => $state:ident, $game_ui:ident) => {
        let $state = core::cell::RefCell::new(
            mmheroes_core::logic::ObservableGameState::new($mode),
        );
        let mut game = mmheroes_core::logic::create_game($seed, &$state);
        let game = core::pin::pin!(game);
        let mut $game_ui = $crate::TestGameUI::new(
            &$state,
            game,
            $high_scores,
            TestRendererRequestConsumer::new(),
        );
        $game_ui.continue_game(mmheroes_core::ui::Input::Enter)
    };
    (($seed:expr, $mode:expr) => $state:ident, $game_ui:ident) => {
        initialize_game!(($seed, $mode, None) => $state, $game_ui)
    };
}
