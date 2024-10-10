use crate::logic::actions::{illegal_action, ActionVec};
use crate::logic::{Action, GameScreen, GameState, InternalGameState};

pub(super) async fn go_to_professor(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    let mut available_actions = state
        .current_day()
        .current_exams(state.location, state.current_time)
        .map(|exam| Action::Exam(exam.subject()))
        .collect::<ActionVec>();
    available_actions.push(Action::DontGoToProfessor);
    g.set_screen_and_action_vec(
        GameScreen::GoToProfessor(state.clone()),
        available_actions,
    );
    let _subject = match g.wait_for_action().await {
        Action::Exam(subject) => subject,
        Action::DontGoToProfessor => return,
        action => illegal_action!(action),
    };
    todo!("Экзамен")
}
