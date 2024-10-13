//! # Устаревшие функции
//!
//! Пока мы в процессе переписывания игры в стиле async/await нам всё ещё нужны эти
//! функции. Когда вся игра будет переписана, их можно будет удалить.
//!

#![allow(deprecated)]

use crate::logic::actions::{wait_for_any_key, ActionVec, HelpAction};
use crate::logic::kolya::KolyaInteraction;
use crate::logic::kolya::KolyaInteraction::{
    Altruism, BrakeFluidBecauseRefused, BrakeFluidNoMoney, PromptOatTincture,
    SolvedAlgebraProblemsForFree, SolvedAlgebraProblemsForOatTincture,
};
use crate::logic::kuzmenko::KuzmenkoInteraction::{
    AdditionalComputerScienceExam, BillGatesMustDie, ByteVisualization, CSeminar,
    FiltersInWindows, FormatFloppy, GetYourselvesAnEmail, MmheroesBP7, MonitorJournal,
    OlegPliss, TerekhovSenior, ThirdYear, STAR,
};
use crate::logic::pasha::PashaInteraction;
use crate::logic::pasha::PashaInteraction::{Inspiration, Stipend};
use crate::logic::sasha::SashaInteraction;
use crate::logic::sasha::SashaInteraction::{
    ChooseSubject, SorryGaveToSomeoneElse, SuitYourself, YesIHaveTheLectureNotes,
};
use crate::logic::scene_router::train;
use crate::logic::scene_router::train::TrainToPDMI;
use crate::logic::scene_router::train::TrainToPDMI::{
    BoughtRoundtripTicket, GatecrashBecauseNoMoney, GatecrashByChoice, NoPointToGoToPDMI,
    PromptToBuyTickets,
};
use crate::logic::*;
use crate::random::Rng;

#[deprecated]
pub(in crate::logic) fn start_game(g: &mut InternalGameState) -> ActionVec {
    if entry_point::should_select_game_style(g) {
        g.state_holder.observable_state().available_actions.clone()
    } else {
        ding(
            g,
            Action::SelectPlayStyle(actions::PlayStyle::RandomStudent),
        )
    }
}

#[deprecated]
pub(in crate::logic) fn ding(g: &mut InternalGameState, action: Action) -> ActionVec {
    let play_style = match action {
        Action::SelectPlayStyle(play_style) => play_style,
        _ => illegal_action!(action),
    };
    let player = g.initialize_player(play_style);
    g.set_screen(GameScreen::Ding(player));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn view_timetable(
    g: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    g.set_screen(GameScreen::Timetable(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn scene_router_run(
    game: &mut InternalGameState,
    state: &GameState,
) -> ActionVec {
    let available_actions = scene_router::available_actions(state);
    game.set_screen(GameScreen::SceneRouter(state.clone()));
    available_actions
}

#[deprecated]
pub(in crate::logic) fn handle_action_sync(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    use Location::*;
    match state.location() {
        PUNK => handle_punk_action(game, state, action),
        PDMI => handle_pdmi_action(game, state, action),
        ComputerClass => handle_computer_class_action(game, state, action),
        Dorm => handle_dorm_action(game, state, action),
        Mausoleum => handle_mausoleum_action(game, state, action),
    }
}

#[deprecated]
pub(in crate::logic) fn game_end(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    game.set_screen(GameScreen::GameEnd(state));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn wanna_try_again(game: &mut InternalGameState) -> ActionVec {
    game.set_screen(GameScreen::WannaTryAgain);
    // Хочешь попробовать снова? Да или нет.
    ActionVec::from([Action::WantToTryAgain, Action::DontWantToTryAgain])
}

#[deprecated]
pub(in crate::logic) fn handle_wanna_try_again(
    game: &mut InternalGameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::WantToTryAgain => start_game(game),
        Action::DontWantToTryAgain => {
            game.set_screen(GameScreen::Disclaimer);
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_dorm_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    match action {
        Action::Study => {
            game.set_screen(GameScreen::Study(state.clone()));
            scene_router::dorm::subjects_to_study(&state)
        }
        Action::ViewTimetable => view_timetable(game, state),
        Action::Rest => rest(game, state),
        Action::GoToBed => try_to_sleep(game, state),
        Action::GoFromDormToPunk => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToPDMI => go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::WhatToDo => handle_what_to_do(game, state, HelpAction::WhatToDoAtAll),
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_what_to_do(
    game: &mut InternalGameState,
    state: GameState,
    action: HelpAction,
) -> ActionVec {
    use crate::logic::GameScreen::*;
    assert_eq!(state.location(), Location::Dorm);
    game.set_screen(match action {
        HelpAction::WhatToDoAtAll => WhatToDo(state),
        HelpAction::AboutScreen => AboutScreen(state),
        HelpAction::WhereToGoAndWhy => WhereToGoAndWhy(state),
        HelpAction::AboutProfessors => AboutProfessors(state),
        HelpAction::AboutCharacters => AboutCharacters(state),
        HelpAction::AboutThisProgram => AboutThisProgram(state),
        HelpAction::ThanksButNothing => {
            return scene_router_run(game, &state);
        }
    });
    ActionVec::from([
        Action::Help(HelpAction::WhatToDoAtAll),
        Action::Help(HelpAction::AboutScreen),
        Action::Help(HelpAction::WhereToGoAndWhy),
        Action::Help(HelpAction::AboutProfessors),
        Action::Help(HelpAction::AboutCharacters),
        Action::Help(HelpAction::AboutThisProgram),
        Action::Help(HelpAction::ThanksButNothing),
    ])
}

#[deprecated]
pub(in crate::logic) fn study(
    game: &mut InternalGameState,
    mut state: GameState,
    subject: Subject,
    use_lecture_notes: bool,
) -> ActionVec {
    // Импликация "использовать конспект => у игрока есть конспект"
    // должна быть истинной
    assert!(
        !use_lecture_notes
            || state.player.status_for_subject(subject).has_lecture_notes(),
        "Нельзя воспользоваться конспектом, так как его на самом деле нет"
    );
    let brain_or_stamina = if subject == Subject::PhysicalEducation {
        state.player.stamina.0
    } else {
        state.player.brain.0
    };
    if brain_or_stamina <= 0 {
        return scene_router_run(game, &state);
    }
    let health = state.player.health;
    let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
    *knowledge += if state.current_time.is_optimal_study_time() {
        brain_or_stamina
    } else {
        brain_or_stamina * 2 / 3
    };
    *knowledge -= game.rng.random(brain_or_stamina / 2);
    *knowledge += game.rng.random(health.0 / 18);
    if use_lecture_notes {
        *knowledge += 10
    }
    assert!(*knowledge >= BrainLevel(0));
    assert!(state.player.stamina >= StaminaLevel(0));
    let mut health_penalty = 10 - game.rng.random(state.player.stamina.0);
    if health_penalty < 0 || use_lecture_notes {
        health_penalty = 0;
    }
    if state.current_time.is_suboptimal_study_time() {
        health_penalty += 12;
    }

    game.decrease_health(
        HealthLevel(health_penalty),
        state,
        CauseOfDeath::Overstudied,
        |game, state| {
            if state
                .player
                .status_for_subject(subject)
                .knowledge
                .is_lethal()
            {
                game.decrease_health(
                    HealthLevel(10),
                    state.clone(),
                    CauseOfDeath::StudiedTooWell,
                    |game, state| game.hour_pass(state.clone()),
                )
            } else {
                game.hour_pass(state.clone())
            }
        },
    )
}

pub(in crate::logic) fn choose_use_lecture_notes(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    match action {
        Action::DoStudy {
            subject,
            lecture_notes_available,
        } => {
            assert_eq!(
                state.player.status_for_subject(subject).has_lecture_notes(),
                lecture_notes_available
            );
            if lecture_notes_available {
                game.set_screen(GameScreen::PromptUseLectureNotes(state));
                ActionVec::from([
                    Action::UseLectureNotes(subject),
                    Action::DontUseLectureNotes(subject),
                ])
            } else {
                study(game, state, subject, false)
            }
        }
        Action::DontStudy => scene_router_run(game, &state),
        _ => illegal_action!(action),
    }
}

#[deprecated]
fn rest(game: &mut InternalGameState, mut state: GameState) -> ActionVec {
    state.player.health += game.rng.random_in_range(7..15);
    game.hour_pass(state)
}

#[deprecated]
pub(in crate::logic) fn try_to_sleep(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::Dorm);
    if state.current_time > Time(3) && state.current_time < Time(20) {
        game.set_screen(GameScreen::Sleep(state));
        wait_for_any_key()
    } else {
        go_to_sleep(game, state)
    }
}

#[deprecated]
pub(in crate::logic) fn go_to_sleep(
    _game: &mut InternalGameState,
    _state: GameState,
) -> ActionVec {
    todo!()
}

#[deprecated]
pub(in crate::logic) fn handle_sleeping(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    // TODO: Реализовать что-то помимо неудавшегося сна
    assert_matches!(&*game.screen(), GameScreen::Sleep(_));
    assert_eq!(action, Action::AnyKey);
    scene_router_run(game, &state)
}

#[deprecated]
pub(in crate::logic) fn go_to_pdmi(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    assert_ne!(state.location, Location::PDMI);
    if state.current_time > Time(20) {
        game.set_screen(GameScreen::TrainToPDMI(state, NoPointToGoToPDMI));
        return wait_for_any_key();
    }

    let health_penalty = HealthLevel(game.rng.random(10));
    game.decrease_health(
        health_penalty,
        state,
        CauseOfDeath::CorpseFoundInTheTrain,
        |game, state| {
            state.location = Location::PDMI;
            if state.player.money < Money::roundtrip_train_ticket_cost() {
                no_money(game, state.clone())
            } else {
                game.set_screen(GameScreen::TrainToPDMI(
                    state.clone(),
                    PromptToBuyTickets,
                ));
                ActionVec::from([Action::GatecrashTrain, Action::BuyRoundtripTrainTicket])
            }
        },
    )
}

#[deprecated]
fn no_money(game: &mut InternalGameState, mut state: GameState) -> ActionVec {
    let caught_by_inspectors = train::inspectors(&mut game.rng, &state);

    let gatecrash_because_no_money = |game: &mut InternalGameState, state: GameState| {
        game.set_screen(GameScreen::TrainToPDMI(
            state,
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            },
        ));
        wait_for_any_key()
    };

    let health_penalty = HealthLevel(10);
    if caught_by_inspectors {
        if state.location != Location::Dorm {
            return game.decrease_health(
                health_penalty,
                state,
                CauseOfDeath::KilledByInspectors,
                |g, state| gatecrash_because_no_money(g, state.clone()),
            );
        }
        // При попытке поехать в ПОМИ из общежития здоровье уменьшается, но смерть
        // не наступает, даже если здоровье стало отрицательным.
        // Баг в оригинальной реализации. Возможно, стоит исправить, но пока не буду.
        state.player.health -= health_penalty;
    }

    gatecrash_because_no_money(game, state)
}

#[deprecated]
pub(in crate::logic) fn proceed_with_train(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: TrainToPDMI,
) -> ActionVec {
    match action {
        Action::AnyKey => match interaction {
            NoPointToGoToPDMI => scene_router_run(game, &state),
            GatecrashBecauseNoMoney {
                caught_by_inspectors,
            }
            | GatecrashByChoice {
                caught_by_inspectors,
            } => {
                if caught_by_inspectors {
                    todo!("Если поймали контролёры, должно пройти два часа!")
                }
                game.hour_pass(state)
            }
            PromptToBuyTickets => illegal_action!(action),
            BoughtRoundtripTicket => {
                state.player.money -= Money::roundtrip_train_ticket_cost();
                state.player.set_has_roundtrip_train_ticket();
                game.hour_pass(state)
            }
        },
        Action::GatecrashTrain => {
            assert_eq!(interaction, PromptToBuyTickets);
            let caught_by_inspectors = train::inspectors(&mut game.rng, &state);
            game.set_screen(GameScreen::TrainToPDMI(
                state,
                GatecrashByChoice {
                    caught_by_inspectors,
                },
            ));
            wait_for_any_key()
        }
        Action::BuyRoundtripTrainTicket => {
            assert_eq!(interaction, PromptToBuyTickets);
            game.set_screen(GameScreen::TrainToPDMI(state, BoughtRoundtripTicket));
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn go_to_professor(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    let mut available_actions = state
        .current_day()
        .current_exams(state.location, state.current_time)
        .map(|exam| Action::Exam(exam.subject()))
        .collect::<ActionVec>();
    available_actions.push(Action::DontGoToProfessor);
    game.set_screen(GameScreen::GoToProfessor(state));
    available_actions
}

#[deprecated]
pub(in crate::logic) fn handle_cafe_punk_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    // TODO: Логику можно переиспользовать в кафе ПОМИ
    assert_eq!(state.location, Location::PUNK);
    assert!(state.current_time.is_cafe_open());
    assert_matches!(&*game.screen(), GameScreen::CafePUNK(_));
    let money = &mut state.player.money;
    let health = &mut state.player.health;
    let charisma_dependent_health_gain =
        HealthLevel(game.rng.random(state.player.charisma.0));
    match action {
        Action::OrderTea => {
            *money -= Money::tea_cost();
            *health += charisma_dependent_health_gain + 2;
        }
        Action::OrderCake => {
            *money -= Money::cake_cost();
            *health += charisma_dependent_health_gain + 4;
        }
        Action::OrderTeaWithCake => {
            *money -= Money::tea_with_cake_cost();
            *health += charisma_dependent_health_gain + 7;
        }
        Action::RestInCafePUNK => {
            *health += charisma_dependent_health_gain;
        }
        Action::ShouldntHaveComeToCafePUNK => {
            return scene_router_run(game, &state);
        }
        _ => illegal_action!(action),
    }
    game.hour_pass(state)
}

#[deprecated]
pub(in crate::logic) fn interact_with_classmate(
    game: &mut InternalGameState,
    state: GameState,
    classmate: Classmate,
) -> ActionVec {
    use Classmate::*;
    match classmate {
        Kolya => interact_with_kolya(game, state),
        Pasha => interact_with_pasha(game, state),
        Diamond => todo!(),
        RAI => todo!(),
        Misha => todo!(),
        Serj => todo!(),
        Sasha => interact_with_sasha(game, state),
        NiL => todo!(),
        Kuzmenko => interact_with_kuzmenko(game, state),
        DJuG => todo!(),
        Andrew => todo!(),
        Grisha => grisha::interact(game, state),
    }
}

#[deprecated]
pub(in crate::logic) fn interact_with_pasha(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    use crate::logic::pasha::PashaInteraction::{Inspiration, Stipend};
    assert_eq!(state.location, Location::PUNK);
    let interaction = if state.player.got_stipend() {
        Inspiration
    } else {
        Stipend
    };
    game.set_screen(GameScreen::PashaInteraction(state, interaction));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn proceed_with_pasha(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: PashaInteraction,
) -> ActionVec {
    assert_eq!(action, Action::AnyKey);
    assert_eq!(state.location, Location::PUNK);
    assert_matches!(&*game.screen(), GameScreen::PashaInteraction(_, _));
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
    scene_router_run(game, &state)
}

#[deprecated]
pub(in crate::logic) fn handle_punk_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::PUNK);
    match action {
        Action::GoToProfessor => go_to_professor(game, state),
        Action::LookAtBaobab => {
            game.set_screen(GameScreen::HighScores(state));
            wait_for_any_key()
        }
        Action::GoFromPunkToDorm => {
            state.location = Location::Dorm;
            scene_router_run(game, &state)
        }
        Action::GoToPDMI => go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToComputerClass => {
            assert!(state.current_time < Time::computer_class_closing());
            state.location = Location::ComputerClass;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::FellFromStairs,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToCafePUNK => {
            // TODO: Логику можно переиспользовать в кафе ПОМИ
            assert!(state.current_time.is_cafe_open());
            let mut available_actions = ActionVec::new();
            let available_money = state.player.money;
            if available_money >= Money::tea_cost() {
                available_actions.push(Action::OrderTea);
            }
            if available_money >= Money::cake_cost() {
                available_actions.push(Action::OrderCake);
            }
            if available_money >= Money::tea_with_cake_cost() {
                available_actions.push(Action::OrderTeaWithCake);
            }
            available_actions.push(Action::RestInCafePUNK);
            available_actions.push(Action::ShouldntHaveComeToCafePUNK);
            game.set_screen(GameScreen::CafePUNK(state));
            available_actions
        }
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PUNK)
            );
            interact_with_classmate(game, state, classmate)
        }
        Action::GoToWork => {
            assert!(state.player.is_employed_at_terkom());
            todo!()
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_pdmi_action(
    game: &mut InternalGameState,
    state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::PDMI);
    match action {
        Action::GoToProfessor => go_to_professor(game, state),
        Action::LookAtBulletinBoard => {
            game.set_screen(GameScreen::HighScores(state));
            wait_for_any_key()
        }
        Action::RestInCafePDMI => todo!("Пойти в кафе"),
        Action::GoToPUNKFromPDMI => todo!("Поехать в ПУНК"),
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::PDMI)
            );
            interact_with_classmate(game, state, classmate)
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_computer_class_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::ComputerClass);
    match action {
        Action::Exam(Subject::ComputerScience) => todo!(),
        Action::GoFromPunkToDorm => {
            state.location = Location::Dorm;
            scene_router_run(game, &state)
        }
        Action::LeaveComputerClass => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::CouldntLeaveTheComputer,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToPDMI => go_to_pdmi(game, state),
        Action::GoToMausoleum => {
            state.location = Location::Mausoleum;
            game.decrease_health(
                HealthLevel::location_change_small_penalty(),
                state,
                CauseOfDeath::OnTheWayToMausoleum,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::SurfInternet => surf_internet(game, state),
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::ComputerClass)
            );
            interact_with_classmate(game, state, classmate)
        }
        Action::PlayMMHEROES => todo!(),
        Action::IAmDone => game_end(game, state),
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn surf_internet(
    game: &mut InternalGameState,
    state: GameState,
) -> ActionVec {
    let player = &state.player;
    let cs_problems_done = player
        .status_for_subject(Subject::ComputerScience)
        .problems_done();
    let cs_problems_required = SUBJECTS[Subject::ComputerScience].required_problems;
    let found_program = player.is_god_mode()
        || (game.rng.random(player.brain) > BrainLevel(6)
            && cs_problems_done < cs_problems_required);
    game.set_screen(GameScreen::SurfInternet(state, found_program));
    wait_for_any_key()
}

#[deprecated]
pub(in crate::logic) fn proceed_with_internet(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    found_program: bool,
) -> ActionVec {
    assert_eq!(action, Action::AnyKey);
    if found_program {
        state
            .player
            .status_for_subject_mut(Subject::ComputerScience)
            .more_problems_solved(1);
    } else if state.player.brain < BrainLevel(5) && game.rng.roll_dice(3) {
        state.player.brain += 1;
    }
    game.hour_pass(state)
}

#[deprecated]
pub(in crate::logic) fn handle_mausoleum_action(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    match action {
        Action::GoFromMausoleumToPunk => {
            state.location = Location::PUNK;
            game.decrease_health(
                HealthLevel::location_change_large_penalty(),
                state,
                CauseOfDeath::OnTheWayToPUNK,
                |g, state| scene_router_run(g, state),
            )
        }
        Action::GoToPDMI => go_to_pdmi(game, state),
        Action::GoFromMausoleumToDorm => {
            state.location = Location::Dorm;
            scene_router_run(game, &state)
        }
        Action::Rest => {
            let money = state.player.money;
            game.set_screen(GameScreen::RestInMausoleum(state));
            let mut available_actions = ActionVec::new();
            if money >= Money::cola_cost() {
                available_actions.push(Action::OrderCola);
            }
            if money >= Money::soup_cost() {
                available_actions.push(Action::OrderSoup);
            }
            if money >= Money::beer_cost() {
                available_actions.push(Action::OrderBeer);
            }
            available_actions.push(Action::RestByOurselvesInMausoleum);
            available_actions.push(Action::NoRestIsNoGood);
            available_actions
        }
        Action::InteractWithClassmate(classmate) => {
            assert_matches!(
                state.classmates[classmate].current_location(),
                ClassmateLocation::Location(Location::Mausoleum)
            );
            interact_with_classmate(game, state, classmate)
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn handle_mausoleum_rest(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
) -> ActionVec {
    let player = &mut state.player;
    match action {
        Action::OrderCola => {
            assert!(player.money >= Money::cola_cost());
            player.money -= Money::cola_cost();
            player.health += game.rng.random(player.charisma.0) + 3;
        }
        Action::OrderSoup => {
            assert!(player.money >= Money::soup_cost());
            player.money -= Money::soup_cost();
            player.health += game.rng.random(player.charisma.0) + 5;
        }
        Action::OrderBeer => {
            assert!(player.money >= Money::beer_cost());
            player.money -= Money::beer_cost();
            if game.rng.roll_dice(3) {
                player.brain -= 1;
            }
            if game.rng.roll_dice(3) {
                player.charisma += 1;
            }
            if game.rng.roll_dice(3) {
                player.stamina += 2;
            }
            player.health += game.rng.random(player.charisma.0);
            if player.brain <= BrainLevel(0) {
                player.health = HealthLevel(0);
                player.cause_of_death = Some(CauseOfDeath::BeerAlcoholism);
                return game_end(game, state);
            }
        }
        Action::RestByOurselvesInMausoleum => {
            player.health += game.rng.random(player.charisma.0);
        }
        Action::NoRestIsNoGood => return scene_router_run(game, &state),
        _ => illegal_action!(action),
    }

    game.hour_pass(state)
}

#[deprecated]
pub(in crate::logic) fn interact_with_kolya(
    game: &mut InternalGameState,
    mut state: GameState,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    let player = &mut state.player;
    let (available_actions, interaction) = if let Some(available_actions) =
        kolya_maybe_solve_algebra_problems(&mut game.rng, player)
    {
        (available_actions, SolvedAlgebraProblemsForFree)
    } else if player.money < Money::oat_tincture_cost() {
        // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        (wait_for_any_key(), BrakeFluidNoMoney)
    } else {
        // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
        // "Заказать Коле настойку овса?"
        (
            ActionVec::from([Action::Yes, Action::No]),
            PromptOatTincture,
        )
    };
    game.set_screen(GameScreen::KolyaInteraction(state, interaction));
    available_actions
}

#[deprecated]
pub(in crate::logic) fn proceed_with_kolya(
    game: &mut InternalGameState,
    mut state: GameState,
    action: Action,
    interaction: KolyaInteraction,
) -> ActionVec {
    assert_eq!(state.location, Location::Mausoleum);
    assert_matches!(&*game.screen(), GameScreen::KolyaInteraction(_, _));
    let player = &mut state.player;
    match action {
        Action::AnyKey => {
            let algebra_status =
                player.status_for_subject_mut(Subject::AlgebraAndNumberTheory);
            match interaction {
                SolvedAlgebraProblemsForFree => {
                    algebra_status.more_problems_solved(2);
                    return game.hour_pass(state);
                }
                PromptOatTincture => unreachable!(),
                SolvedAlgebraProblemsForOatTincture => {
                    algebra_status.more_problems_solved(2);
                    player.money -= Money::oat_tincture_cost();
                    return game.hour_pass(state);
                }
                BrakeFluidNoMoney => {
                    player.brain -= 1;
                    if player.brain <= BrainLevel(0) {
                        player.health = HealthLevel(0);
                        player.cause_of_death = Some(CauseOfDeath::DrankTooMuch);
                        return game_end(game, state);
                    }
                }
                BrakeFluidBecauseRefused => {
                    state.player.brain -= 1;
                    // Забавно, что в этой ветке можно бесконечно пить тормозную
                    // жидкость и никогда не спиться. Баг в оригинальной реализации.
                }
                Altruism => {
                    player.money -= Money::oat_tincture_cost();
                }
            }
            scene_router_run(game, &state)
        }
        Action::Yes => {
            assert_eq!(interaction, PromptOatTincture);
            if let Some(num_actions) =
                kolya_maybe_solve_algebra_problems(&mut game.rng, player)
            {
                // "Коля решил тебе ещё 2 задачи по алгебре!"
                game.set_screen(GameScreen::KolyaInteraction(
                    state,
                    SolvedAlgebraProblemsForOatTincture,
                ));
                num_actions
            } else {
                // "Твой альтруизм навсегда останется в памяти потомков."
                game.set_screen(GameScreen::KolyaInteraction(state, Altruism));
                wait_for_any_key()
            }
        }
        Action::No => {
            assert_eq!(interaction, PromptOatTincture);
            // "Зря, ой, зря ..."
            // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
            game.set_screen(GameScreen::KolyaInteraction(
                state,
                BrakeFluidBecauseRefused,
            ));
            wait_for_any_key()
        }
        _ => illegal_action!(action),
    }
}

/// Возвращает `Some`, если Коля может помочь решить задачи по алгебре,
/// иначе — `None`.
#[deprecated]
fn kolya_maybe_solve_algebra_problems(
    rng: &mut Rng,
    player: &mut Player,
) -> Option<ActionVec> {
    use crate::logic::Subject::AlgebraAndNumberTheory;
    let has_enough_charisma = player.charisma > rng.random(CharismaLevel(10));
    let algebra = player.status_for_subject(AlgebraAndNumberTheory);
    let problems_done = algebra.problems_done();
    let required_problems = SUBJECTS[AlgebraAndNumberTheory].required_problems;
    let has_at_least_2_remaining_problems = problems_done + 2 <= required_problems;
    if has_enough_charisma && has_at_least_2_remaining_problems {
        Some(wait_for_any_key())
    } else {
        None
    }
}

#[deprecated]
pub(in crate::logic) fn interact_with_sasha(
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

#[deprecated]
pub(in crate::logic) fn proceed_with_sasha(
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
            scene_router_run(game, &state)
        }
        _ => illegal_action!(action),
    }
}

#[deprecated]
pub(in crate::logic) fn interact_with_kuzmenko(
    game: &mut InternalGameState,
    mut state: GameState,
) -> ActionVec {
    let tomorrow = state.current_day_index + 1;
    let saturday = 5;
    let mut additional_exam_day_idx: Option<u8> = None;
    if tomorrow <= saturday {
        for i in (tomorrow..=saturday).rev() {
            let day = state.timetable.day_mut(i);
            let has_enough_charisma =
                state.player.charisma > game.rng.random(CharismaLevel(18));
            let can_add_exam = day.exam(Subject::ComputerScience).is_none();
            if has_enough_charisma && can_add_exam {
                let exam_start_time = Time(10u8 + game.rng.random(5));
                let exam_end_time = exam_start_time + Duration(1i8 + game.rng.random(2));
                let additional_exam = timetable::Exam::new(
                    Subject::ComputerScience,
                    exam_start_time,
                    exam_end_time,
                    Location::ComputerClass,
                );
                day.add_exam(additional_exam);
                additional_exam_day_idx = Some(i);
                break;
            }
        }
    }

    let new_screen = match additional_exam_day_idx {
        // Проверка на `state.additional_computer_science_exams() < 2` должна быть
        // раньше — до модификации расписания.
        // Иначе получается, что при достаточной харизме Кузьменко может
        // добавить экзамены по информатике на каждый день.
        // Баг в оригинальной реализации. Возможно, стоит исправить, но
        // пока не буду.
        Some(additional_exam_day_idx)
            if state.additional_computer_science_exams() < 2 =>
        {
            state.add_additional_computer_science_exam();
            GameScreen::KuzmenkoInteraction(
                state,
                AdditionalComputerScienceExam {
                    day_index: additional_exam_day_idx,
                },
            )
        }
        _ => {
            let replies = [
                FormatFloppy,
                FiltersInWindows,
                ByteVisualization,
                OlegPliss,
                BillGatesMustDie,
                MonitorJournal,
                MmheroesBP7,
                CSeminar,
                ThirdYear,
                STAR,
                GetYourselvesAnEmail,
                TerekhovSenior,
            ];
            GameScreen::KuzmenkoInteraction(state, *game.rng.random_element(&replies))
        }
    };
    game.set_screen(new_screen);

    wait_for_any_key()
}
