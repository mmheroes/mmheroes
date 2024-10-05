pub mod timetable;
pub use timetable::{Day, Duration, Time, Timetable};

pub mod characteristics;
pub use characteristics::*;

pub mod game_state;
pub use game_state::*;

pub mod subjects;
pub use subjects::*;

pub mod npc;
pub use npc::*;

pub mod player;
pub use player::Player;

pub mod subject_status;
pub use subject_status::SubjectStatus;

pub mod actions;
pub use actions::Action;
pub(in crate::logic) use actions::{illegal_action, wait_for_any_key, ActionVec};

pub mod cause_of_death;
pub use cause_of_death::*;

pub mod game_screen;
pub use game_screen::*;

pub mod scene_router;

mod entry_point;
mod legacy;

use crate::random;

use crate::util::async_support::*;
use assert_matches::*;
use core::cell::{Ref, RefCell};
use core::future::Future;
use core::pin::Pin;

/// Максимальное число возможных вариантов на главном экране.
pub const MAX_OPTIONS_IN_SCENE_ROUTER: usize = 12;

/// The game mode selector.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum GameMode {
    /// Normal game mode, the character has average characteristics.
    /// This is the default.
    Normal,

    /// The player will be prompted to select initial character characteristics:
    /// - Random student, same as `Normal` mode.
    /// - Clever student: better brain, but worse stamina and charisma
    /// - Impudent student: better stamina, but worse brain and charisma
    /// - Sociable student: better charisma, but worse brain and stamina.
    SelectInitialParameters,

    /// Same as `SelectInitialParameters`, but another option is available —
    /// God mode. It enables maximum abilities.
    ///
    /// This mode is enable by passing a special flag to the executable.
    God,
}

pub struct ObservableGameState {
    mode: GameMode,
    screen: GameScreen,
    available_actions: ActionVec,
}

impl ObservableGameState {
    pub fn new(mode: GameMode) -> Self {
        Self {
            mode,
            screen: GameScreen::Intro,
            available_actions: ActionVec::new(),
        }
    }

    fn reset(&mut self) {
        self.screen = GameScreen::Intro;
        self.available_actions.clear();
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    pub fn screen(&self) -> &GameScreen {
        &self.screen
    }

    pub fn available_actions(&self) -> &[Action] {
        &self.available_actions
    }
}

struct InternalGameState<'a> {
    observable_state: &'a RefCell<ObservableGameState>,
    rng: random::Rng,
}

impl<'a: 'b, 'b> InternalGameState<'a> {
    fn new(
        seed: u64,
        observable_state: &'a RefCell<ObservableGameState>,
    ) -> InternalGameState<'a> {
        let rng = random::Rng::new(seed);
        observable_state.borrow_mut().reset();
        InternalGameState {
            observable_state,
            rng,
        }
    }

    fn screen(&self) -> Ref<'b, GameScreen> {
        Ref::map(self.observable_state.borrow(), ObservableGameState::screen)
    }

    fn set_screen(&self, new_screen: GameScreen) {
        self.observable_state.borrow_mut().screen = new_screen;
    }

    fn set_available_actions_from_vec(&self, actions: ActionVec) {
        self.observable_state.borrow_mut().available_actions = actions
    }

    fn set_available_actions<const N: usize>(&self, actions: [Action; N]) {
        self.set_available_actions_from_vec(ActionVec::from(actions))
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    fn perform_action(&mut self, action: Action) -> ActionVec {
        use GameScreen::*;
        let borrowed_screen = self.screen();
        match &*borrowed_screen {
            Terminal => panic!("Attempted to perform an action in terminal state"),
            Intro => {
                unreachable!()
            }
            InitialParameters => {
                drop(borrowed_screen);
                legacy::ding(self, action)
            }
            Ding(player) => {
                // TODO: Remove player
                let state = GameState::new(
                    player.clone(),
                    timetable::Timetable::random(&mut self.rng),
                    Location::Dorm,
                );
                drop(borrowed_screen);
                legacy::view_timetable(self, state)
            }
            Timetable(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::run_sync(self, state)
            }
            SceneRouter(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::handle_action(self, state, action)
            }
            Study(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::dorm::choose_use_lecture_notes(self, state, action)
            }
            PromptUseLectureNotes(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                let (subject, use_lecture_notes) = match action {
                    Action::UseLectureNotes(subject) => (subject, true),
                    Action::DontUseLectureNotes(subject) => (subject, false),
                    _ => illegal_action!(action),
                };
                scene_router::dorm::study(self, state, subject, use_lecture_notes)
            }
            Sleep(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::dorm::handle_sleeping(self, state, action)
            }
            HighScores(state) => match action {
                Action::AnyKey => {
                    let state = state.clone();
                    drop(borrowed_screen);
                    scene_router::run_sync(self, state)
                }
                _ => illegal_action!(action),
            },
            RestInMausoleum(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::mausoleum::handle_rest(self, state, action)
            }
            CafePUNK(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::punk::handle_cafe_punk_action(self, state, action)
            }
            TrainToPDMI(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                drop(borrowed_screen);
                scene_router::train::proceed_with_train(self, state, action, interaction)
            }
            KolyaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                drop(borrowed_screen);
                npc::kolya::proceed(self, state, action, interaction)
            }
            PashaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                drop(borrowed_screen);
                npc::pasha::proceed(self, state, action, interaction)
            }
            GrishaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                drop(borrowed_screen);
                npc::grisha::proceed(self, state, action, interaction)
            }
            SashaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                drop(borrowed_screen);
                npc::sasha::proceed(self, state, action, interaction)
            }
            KuzmenkoInteraction(state, _) => {
                assert_eq!(action, Action::AnyKey);
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::run_sync(self, state)
            }
            GoToProfessor(state) => match action {
                Action::Exam(subject) => {
                    let state = state.clone();
                    drop(borrowed_screen);
                    self.enter_exam(state, subject)
                }
                Action::DontGoToProfessor => {
                    let state = state.clone();
                    drop(borrowed_screen);
                    scene_router::run_sync(self, state)
                }
                _ => illegal_action!(action),
            },
            Exam(_state, _subject) => {
                todo!()
            }
            SurfInternet(state, found_program) => {
                let state = state.clone();
                let found_program = *found_program;
                drop(borrowed_screen);
                scene_router::computer_class::proceed_with_internet(
                    self,
                    state,
                    action,
                    found_program,
                )
            }
            IAmDone(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::handle_i_am_done(self, state, action)
            }
            GameEnd(_) => {
                drop(borrowed_screen);
                scene_router::wanna_try_again(self)
            }
            WannaTryAgain => {
                drop(borrowed_screen);
                scene_router::handle_wanna_try_again(self, action)
            }
            Disclaimer => {
                drop(borrowed_screen);
                self.set_screen(Terminal);
                ActionVec::new()
            }
            WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state) => {
                let state = state.clone();
                drop(borrowed_screen);
                scene_router::dorm::handle_what_to_do(self, state, action)
            }
        }
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        let (god_mode, brain, stamina, charisma) = match parameters {
            Action::RandomStudent => (
                false,
                BrainLevel(self.rng.random_in_range(4..7)),
                StaminaLevel(self.rng.random_in_range(4..7)),
                CharismaLevel(self.rng.random_in_range(4..7)),
            ),
            Action::CleverStudent => (
                false,
                BrainLevel(self.rng.random_in_range(5..10)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            Action::ImpudentStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(5..10)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            Action::SociableStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(5..10)),
            ),
            Action::GodMode => {
                (true, BrainLevel(30), StaminaLevel(30), CharismaLevel(30))
            }
            _ => illegal_action!(parameters),
        };

        let health = HealthLevel(self.rng.random(stamina.0 * 2) + 40);

        Player::new(god_mode, health, brain, stamina, charisma, |_| {
            self.rng.random(brain)
        })
    }

    fn enter_exam(&mut self, _state: GameState, _subject: Subject) -> ActionVec {
        todo!()
    }

    fn decrease_health<F: FnOnce(&mut InternalGameState, GameState) -> ActionVec>(
        &mut self,
        delta: HealthLevel,
        mut state: GameState,
        cause_of_death: CauseOfDeath,
        if_alive: F,
    ) -> ActionVec {
        if state.player.health <= delta {
            state.player.cause_of_death = Some(cause_of_death);
            scene_router::game_end(self, state)
        } else {
            state.player.health -= delta;
            if_alive(self, state)
        }
    }

    fn midnight(&mut self, state: GameState) -> ActionVec {
        match state.location {
            Location::PUNK => todo!("sub_1E907"),
            Location::PDMI => todo!("sub_1E7F8"),
            Location::ComputerClass => {
                unreachable!("Компьютерный класс уже должен быть закрыт в полночь!")
            }
            Location::Dorm => scene_router::dorm::go_to_sleep(self, state),
            Location::Mausoleum => todo!("sub_1E993"),
        }
    }

    fn run_classmate_routines(&mut self, state: &mut GameState) {
        let timetable = &state.timetable;
        let day = &timetable.days()[state.current_day_index];
        let time = state.current_time;
        let classmates = &mut state.classmates;
        for classmate in classmates.iter_mut() {
            classmate.update(&mut self.rng, state.location, day, time);
        }
    }

    fn hour_pass(&mut self, mut state: GameState) -> ActionVec {
        // TODO: Lot of stuff going on here

        // TODO: Поменять эти строки местами и не забыть отредактировать метод
        // Time::is_between_9_and_19()!
        self.run_classmate_routines(&mut state);
        state.current_time += Duration(1);

        if state.player.charisma <= CharismaLevel(0) {
            state.player.health = HealthLevel(0);
        }

        if state.current_time.is_midnight() {
            state.current_day_index += 1;
            state.current_time = Time(0);
            return self.midnight(state);
        }

        scene_router::run_sync(self, state)
    }

    async fn wait_for_action(&self) -> Action {
        let action = prompt(()).await;
        if !self
            .observable_state
            .borrow()
            .available_actions()
            .contains(&action)
        {
            illegal_action!(action);
        }
        action
    }

    async fn wait_for_any_key(&self) {
        self.set_available_actions([Action::AnyKey]);
        self.wait_for_action().await;
    }
}

pub trait Game {
    fn perform_action(self: Pin<&mut Self>, action: Action);
}

pub(in crate::logic) type GameExecutor<F> = PromptingExecutor<F, Action, ()>;

impl<'a, F: Future> Game for GameExecutor<F> {
    fn perform_action(self: Pin<&mut Self>, action: Action) {
        // FIXME: The result is ignored, we probably don't want that
        self.resume_with_input(action);
    }
}

pub fn create_game(seed: u64, state: &RefCell<ObservableGameState>) -> impl Game + '_ {
    let mut game = InternalGameState::new(seed, state);
    GameExecutor::new(async move { entry_point::run(&mut game).await })
}

#[test]
fn memory() {
    let observable_game_state = ObservableGameState::new(GameMode::Normal);
    assert_eq!(size_of_val(&observable_game_state), 344);
    assert_eq!(size_of::<Player>(), 40);
    assert_eq!(size_of::<Action>(), 2);
    assert_eq!(size_of::<GameScreen>(), 296);

    let observable_game_state = RefCell::new(observable_game_state);
    let game = create_game(0, &observable_game_state);
    assert_eq!(size_of_val(&game), 976);
}
