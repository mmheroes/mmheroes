use super::super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PashaInteraction {
    /// "Паша вручает тебе твою стипуху за май: 50 руб."
    Stipend,

    /// "Паша воодушевляет тебя на великие дела."
    Inspiration,
}

use PashaInteraction::*;

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    let interaction = if state.player.got_stipend() {
        Inspiration
    } else {
        Stipend
    };
    g.set_screen_and_wait_for_any_key(GameScreen::PashaInteraction(
        state.clone(),
        interaction,
    ))
    .await;
    let player = &mut state.player;
    match interaction {
        Stipend => {
            player.set_got_stipend();
            player.money += Money::stipend();
        }
        Inspiration => {
            player.stamina += 1;
            for subject in Subject::all_subjects() {
                let knowledge = &mut player.status_for_subject_mut(subject).knowledge;
                if *knowledge > 3 {
                    *knowledge -= g.rng.random(3);
                }
            }
        }
    }
}
