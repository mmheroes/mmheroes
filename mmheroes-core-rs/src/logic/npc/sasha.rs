#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SashaInteraction {
    /// Выбор предмета, по которому попросить конспект у Саши
    ChooseSubject,

    /// "Как знаешь..."
    SuitYourself,

    /// "Да, у меня с собой этот конспект ..."
    YesIHaveTheLectureNotes,

    /// "Ох, извини, кто-то другой уже позаимствовал ..."
    SorryGaveToSomeoneElse,
}

use crate::logic::actions::{illegal_action, ActionVec};
use crate::logic::{
    Action, CharismaLevel, GameScreen, GameState, InternalGameState, Location,
    SUBJECTS_WITH_LECTURE_NOTES,
};
use SashaInteraction::*;

pub(super) async fn interact(g: &mut InternalGameState<'_>, state: &mut GameState) {
    assert_eq!(state.location(), Location::PUNK);
    let mut available_actions = SUBJECTS_WITH_LECTURE_NOTES
        .into_iter()
        .filter(|subject| {
            !state
                .player
                .status_for_subject(*subject)
                .has_lecture_notes()
        })
        .map(Action::RequestLectureNotesFromSasha)
        .collect::<ActionVec>();
    available_actions.push(Action::DontNeedAnythingFromSasha);
    g.set_screen_and_action_vec(
        GameScreen::SashaInteraction(ChooseSubject),
        available_actions,
    );
    let new_interaction = match g.wait_for_action().await {
        Action::RequestLectureNotesFromSasha(subject) => {
            if state.player.charisma > CharismaLevel(g.rng.random(18))
                && state.sasha_has_lecture_notes(subject)
            {
                state
                    .player
                    .status_for_subject_mut(subject)
                    .set_has_lecture_notes();
                YesIHaveTheLectureNotes
            } else {
                state.set_sasha_has_lecture_notes(subject, false);
                SorryGaveToSomeoneElse
            }
        }
        Action::DontNeedAnythingFromSasha => SuitYourself,
        action => illegal_action!(action),
    };
    g.set_screen_and_wait_for_any_key(GameScreen::SashaInteraction(new_interaction))
        .await;
}
