use super::super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PashaInteraction {
    /// "Паша вручает тебе твою стипуху за май: 50 руб."
    Stipend,

    /// "Паша воодушевляет тебя на великие дела."
    Inspiration,
}

use PashaInteraction::*;

pub(in crate::logic) fn interact(game: &mut Game, state: GameState) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
    let interaction = if state.player.got_stipend() {
        Inspiration
    } else {
        Stipend
    };
    game.screen = GameScreen::PashaInteraction(state, interaction);
    wait_for_any_key()
}

pub(in crate::logic) fn proceed(
    game: &mut Game,
    mut state: GameState,
    action: Action,
    interaction: PashaInteraction,
) -> ActionVec {
    assert_eq!(action, Action::AnyKey);
    assert_eq!(state.location, Location::PUNK);
    assert_matches!(game.screen, GameScreen::PashaInteraction(_, _));
    let player = &mut state.player;
    match interaction {
        Stipend => {
            assert!(!player.got_stipend());
            player.set_got_stipend();
            player.money += Money::stipend();
        }
        Inspiration => {
            player.stamina += 1;
            for (subject, _) in SUBJECTS.iter() {
                let knowledge = &mut player.status_for_subject_mut(*subject).knowledge;
                if *knowledge > BrainLevel(3) {
                    *knowledge -= game.rng.random(3);
                }
            }
        }
    }
    scene_router::run(game, state)
}
