use assert_matches::assert_matches;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Game, GameMode, GameScreen, StateHolder};
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
    state: &StateHolder,
    game_ui: &mut TestGameUI<G>,
    style: PlayStyle,
) {
    replay_game(game_ui, "r");
    let mode = state.observable_state().mode();
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
    assert_matches!(
        state.observable_state().screen(),
        GameScreen::SceneRouter(_)
    );
}

#[macro_export]
macro_rules! initialize_game {
    (($seed:expr, $mode:expr, $high_scores:expr) => $state:ident, $game_ui:ident) => {
        let state_holder = mmheroes_core::logic::StateHolder::new($mode);
        let $state = &state_holder;
        let mut game = mmheroes_core::logic::create_game($seed, $state);
        let game = core::pin::pin!(game);
        let mut game_ui = $crate::TestGameUI::new(
            $state,
            game,
            $seed,
            $high_scores,
            $crate::TestRendererRequestConsumer::new(),
        );
        let $game_ui = &mut game_ui;
        $game_ui.continue_game(mmheroes_core::ui::Input::Enter)
    };
    (($seed:expr, $mode:expr) => $state:ident, $game_ui:ident) => {
        initialize_game!(($seed, $mode, None) => $state, $game_ui);
    };
}

#[macro_export]
macro_rules! assert_characteristics {
    (
        $state:expr,
        health: $health:expr,
        money: $money:expr,
        brain: $brain:expr,
        stamina: $stamina:expr,
        charisma: $charisma:expr $(,)?
    ) => {{
        #[derive(Debug, Eq, PartialEq)]
        struct Characterisctis {
            health: i16,
            money: i16,
            brain: i16,
            stamina: i16,
            charisma: i16,
        }
        assert_eq!(
            Characterisctis {
                health: $state.player().health().0,
                money: $state.player().money().0,
                brain: $state.player().brain().0,
                stamina: $state.player().stamina().0,
                charisma: $state.player().charisma().0,
            },
            Characterisctis {
                health: $health,
                money: $money,
                brain: $brain,
                stamina: $stamina,
                charisma: $charisma,
            },
        );
    }};
}

#[macro_export]
macro_rules! assert_subject_knowledge {
    (
        $state:expr,
        algebra: $algebra_knowledge:expr,
        calculus: $calculus_knowledge:expr,
        geometry: $geometry_knowledge:expr,
        cs: $cs_knowledge:expr,
        english: $english_knowledge:expr,
        pe: $pe_knowledge:expr $(,)?
    ) => {{
        #[derive(Debug, Eq, PartialEq)]
        struct SubjectKnowledge {
            algebra: i16,
            calculus: i16,
            geometry: i16,
            cs: i16,
            english: i16,
            pe: i16,
        }
        assert_eq!(
            SubjectKnowledge {
                algebra: $state
                    .player()
                    .status_for_subject(
                        mmheroes_core::logic::Subject::AlgebraAndNumberTheory
                    )
                    .knowledge()
                    .0,
                calculus: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::Calculus)
                    .knowledge()
                    .0,
                geometry: $state
                    .player()
                    .status_for_subject(
                        mmheroes_core::logic::Subject::GeometryAndTopology
                    )
                    .knowledge()
                    .0,
                cs: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::ComputerScience)
                    .knowledge()
                    .0,
                english: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::English)
                    .knowledge()
                    .0,
                pe: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::PhysicalEducation)
                    .knowledge()
                    .0,
            },
            SubjectKnowledge {
                algebra: $algebra_knowledge,
                calculus: $calculus_knowledge,
                geometry: $geometry_knowledge,
                cs: $cs_knowledge,
                english: $english_knowledge,
                pe: $pe_knowledge,
            },
        );
    }};
}
