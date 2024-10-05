use super::super::*;

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

use SashaInteraction::*;

pub(in crate::logic) fn interact(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
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
    game.set_screen(GameScreen::SashaInteraction(state, ChooseSubject));
    available_actions
}

pub(in crate::logic) fn proceed(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: SashaInteraction,
) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
    assert_matches!(&*game.screen(), GameScreen::SashaInteraction(_, _));
    match action {
        Action::RequestLectureNotesFromSasha(subject) => {
            assert_eq!(interaction, ChooseSubject);
            let new_interaction = if state.player.charisma
                > CharismaLevel(game.rng.random(18))
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
            };
            game.set_screen(GameScreen::SashaInteraction(state, new_interaction));
            wait_for_any_key()
        }
        Action::DontNeedAnythingFromSasha => {
            assert_eq!(interaction, ChooseSubject);
            game.set_screen(GameScreen::SashaInteraction(state, SuitYourself));
            wait_for_any_key()
        }
        Action::AnyKey => {
            assert_ne!(interaction, ChooseSubject);
            scene_router::run_sync(game, state)
        }
        _ => illegal_action!(action),
    }
}
