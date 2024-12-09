use assert_matches::assert_matches;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::{Game, GameMode, GameScreen, StateHolder};
use mmheroes_core::ui::recording::{InputRecordingParser, InputRecordingParserError};
use mmheroes_core::ui::renderer::RendererRequestConsumer;
use mmheroes_core::ui::*;

type Canvas = [[char; TERMINAL_WIDTH]; TERMINAL_HEIGHT];

pub struct TestRendererRequestConsumer {
    canvas: Canvas,
    line: usize,
    column: usize,
    foreground_color: Color,
    background_color: Color,
}

#[allow(clippy::new_without_default, clippy::inherent_to_string)]
impl TestRendererRequestConsumer {
    fn make_canvas() -> Canvas {
        [[' '; TERMINAL_WIDTH]; TERMINAL_HEIGHT]
    }

    pub fn new() -> Self {
        Self {
            canvas: Self::make_canvas(),
            line: 0,
            column: 0,
            foreground_color: Color::White,
            background_color: Color::Black,
        }
    }

    #[allow(dead_code)] // false positive
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for i in 0..TERMINAL_HEIGHT {
            let mut tmp = String::new();
            for j in 0..TERMINAL_WIDTH {
                if self.line == i && self.column == j {
                    tmp.push('▁');
                } else {
                    tmp.push(self.canvas[i][j]);
                }
            }
            tmp = tmp.trim_end().to_string();
            output.push_str(&tmp);
            output.push('\n');
        }
        output
    }
}

impl RendererRequestConsumer for TestRendererRequestConsumer {
    fn consume_request(&mut self, request: RendererRequest) {
        match request {
            RendererRequest::ClearScreen => {
                self.canvas = Self::make_canvas();
                self.line = 0;
                self.column = 0;
            }
            RendererRequest::Flush => (),
            RendererRequest::WriteStr(s) => {
                for c in s.chars() {
                    if c == '\n' {
                        self.line += 1;
                        self.column = 0;
                    } else {
                        if self.column == TERMINAL_WIDTH {
                            self.line += 1;
                            self.column = 0;
                        }
                        self.canvas[self.line][self.column] = c;
                        self.column += 1;
                    }
                }
            }
            RendererRequest::MoveCursor { line, column } => {
                self.line = line as usize;
                self.column = column as usize;
            }
            RendererRequest::SetColor {
                foreground,
                background,
            } => {
                self.foreground_color = foreground;
                self.background_color = background;
            }
            RendererRequest::Sleep(_) => (),
        };
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
        let mut game_ui = $crate::common::TestGameUI::new(
            $state,
            game,
            $seed,
            $high_scores,
            $crate::common::TestRendererRequestConsumer::new(),
            None,
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
            health: mmheroes_core::logic::characteristics::HealthLevel,
            money: i16,
            brain: mmheroes_core::logic::characteristics::BrainLevel,
            stamina: mmheroes_core::logic::characteristics::StaminaLevel,
            charisma: i16,
        }
        assert_eq!(
            Characterisctis {
                health: $state.player().health(),
                money: $state.player().money().0,
                brain: $state.player().brain(),
                stamina: $state.player().stamina(),
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
                    .knowledge(),
                calculus: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::Calculus)
                    .knowledge(),
                geometry: $state
                    .player()
                    .status_for_subject(
                        mmheroes_core::logic::Subject::GeometryAndTopology
                    )
                    .knowledge(),
                cs: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::ComputerScience)
                    .knowledge(),
                english: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::English)
                    .knowledge(),
                pe: $state
                    .player()
                    .status_for_subject(mmheroes_core::logic::Subject::PhysicalEducation)
                    .knowledge(),
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

#[macro_export]
macro_rules! assert_ui {
    ($game_ui:expr, $expected:literal) => {
        assert_eq!(
            $game_ui.request_consumer().to_string().trim_end(),
            $expected[1..].trim_end(),
        );
    };
}
