#![macro_use]

macro_rules! write_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {{
        $renderer.set_color(Color::$color, Color::Black);
        write!($renderer, $($arg)*)
    }};
}

macro_rules! writeln_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {{
        $renderer.set_color(Color::$color, Color::Black);
        writeln!($renderer, $($arg)*);
    }};
}

mod screens;

pub mod renderer;

pub use renderer::RendererRequest;
use renderer::*;

pub(crate) mod cp866_encoding;
pub mod recording;

pub mod high_scores;

mod dialog;

use dialog::*;

use high_scores::HighScore;

use crate::logic::*;

use core::fmt::Display;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum Color {
    #[default]
    Black = 0, // 0x0
    Red = 1,            // 0x4
    Yellow = 3,         // 0x6
    Blue = 4,           // 0x1
    Magenta = 5,        // 0x5
    Cyan = 6,           // 0x3
    White = 7,          // 0x7
    Gray = 8,           // 0x8
    RedBright = 9,      // 0xC
    Green = 10,         // 0xA
    YellowBright = 11,  // 0xE
    BlueBright = 12,    // 0x9
    MagentaBright = 13, // 0xD
    CyanBright = 14,    // 0xB
    WhiteBright = 15,   // 0xF
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let color = match value {
            0 => Color::Black,
            1 => Color::Red,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::Gray,
            9 => Color::RedBright,
            10 => Color::Green,
            11 => Color::YellowBright,
            12 => Color::BlueBright,
            13 => Color::MagentaBright,
            14 => Color::CyanBright,
            15 => Color::WhiteBright,
            _ => return Err(()),
        };
        assert_eq!(color as u8, value);
        Ok(color)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    KeyUp,
    KeyDown,
    Enter,
    Other,
}

#[derive(Debug, Clone)]
enum WaitingState {
    PressAnyKey,
    Dialog {
        current_choice: u8,
        start: (Line, Column),
    },
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Milliseconds(pub i32);

pub struct GameUI<'game, G, C: RendererRequestConsumer> {
    renderer: Renderer<C>,
    state_holder: &'game StateHolder,
    game: core::pin::Pin<&'game mut G>,
    rng: crate::random::Rng,
    pub high_scores: [HighScore; high_scores::SCORE_COUNT],
}

impl<'game, G: Game, C: RendererRequestConsumer> GameUI<'game, G, C> {
    pub fn new(
        state_holder: &'game StateHolder,
        game: core::pin::Pin<&'game mut G>,
        seed: u64,
        high_scores: Option<[HighScore; high_scores::SCORE_COUNT]>,
        renderer_request_consumer: C,
    ) -> Self {
        let default_high_scores = high_scores::default_high_scores();
        GameUI {
            renderer: Renderer::new(renderer_request_consumer),
            state_holder,
            game,
            rng: crate::random::Rng::new(seed),
            high_scores: high_scores.unwrap_or(default_high_scores),
        }
    }

    pub fn continue_game(&mut self, input: Input) -> bool {
        use GameScreen::*;

        if let Some(ref waiting_state) = self.renderer.waiting_state {
            let waiting_state = waiting_state.clone();

            let action = match waiting_state {
                WaitingState::PressAnyKey => Action::AnyKey,
                WaitingState::Dialog {
                    current_choice,
                    start,
                } => {
                    let borrowed_state = self.state_holder.observable_state();
                    let actions = borrowed_state.available_actions();
                    let option_count = actions.len() as u8;
                    match input {
                        Input::KeyUp => {
                            let current_choice =
                                (option_count + current_choice - 1) % option_count;
                            display_dialog(
                                &mut self.renderer,
                                start,
                                Some(current_choice),
                                actions,
                            );
                            self.renderer.waiting_state = Some(WaitingState::Dialog {
                                current_choice,
                                start,
                            });
                            return true;
                        }
                        Input::KeyDown => {
                            let current_choice =
                                (option_count + current_choice + 1) % option_count;
                            display_dialog(
                                &mut self.renderer,
                                start,
                                Some(current_choice),
                                actions,
                            );
                            self.renderer.waiting_state = Some(WaitingState::Dialog {
                                current_choice,
                                start,
                            });
                            return true;
                        }
                        Input::Enter => {
                            display_dialog(&mut self.renderer, start, None, actions);
                            actions[current_choice as usize]
                        }
                        Input::Other => return true, // Do nothing
                    }
                }
            };

            self.game.as_mut().perform_action(action);
        }

        let new_waiting_state = match self.state_holder.observable_state().screen() {
            Intro => screens::initial::display_intro(&mut self.renderer),
            InitialParameters => screens::initial::display_initial_parameters(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                self.state_holder.observable_state().mode(),
            ),
            Ding => screens::initial::display_ding(&mut self.renderer),
            Timetable(state) => {
                screens::timetable::display_timetable(&mut self.renderer, state)
            }
            SceneRouter(state) => screens::scene_router::display_scene_router(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                state,
            ),
            Study(state) => screens::scene_router::display_study_options(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                state,
            ),
            PromptUseLectureNotes(_state) => {
                screens::scene_router::display_prompt_use_lecture_notes(
                    &mut self.renderer,
                    self.state_holder.observable_state().available_actions(),
                )
            }
            Sleep(state) => {
                screens::scene_router::display_sleeping(&mut self.renderer, state)
            }
            HighScores(_) => screens::high_scores::display_high_scores(
                &mut self.renderer,
                &self.high_scores,
            ),
            RestInMausoleum(state) => screens::rest::display_rest_in_mausoleum(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                state,
            ),
            CafePUNK(state) => screens::rest::display_cafe(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                state,
            ),
            TrainToPDMI(state, interaction) => screens::train::display_train_to_pdmi(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                state,
                *interaction,
            ),
            BaltiyskiyRailwayStation(scene) => {
                screens::train::display_baltiyskiy_railway_station(
                    &mut self.renderer,
                    self.state_holder.observable_state().available_actions(),
                    scene,
                )
            }
            KolyaInteraction(state, interaction) => {
                screens::npc::display_kolya_interaction(
                    &mut self.renderer,
                    state,
                    self.state_holder.observable_state().available_actions(),
                    *interaction,
                )
            }
            PashaInteraction(state, interaction) => {
                screens::npc::display_pasha_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                )
            }
            GrishaInteraction(state, interaction) => {
                screens::npc::display_grisha_interaction(
                    &mut self.renderer,
                    state,
                    self.state_holder.observable_state().available_actions(),
                    *interaction,
                )
            }
            SashaInteraction(state, interaction) => {
                screens::npc::display_sasha_interaction(
                    &mut self.renderer,
                    state,
                    self.state_holder.observable_state().available_actions(),
                    *interaction,
                )
            }
            KuzmenkoInteraction(state, interaction) => {
                screens::npc::display_kuzmenko_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                )
            }
            DiamondInteraction(state, interaction, diamond_leaves) => {
                screens::npc::display_diamond_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                    self.state_holder.observable_state().available_actions(),
                    *diamond_leaves,
                )
            }
            SerjInteraction(state, interaction, serj_leaves) => {
                screens::npc::display_serj_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                    *serj_leaves,
                )
            }
            MishaInteraction(interaction) => screens::npc::display_misha_interaction(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                interaction,
            ),
            AndrewInteraction(interaction) => screens::npc::display_andrew_interaction(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                interaction,
            ),
            DjugInteraction(state) => {
                screens::npc::display_djug_interaction(&mut self.renderer, state)
            }
            RaiInteraction(interaction) => screens::npc::display_rai_interaction(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                interaction,
            ),
            NilInteraction(interaction) => screens::npc::display_nil_interaction(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                interaction,
            ),
            Terkom(state, terkom_screen) => screens::terkom::display_terkom(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
                &mut self.rng,
                state,
                *terkom_screen,
            ),
            GoToProfessor(state) => screens::scene_router::display_available_professors(
                &mut self.renderer,
                state,
                self.state_holder.observable_state().available_actions(),
            ),
            ExamIntro(intro) => {
                screens::exam::display_exam_intro(&mut self.renderer, *intro)
            }
            Exam(scene) => screens::exam::display_exam(
                &mut self.renderer,
                &mut self.rng,
                self.state_holder.observable_state().available_actions(),
                scene,
            ),
            SurfInternet { found_program } => {
                screens::scene_router::display_surfing_internet(
                    &mut self.renderer,
                    *found_program,
                )
            }
            IAmDone(_) => screens::game_end::display_i_am_done(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            GameEnd(state) => {
                screens::game_end::display_game_end(&mut self.renderer, state)
            }
            WannaTryAgain => screens::game_end::display_wanna_try_again(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            Disclaimer => screens::game_end::display_disclaimer(&mut self.renderer),
            WhatToDo(_) => screens::help::display_what_to_do(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            AboutScreen(_) => screens::help::display_about_screen(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            WhereToGoAndWhy(_) => screens::help::display_where_to_go_and_why(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            AboutProfessors(_) => screens::help::display_about_professors(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            AboutCharacters(_) => screens::help::display_about_characters(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            AboutThisProgram(_) => screens::help::display_about_this_program(
                &mut self.renderer,
                self.state_holder.observable_state().available_actions(),
            ),
            Terminal => {
                self.renderer.waiting_state = None;
                return false;
            }
        };
        self.renderer.waiting_state = Some(new_waiting_state);
        true
    }

    pub fn request_consumer(&self) -> &C {
        self.renderer.request_consumer()
    }
}

fn sleep(r: &mut Renderer<impl RendererRequestConsumer>, ms: Milliseconds) {
    r.flush();
    r.sleep_ms(ms)
}

fn wait_for_any_key(r: &mut Renderer<impl RendererRequestConsumer>) -> WaitingState {
    r.move_cursor_to(23, 0);
    r.set_color(Color::YellowBright, Color::Black);
    write!(r, "Нажми любую клавишу ...");
    r.flush();
    WaitingState::PressAnyKey
}

pub fn professor_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "Всемирнов М.А.",
        Subject::Calculus => "Дубцов Е.С.",
        Subject::GeometryAndTopology => "Подкорытов С.С.",
        Subject::ComputerScience => "Климов А.А.",
        Subject::English => "Влащенко Н.П.",
        Subject::PhysicalEducation => "Альбинский Е.Г.",
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

pub fn professor_gender(subject: Subject) -> Gender {
    match subject {
        Subject::AlgebraAndNumberTheory => Gender::Male,
        Subject::Calculus => Gender::Male,
        Subject::GeometryAndTopology => Gender::Male,
        Subject::ComputerScience => Gender::Male,
        Subject::English => Gender::Female,
        Subject::PhysicalEducation => Gender::Male,
    }
}

pub fn subject_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "Алгебра и Т.Ч.",
        Subject::Calculus => "Мат. Анализ",
        Subject::GeometryAndTopology => "Геометрия и Топология",
        Subject::ComputerScience => "Информатика",
        Subject::English => "English",
        Subject::PhysicalEducation => "Физ-ра",
    }
}

pub fn subject_short_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "АиТЧ",
        Subject::Calculus => "МатАн",
        Subject::GeometryAndTopology => "ГиТ",
        Subject::ComputerScience => "Инф",
        Subject::English => "ИнЯз",
        Subject::PhysicalEducation => "Физ-ра",
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Location::Dorm => "Общага",
            Location::PUNK => "ПУНК",
            Location::Mausoleum => "Мавзолей",
            Location::ComputerClass => "Компы",
            Location::PDMI => "ПОМИ",
        };
        f.write_str(name)
    }
}

pub fn day_date(day: &Day) -> &'static str {
    const DATES: [&str; timetable::NUM_DAYS] =
        ["22.5", "23.5", "24.5", "25.5", "26.5", "27.5"];
    DATES[day.index()]
}

pub fn classmate_name(classmate: Classmate) -> &'static str {
    match classmate {
        Classmate::Kolya => "Коля",
        Classmate::Pasha => "Паша",
        Classmate::Diamond => "Diamond",
        Classmate::RAI => "RAI",
        Classmate::Misha => "Миша",
        Classmate::Serj => "Серж",
        Classmate::Sasha => "Саша",
        Classmate::NiL => "NiL",
        Classmate::Kuzmenko => "Кузьменко В.Г.",
        Classmate::DJuG => "DJuG",
        Classmate::Andrew => "Эндрю",
        Classmate::Grisha => "Гриша",
    }
}

fn problems_inflected(problems: u8) -> &'static str {
    match problems {
        1 => "задание",
        2..=4 => "задания",
        _ => "заданий",
    }
}
