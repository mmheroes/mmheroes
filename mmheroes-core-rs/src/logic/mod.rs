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
pub(in crate::logic) use actions::{illegal_action, ActionVec};

pub mod cause_of_death;
pub use cause_of_death::*;

pub mod game_screen;
pub use game_screen::*;

pub mod scene_router;

mod entry_point;
mod misc;

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
    /// This mode is enabled by passing a special flag to the executable.
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
    state_holder: &'a StateHolder,
    rng: random::Rng,
}

impl<'a: 'b, 'b> InternalGameState<'a> {
    fn new(seed: u64, state_holder: &'a StateHolder) -> InternalGameState<'a> {
        let rng = random::Rng::new(seed);
        state_holder.observable_state.borrow_mut().reset();
        InternalGameState { state_holder, rng }
    }

    fn set_screen(&self, new_screen: GameScreen) {
        self.state_holder.observable_state.borrow_mut().screen = new_screen;
    }

    async fn set_screen_and_wait_for_any_key(&self, new_screen: GameScreen) {
        self.set_screen(new_screen);
        self.wait_for_any_key().await;
    }

    fn set_screen_and_action_vec(&self, new_screen: GameScreen, actions: ActionVec) {
        let mut state = self.state_holder.observable_state.borrow_mut();
        state.screen = new_screen;
        state.available_actions = actions;
    }

    fn set_available_actions_from_vec(&self, actions: ActionVec) {
        self.state_holder
            .observable_state
            .borrow_mut()
            .available_actions = actions
    }

    fn set_screen_and_available_actions<const N: usize>(
        &self,
        new_screen: GameScreen,
        actions: [Action; N],
    ) {
        self.set_screen_and_action_vec(new_screen, ActionVec::from(actions))
    }

    fn set_available_actions<const N: usize>(&self, actions: [Action; N]) {
        self.set_available_actions_from_vec(ActionVec::from(actions))
    }

    fn initialize_player(&mut self, style: actions::PlayStyle) -> Player {
        let (god_mode, brain, stamina, charisma) = match style {
            actions::PlayStyle::RandomStudent => (
                false,
                BrainLevel(self.rng.random_in_range(4..7)),
                StaminaLevel(self.rng.random_in_range(4..7)),
                CharismaLevel(self.rng.random_in_range(4..7)),
            ),
            actions::PlayStyle::CleverStudent => (
                false,
                BrainLevel(self.rng.random_in_range(5..10)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            actions::PlayStyle::ImpudentStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(5..10)),
                CharismaLevel(self.rng.random_in_range(2..5)),
            ),
            actions::PlayStyle::SociableStudent => (
                false,
                BrainLevel(self.rng.random_in_range(2..5)),
                StaminaLevel(self.rng.random_in_range(2..5)),
                CharismaLevel(self.rng.random_in_range(5..10)),
            ),
            actions::PlayStyle::GodMode => {
                (true, BrainLevel(30), StaminaLevel(30), CharismaLevel(30))
            }
        };

        let health = HealthLevel(self.rng.random(stamina.0 * 2) + 40);

        Player::new(god_mode, health, brain, stamina, charisma, |_| {
            self.rng.random(brain)
        })
    }

    fn run_classmate_routines(&mut self, state: &mut GameState) {
        let timetable = &state.timetable;
        let day = timetable.day(state.current_day_index());
        let time = state.current_time();
        let location = state.location();
        let classmates = &mut state.classmates;
        for classmate in classmates.iter_mut() {
            classmate.update(&mut self.rng, location, day, time);
        }
    }

    async fn wait_for_action(&self) -> Action {
        let action = prompt((), &self.state_holder.shared_future_data).await;
        if !self
            .state_holder
            .observable_state()
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

pub(in crate::logic) type GameExecutor<'a, F> = PromptingExecutor<'a, F, Action, ()>;

impl<F: Future> Game for GameExecutor<'_, F> {
    fn perform_action(self: Pin<&mut Self>, action: Action) {
        // FIXME: The result is ignored, we probably don't want that
        self.resume_with_input(action);
    }
}

pub struct StateHolder {
    observable_state: RefCell<ObservableGameState>,
    shared_future_data: RefCell<Option<FutureData<Action, ()>>>,
}

impl StateHolder {
    pub fn new(mode: GameMode) -> Self {
        Self {
            observable_state: RefCell::new(ObservableGameState::new(mode)),
            shared_future_data: RefCell::new(None),
        }
    }

    pub fn observable_state(&self) -> Ref<ObservableGameState> {
        self.observable_state.borrow()
    }
}

pub fn create_game(seed: u64, state_holder: &StateHolder) -> impl Game + '_ {
    let mut game = InternalGameState::new(seed, state_holder);
    GameExecutor::new(
        async move { entry_point::run(&mut game).await },
        &state_holder.shared_future_data,
    )
}

#[test]
fn memory() {
    let observable_game_state = ObservableGameState::new(GameMode::Normal);
    assert_eq!(size_of_val(&observable_game_state), 328);
    assert_eq!(size_of::<Player>(), 40);
    assert_eq!(size_of::<Action>(), 2);
    assert_eq!(size_of::<GameState>(), 272);
    assert_eq!(size_of::<GameScreen>(), 280);

    let state_holder = StateHolder::new(GameMode::Normal);
    let game = create_game(0, &state_holder);
    assert_eq!(size_of_val(&game), 1584);
}
