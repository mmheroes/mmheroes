mod common;
use common::*;

use assert_matches::*;
use mmheroes_core::logic::actions::PlayStyle;
use mmheroes_core::logic::*;

/// Проверяем что экран выбора стиля игры показывается после перезапуска
#[test]
fn initial_parameters_screen_shown_when_rerunning() {
    initialize_game!((0, GameMode::SelectInitialParameters) => state, game_ui);
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::InitialParameters);
    replay_game(&mut game_ui, "↓3r2↑r↓3r");
    assert_matches!(state.borrow().screen(), GameScreen::InitialParameters);
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::Ding(_));
}

/// Проверяем что игра завершается корректно
#[test]
fn game_end() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_game(&mut game_ui, "3r2↑2r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(_));
    replay_game(&mut game_ui, "2↑r↓r");
    assert_matches!(state.borrow().screen(), GameScreen::GameEnd(_));
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::WannaTryAgain);
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::Ding(_));
    replay_game(&mut game_ui, "2r2↑r↓2r↓r");
    assert_matches!(state.borrow().screen(), GameScreen::Disclaimer);
    assert!(replay_game(&mut game_ui, "r"));
    assert_matches!(state.borrow().screen(), GameScreen::Terminal);
}

#[test]
fn game_end_after_visiting_punk() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);
    replay_game(&mut game_ui, "4↓r↑r");
    assert_matches!(state.borrow().screen(), GameScreen::IAmDone(_));
}

#[test]
fn show_timetable_in_dorm() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);
    replay_game(&mut game_ui, "↓r");
    assert_matches!(state.borrow().screen(), GameScreen::Timetable(_));
    replay_game(&mut game_ui, "r↓r");
    assert_matches!(state.borrow().screen(), GameScreen::Timetable(_));
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(_));
}

#[test]
fn show_help() {
    initialize_game!((0, GameMode::Normal) => state, game_ui);
    replay_until_dorm(&state, &mut game_ui, PlayStyle::RandomStudent);
    replay_game(&mut game_ui, "↑r");
    assert_matches!(state.borrow().screen(), GameScreen::WhatToDo(_));
    replay_game(&mut game_ui, "r");
    assert_matches!(state.borrow().screen(), GameScreen::WhatToDo(_));
    replay_game(&mut game_ui, "↓r");
    assert_matches!(state.borrow().screen(), GameScreen::AboutScreen(_));
    replay_game(&mut game_ui, "2↓r");
    assert_matches!(state.borrow().screen(), GameScreen::WhereToGoAndWhy(_));
    replay_game(&mut game_ui, "3↓r");
    assert_matches!(state.borrow().screen(), GameScreen::AboutProfessors(_));
    replay_game(&mut game_ui, "4↓r");
    assert_matches!(state.borrow().screen(), GameScreen::AboutCharacters(_));
    replay_game(&mut game_ui, "5↓r");
    assert_matches!(state.borrow().screen(), GameScreen::AboutThisProgram(_));
    replay_game(&mut game_ui, "6↓r");
    assert_matches!(state.borrow().screen(), GameScreen::SceneRouter(_));
}
