use super::super::*;

pub(in crate::logic) fn interact(game: &mut Game, state: GameState) -> ActionVec {
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
    game.screen =
        GameScreen::SashaInteraction(state, npc::SashaInteraction::ChooseSubject);
    available_actions
}

pub(in crate::logic) fn proceed(
    game: &mut Game,
    mut state: GameState,
    action: Action,
    interaction: npc::SashaInteraction,
) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
    assert_matches!(game.screen, GameScreen::SashaInteraction(_, _));
    match action {
        Action::RequestLectureNotesFromSasha(subject) => {
            assert_eq!(interaction, npc::SashaInteraction::ChooseSubject);
            let new_interaction = if state.player.charisma
                > CharismaLevel(game.rng.random(18))
                && state.sasha_has_lecture_notes(subject)
            {
                state
                    .player
                    .status_for_subject_mut(subject)
                    .set_has_lecture_notes();
                npc::SashaInteraction::YesIHaveTheLectureNotes
            } else {
                state.set_sasha_has_lecture_notes(subject, false);
                npc::SashaInteraction::SorryGaveToSomeoneElse
            };
            game.screen = GameScreen::SashaInteraction(state, new_interaction);
            wait_for_any_key()
        }
        Action::DontNeedAnythingFromSasha => {
            assert_eq!(interaction, npc::SashaInteraction::ChooseSubject);
            game.screen =
                GameScreen::SashaInteraction(state, npc::SashaInteraction::SuitYourself);
            wait_for_any_key()
        }
        Action::AnyKey => {
            assert_ne!(interaction, npc::SashaInteraction::ChooseSubject);
            game.scene_router(state)
        }
        _ => illegal_action!(action),
    }
}
