use super::*;
use crate::logic::scene_router::train::BaltiyskiyRailwayStationScene;
use crate::util::bitset::BitSet;
use crate::util::TinyVec;
use actions::{
    BaltiyskiyRailwayStationAction, ContinueSufferingWithExamInTrainAction,
    NpcApproachAction,
};
use core::cmp::{max, min};
use strum::VariantArray;

pub(super) async fn go_to_professor(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    let mut available_actions = state
        .current_day()
        .current_exams(state.location(), state.current_time())
        .map(|exam| Action::Exam(exam.subject()))
        .collect::<ActionVec>();
    available_actions.push(Action::DontGoToProfessor);
    g.set_screen_and_action_vec(
        GameScreen::GoToProfessor(state.clone()),
        available_actions,
    );
    let subject = match g.wait_for_action().await {
        Action::Exam(subject) => subject,
        Action::DontGoToProfessor => return,
        action => illegal_action!(action),
    };
    enter_exam(g, state, subject).await;
}

pub(super) async fn enter_exam(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) {
    if g.rng.roll_dice(2) {
        exam_intro(g, state, subject).await;
    }
    state.player.set_last_exam(subject);
    assert!(
        state.player.health > HealthLevel(0) && state.player().cause_of_death().is_none(),
    );
    exam(g, state, subject).await;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExamIntro {
    /// Болшая, рассчитанная на поток аудитория кажется забитой народом.
    /// Здесь присутствуют не только твои одногруппники,
    /// но и какие-то не очень знакомые тебе люди
    /// (кажется, прикладники со второго курса).
    /// За столом около доски сидит М. А. Всемирнов
    /// и принимает зачет у студентов.
    /// Ты решаешь не терять времени даром и присоединиться к остальным.
    AlgebraPunkBigCrowdedRoom,

    /// Ты заходишь в небольшую аудиторию, забитую народом.
    /// Около доски сидит весьма своеобразный преподаватель.
    /// Сие своебразие проявляется, в первую очередь, значком
    /// с надписью: "НЕ СТРЕЛЯЕЙТЕ В ПРЕПОДА - ОБУЧАЕТ КАК УМЕЕТ".
    /// "А вы к кому? Максим Александрович в аудитории напротив!"
    /// Похоже, ты не туда попал. Ты извиняешься и идешь к Всемирнову.
    AlgebraPunkWrongRoom,

    /// Маленький кабинет в ПОМИ заполнен людьми.
    /// И, как ни странно, почти все они хотят одного и того же.
    /// Похоже, ты тоже хочешь именно этого -
    /// РАЗДЕЛАТЬСЯ НАКОНЕЦ С ЗАЧЕТОМ ПО АЛГЕБРЕ!
    AlgebraPdmi,

    /// В обычной "групповой" аудитории сидят около 15 человек.
    /// В центре их внимания находится Е.С. Дубцов,
    /// принимающий зачет по матанализу.
    /// Ты получаешь задание и садишься за свободную парту.
    Calculus,

    /// Небольшая, полупустая аудитория.
    /// И доска, и стены, и, похоже, даже пол
    /// исписаны различными геометрическими утверждениями.
    /// В центре всего этого хаоса находится
    /// (или, скорее, постоянно перемещается)
    /// Подкорытов-младший.
    /// Ты радуешься, что смог застать его на факультете!
    GeometryPunk,

    /// В небольшом ПОМИшном кабинете собралось человек 10 студентов.
    /// Кроме них, в комнате ты видишь Подкорытова-младшего,
    /// а также - полного седоволосого лысеющего господина,
    /// издающего характерные пыхтящие звуки.
    /// Ты надеешься, что все это скоро кончится...
    GeometryPdmi,

    /// Климов А.А. сидит и тоскует по халявному Inet'у.
    ComputerScience,

    /// На третьем этаже учебного корпуса Мат-Меха
    /// в одной из аудиторий, закрепленных за кафедрой иностранных языков,
    /// расположилась Н.П. Влащенко.
    /// Стены кабинета выглядят как-то странно.
    /// Рядом с небольшой доской висит изображение Эйфелевой башни,
    /// чуть дальше - странное изображение,
    /// обладающее непостижимым метафизическим смыслом.
    /// Похоже, сейчас ты будешь сдавать зачет по английскому.
    English,

    /// Альбинский проводит лекцию о пользе бега для <…>
    /// Похоже, он, как всегда, немного увлекся.
    /// Немного в нашем случае - 1 час.
    /// Альбинский просит тебя замерить пульс.
    /// Назвав первое пришедшее в замученную математикой голову число,
    /// ты отправляешься мотать круги в парке,
    /// в котором, вообще-то, "запрещены спортивные мероприятия".
    PhysicalEducation(Option<BenefitsOfRunning>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, VariantArray)]
pub enum BenefitsOfRunning {
    /// …для народного хозяйства.
    NationalEconomy,

    /// …для личной жизни.
    PersonalLife,

    /// …для научной работы.
    ScientificResearch,

    /// …для коммунистического строительства.
    BuildingCommunism,

    /// …для учебы и досуга.
    StudyAndEntertainment,

    /// …для спасения от контроллеров.
    EscapingFromInspectors,
}

async fn show_intro(g: &mut InternalGameState<'_>, exam_intro: ExamIntro) {
    g.set_screen_and_wait_for_any_key(GameScreen::ExamIntro(exam_intro))
        .await
}

async fn exam_intro(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) {
    match subject {
        Subject::AlgebraAndNumberTheory => match state.location() {
            Location::PUNK => {
                let intro = if g.rng.roll_dice(3) {
                    ExamIntro::AlgebraPunkWrongRoom
                } else {
                    ExamIntro::AlgebraPunkBigCrowdedRoom
                };
                show_intro(g, intro).await;
            }
            Location::PDMI => show_intro(g, ExamIntro::AlgebraPdmi).await,
            _ => unreachable!("invalid location"),
        },
        Subject::Calculus => show_intro(g, ExamIntro::Calculus).await,
        Subject::GeometryAndTopology => match state.location() {
            Location::PUNK => {
                show_intro(g, ExamIntro::GeometryPunk).await;
                state.player.health += 5;
            }
            Location::PDMI => {
                show_intro(g, ExamIntro::GeometryPdmi).await;
            }
            _ => unreachable!("invalid location"),
        },
        Subject::ComputerScience => show_intro(g, ExamIntro::ComputerScience).await,
        Subject::English => show_intro(g, ExamIntro::English).await,
        Subject::PhysicalEducation => {
            if g.rng.roll_dice(3) {
                let lecture_topic = g.rng.random_variant();
                show_intro(g, ExamIntro::PhysicalEducation(Some(lecture_topic))).await;
                state
                    .current_day_mut()
                    .exam_mut(Subject::PhysicalEducation)
                    .unwrap()
                    .one_hour_more();
                misc::hour_pass(g, state, Some(subject)).await;
            } else {
                show_intro(g, ExamIntro::PhysicalEducation(None)).await;
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum ExamResult {
    Continue,
    Exit,
}

#[derive(Debug, Clone)]
pub enum ExamScene {
    /// Экран выбора действий во время зачёта
    Router(GameState, Subject),

    /// Попытка сдать зачёт
    ExamSuffering {
        solved_problems: u8,
        too_smart: bool,
    },

    /// Во время сдачи зачёта пристаёт NPC
    ClassmateWantsSomething(GameState, Subject, Classmate),

    /// Выбрали игнорировать NPC.
    IgnoredClassmate { feeling_bad: bool },

    /// Преподаватель уходит.
    ProfessorLeaves(GameState, Subject),

    /// Предложение пойти за преподавателем сдавать зачёт в электричке.
    PromptExamInTrain(GameState, Subject),

    /// Преподаватель согласился принимать зачёт в электричке
    Train(GameState, train::TrainScene),

    /// Воспроизводим баг оригинальной реализации
    CaughtByInspectorsEmptyScreenBug,

    /// Мучаемся, сдавая зачёт в электричке
    SufferInTrain {
        state: GameState,
        solved_problems: u8,
    },

    /// Преподаватель задерживается ещё на час
    ProfessorLingers(GameState, Subject),

    /// "Всемирнов медленно рисует минус ...
    /// И так же медленно пририсовывает к нему вертикальную палочку!
    /// Уф! Ну и шуточки у него!
    /// Хорошо хоть, зачет поставил..."
    AlgebraExamPassed(GameState),

    /// "Закройте глаза ..."
    EnglishExamPassed(GameState, EnglishExamFeeling),

    /// "Твоя зачетка пополнилась еще одной записью."
    ExamPassed(GameState, Subject),
}

async fn exam(g: &mut InternalGameState<'_>, state: &mut GameState, subject: Subject) {
    loop {
        let day_index = state.current_day_index();
        let status = state.player.status_for_subject_mut(subject);
        if status.solved_all_problems() && !status.passed() {
            status.set_passed_exam_day_index(day_index);
            if exam_passed(g, state, subject).await == ExamResult::Exit
                || state.player().cause_of_death().is_some()
            {
                return;
            }
        }
        if state.current_time() >= state.current_day().exam(subject).unwrap().to()
            && exam_ends(g, state, subject).await == ExamResult::Exit
        {
            return;
        }
        if npc_try_approach(g, state, subject).await == ExamResult::Exit {
            return;
        }

        let mut available_actions = ActionVec::new();
        if !state.player.status_for_subject(subject).passed() {
            available_actions.push(Action::SufferMore);
        }
        available_actions.extend(state.classmates.filter_by_exam(subject).map(
            |classmate_info| Action::InteractWithClassmate(classmate_info.classmate()),
        ));
        available_actions.push(Action::ExitExam);

        // Убеждаемся, что DJuG всегда присутствует на зачёте по геометрии в ПОМИ
        // и нигде больше.
        assert!(
            !(state.location() == Location::PDMI
                && subject == Subject::GeometryAndTopology)
                || available_actions
                    .contains(&Action::InteractWithClassmate(Classmate::DJuG)),
            "DJuG не на своём месте",
        );
        g.set_screen_and_action_vec(
            GameScreen::Exam(ExamScene::Router(state.clone(), subject)),
            available_actions,
        );
        match g.wait_for_action().await {
            Action::SufferMore => {
                suffer_exam(g, state, false, subject).await;
            }
            Action::InteractWithClassmate(classmate) => {
                interact_with_classmate(g, state, classmate, Some(subject)).await;
            }
            Action::ExitExam => {
                return;
            }
            action => illegal_action!(action),
        }
    }
}

/// С некоторой вероятностью во время сдачи зачёта к игроку могут пристать NPC.
async fn npc_try_approach(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) -> ExamResult {
    let mut approached_classmates = BitSet::new();
    let garlic = state.player.garlic;
    loop {
        let times_approached = approached_classmates.count() as i16;
        if state.player.charisma.0 / 2 <= times_approached || times_approached > 3 {
            break;
        }
        for &classmate in Classmate::VARIANTS {
            if approached_classmates.contains(classmate) {
                // NPC не пристаёт более одного раза подряд.
                continue;
            }
            if classmate.annoyance() - times_approached / 2 - garlic <= g.rng.random(10) {
                continue;
            }
            if !state.classmates[classmate].is_at_exam(subject) {
                continue;
            }
            if state.player.charisma.0 / 2 > times_approached {
                approached_classmates.add(classmate);
                classmate_wants_something(g, state, subject, classmate).await;

                if state.current_time() >= state.current_day().exam(subject).unwrap().to()
                    && exam_ends(g, state, subject).await == ExamResult::Exit
                    || state.player().cause_of_death().is_some()
                {
                    return ExamResult::Exit;
                }
            }
        }

        if g.rng.roll_dice(2) {
            break;
        }
    }
    ExamResult::Continue
}

pub(in crate::logic) fn number_of_problems_accepted(
    rng: &mut random::Rng,
    state: &GameState,
    subject: Subject,
    in_train: bool,
) -> u8 {
    let mut mental_capacity = state.player.status_for_subject(subject).knowledge
        + rng.random(state.player.brain)
        - subject.mental_load();

    if in_train {
        mental_capacity = BrainLevel(mental_capacity.0 * 3 / 4);
    }

    mental_capacity -= rng.random(BrainLevel(max(5 - state.player.health.0, 0)));

    let accepted_problems = if mental_capacity > 0 {
        ((mental_capacity.0 as f32).sqrt() / subject.single_problem_mental_factor())
            .round() as u8
    } else {
        0
    };

    min(
        state
            .player
            .status_for_subject(subject)
            .problems_remaining(),
        accepted_problems,
    )
}

async fn suffer_exam(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    in_train: bool,
    subject: Subject,
) {
    let charisma = state.player.charisma;
    let mut solved_problems =
        number_of_problems_accepted(&mut g.rng, state, subject, in_train);

    let knowledge_penalty = g.rng.random(subject.mental_load())
        - g.rng.random(BrainLevel(state.player.stamina.0));
    let knowledge = &mut state.player.status_for_subject_mut(subject).knowledge;
    *knowledge -= knowledge_penalty.clamp(BrainLevel(0), *knowledge);

    let too_smart =
        subject == Subject::GeometryAndTopology && charisma.0 * 2 + 26 < knowledge.0;
    if too_smart {
        solved_problems = 0;
    }

    let scene = if in_train {
        ExamScene::SufferInTrain {
            state: state.clone(),
            solved_problems,
        }
    } else {
        ExamScene::ExamSuffering {
            solved_problems,
            too_smart,
        }
    };

    g.set_screen_and_wait_for_any_key(GameScreen::Exam(scene))
        .await;

    state
        .player
        .status_for_subject_mut(subject)
        .more_problems_solved(solved_problems);

    let stamina = if in_train {
        state.player.stamina.0 * 2 / 3
    } else {
        state.player.stamina.0
    };

    let health_penalty = max(
        subject.health_penalty() - g.rng.random(HealthLevel(stamina)),
        HealthLevel(0),
    );
    misc::decrease_health(
        state,
        health_penalty,
        CauseOfDeath::TorturedByProfessor(subject),
    );
    if !in_train || state.player.health > 0 {
        // Баг в оригинальной реализации: если умер в поезде, то экран смерти
        // не показывается и час не проходит.
        misc::hour_pass(g, state, Some(subject)).await;
    }
}

#[derive(Debug, Copy, Clone, VariantArray)]
pub enum EnglishExamFeeling {
    /// "Тебе сильно поплохело.
    /// Фея была явно не в настроении."
    ReallyBad,

    /// "Ты почувствовал себя где-то в другом месте."
    SomeplaceElse,

    /// "Ты чувствуешь, что подзабыл алгебру..."
    ForgotAlgebra,

    /// "Ты чувствуешь, что анализ придется учить заново."
    ForgotCalculus,

    /// "В голову постоянно лезут мысли о всяких феях..."
    ThoughtsAboutFairies,

    /// "Ты чувствуешь, что все вокруг жаждут твоей смерти."
    EveryoneWantsYouDead,

    /// "Куда-то подевалась твоя уверенность в себе."
    StaminaGone,

    /// "Голова стала работать заметно лучше."
    BrainGotBetter,

    /// "Ты проникся любовью к окружающему миру.",
    LoveForTheWorld,

    /// "Ты готов к любым испытаниям."
    ReadyForEverything,

    /// "Пока твои глаза были закрыты, кто-то утащил твои деньги!!!"
    /// или
    /// "Ты нашел в своем кармане какие-то деньги!"
    Money,

    /// "Ты чувствуешь, что от тебя сильно несет чесноком.
    /// Не знаю, выветрится ли такой сильный запах..."
    SmellOfGarlic,

    /// "Странное чувство быстро прошло."
    QuicklyFadedAway,
}

async fn exam_passed(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) -> ExamResult {
    match subject {
        Subject::AlgebraAndNumberTheory => {
            g.set_screen_and_wait_for_any_key(GameScreen::Exam(
                ExamScene::AlgebraExamPassed(state.clone()),
            ))
            .await;
            misc::decrease_health(
                state,
                HealthLevel(g.rng.random(6)),
                CauseOfDeath::DestroyedByVsemirnov,
            );
        }
        Subject::English => {
            const FEELING_COUNT: usize = EnglishExamFeeling::VARIANTS.len() + 2;
            let mut possible_feelings =
                TinyVec::<_, FEELING_COUNT>::from(EnglishExamFeeling::VARIANTS);

            // Запах чеснока чуть более вероятен
            possible_feelings
                .extend(core::iter::repeat(EnglishExamFeeling::SmellOfGarlic).take(2));

            let feeling = *g.rng.random_element(&possible_feelings);
            g.set_screen_and_wait_for_any_key(GameScreen::Exam(
                ExamScene::EnglishExamPassed(state.clone(), feeling),
            ))
            .await;
            match feeling {
                EnglishExamFeeling::ReallyBad => {
                    misc::decrease_health(
                        state,
                        HealthLevel(30),
                        CauseOfDeath::FairyWasNotInTheMood,
                    );
                }
                EnglishExamFeeling::SomeplaceElse => {
                    state.set_location(Location::PDMI);
                    return ExamResult::Exit;
                }
                EnglishExamFeeling::ForgotAlgebra => {
                    state
                        .player
                        .status_for_subject_mut(Subject::AlgebraAndNumberTheory)
                        .knowledge /= 2;
                }
                EnglishExamFeeling::ForgotCalculus => {
                    state
                        .player
                        .status_for_subject_mut(Subject::Calculus)
                        .knowledge /= 2;
                }
                EnglishExamFeeling::ThoughtsAboutFairies => {
                    state.player.brain -= g.rng.random_in_range(1..3);
                }
                EnglishExamFeeling::EveryoneWantsYouDead => {
                    state.player.charisma -= g.rng.random_in_range(1..3);
                }
                EnglishExamFeeling::StaminaGone => {
                    state.player.stamina -= g.rng.random_in_range(1..3);
                }
                EnglishExamFeeling::BrainGotBetter => {
                    state.player.brain += g.rng.random_in_range(1..4);
                }
                EnglishExamFeeling::LoveForTheWorld => {
                    state.player.charisma += g.rng.random_in_range(1..4);
                }
                EnglishExamFeeling::ReadyForEverything => {
                    state.player.stamina += g.rng.random_in_range(1..4);
                }
                EnglishExamFeeling::Money => {
                    if state.player.money > 0 {
                        state.player.money = Money(0)
                    } else {
                        state.player.money = Money(20)
                    }
                }
                EnglishExamFeeling::SmellOfGarlic => {
                    let garlic = g.rng.random_in_range(1..5);
                    state.player.garlic = garlic;
                    state.player.charisma -= g.rng.random(garlic / 2);
                }
                EnglishExamFeeling::QuicklyFadedAway => (),
            }
        }
        Subject::Calculus
        | Subject::GeometryAndTopology
        | Subject::ComputerScience
        | Subject::PhysicalEducation => {
            g.set_screen_and_wait_for_any_key(GameScreen::Exam(ExamScene::ExamPassed(
                state.clone(),
                subject,
            )))
            .await
        }
    }

    ExamResult::Continue
}

async fn exam_ends(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) -> ExamResult {
    match subject {
        Subject::AlgebraAndNumberTheory => {
            if state.location() == Location::PUNK
                && !state
                    .player
                    .status_for_subject(subject)
                    .solved_all_problems()
            {
                match g
                    .set_screen_and_wait_for_action(GameScreen::Exam(
                        ExamScene::PromptExamInTrain(state.clone(), subject),
                    ))
                    .await
                {
                    ContinueSufferingWithExamInTrainAction::WantToSufferMore => {
                        exam_in_train(g, state, subject).await;
                    }
                    ContinueSufferingWithExamInTrainAction::NoThanks => (),
                };
                return ExamResult::Exit;
            }
            // Всемирнов никогда не задерживается.
        }
        _ => {
            if state.player.health <= 0 {
                // Баг в оригинальной реализации, должно быть ExamResult::Exit.
                // TODO: Написать на это тест
                return ExamResult::Continue;
            }
            if subject.mental_load().0 * 2 + state.current_time().0 as i16 * 6
                < state.player.charisma.0 * 3 + g.rng.random_in_range(20..40)
            {
                g.set_screen_and_wait_for_any_key(GameScreen::Exam(
                    ExamScene::ProfessorLingers(state.clone(), subject),
                ))
                .await;
                state
                    .current_day_mut()
                    .exam_mut(subject)
                    .unwrap()
                    .one_hour_more();
                return ExamResult::Continue;
            }
        }
    };
    g.set_screen_and_wait_for_any_key(GameScreen::Exam(ExamScene::ProfessorLeaves(
        state.clone(),
        subject,
    )))
    .await;
    ExamResult::Exit
}

async fn exam_in_train(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
) {
    if state.current_time() > Time(20) {
        // В этом случае Всемирнов должен отказаться принимать зачёт в электричке,
        // но, кажется, эта ветка недостижима, так как Всемирнов никогда не задерживается,
        // а по расписанию экзамен не может заканчиваться позже 18:00.
        unreachable!("Экзамен по алгебре после 20:00? Это как?")
    }
    let caught_by_inspectors =
        train::go_by_train(g, state, HealthLevel(0), &|state, train| {
            GameScreen::Exam(ExamScene::Train(state, train))
        })
        .await;
    if caught_by_inspectors {
        // Баг в оригинальной реализации.
        // Если поймали контролёры, то появляется лишний пустой экран с единственным
        // предложением нажать любую клавишу.
        g.set_screen_and_wait_for_any_key(GameScreen::Exam(
            ExamScene::CaughtByInspectorsEmptyScreenBug,
        ))
        .await;
    }
    suffer_exam(g, state, true, subject).await;

    if state.current_time() > Time(20) {
        // "Увы, ПОМИ уже закрыто, поэтому придется ехать домой.."
        // Тут мы должны мгновенно оказаться в общаге, без контролёров и всего такого.
        // TODO: Выяснить, достижима ли эта ветка. Кажется, будто бы нет.
        unreachable!()
    }

    match g
        .set_screen_and_wait_for_action::<BaltiyskiyRailwayStationAction>(
            GameScreen::BaltiyskiyRailwayStation(BaltiyskiyRailwayStationScene::Prompt(
                state.clone(),
            )),
        )
        .await
    {
        BaltiyskiyRailwayStationAction::GoToPDMI => {
            state.set_location(Location::PDMI);
        }
        BaltiyskiyRailwayStationAction::GoToPUNK => {
            // Баг в оригинальной реализации: даже если есть деньги, билет купить
            // не предлагается.
            // Кроме того, не отнимается здоровье.
            if state.player.has_roundtrip_train_ticket() {
                // Баг в оригинальной реализации: на экран должно быть выведено
                // "Хорошо, билет есть...", но поскольку нет вызова wait_for_key,
                // эта надпись не успевает появиться на экране.
            } else if train::inspectors(&mut g.rng, state) {
                g.set_screen_and_wait_for_any_key(GameScreen::BaltiyskiyRailwayStation(
                    BaltiyskiyRailwayStationScene::CaughtByInspectors,
                ))
                .await;
                misc::decrease_health(
                    state,
                    HealthLevel(10),
                    CauseOfDeath::CorpseFoundInTheTrain,
                );
                misc::hour_pass(g, state, None).await;
            } else {
                // Баг в оригинальной реализации: на экран должно быть выведено
                // "Уф, доехал...", но поскольку нет вызова wait_for_key,
                // эта надпись не успевает появиться на экране.
            }
            state.set_location(Location::PUNK);
            misc::hour_pass(g, state, None).await;
        }
    };
}

async fn classmate_wants_something(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    subject: Subject,
    classmate: Classmate,
) {
    if classmate == Classmate::RAI && subject == Subject::ComputerScience {
        // RAI не пристаёт на зачёте по информатике.
        return;
    }

    let selected_action = g
        .set_screen_and_wait_for_action_vec(
            GameScreen::Exam(ExamScene::ClassmateWantsSomething(
                state.clone(),
                subject,
                classmate,
            )),
            [
                NpcApproachAction::Ignore,
                NpcApproachAction::TalkToClassmate(classmate),
            ],
        )
        .await;

    match selected_action {
        NpcApproachAction::Ignore => {
            let feeling_bad = classmate.health_penalty() > 0;
            g.set_screen_and_wait_for_any_key(GameScreen::Exam(
                ExamScene::IgnoredClassmate { feeling_bad },
            ))
            .await;
            if feeling_bad {
                misc::decrease_health(
                    state,
                    classmate.health_penalty(),
                    CauseOfDeath::BetterNotIgnoreClassmate(classmate),
                );
            }
        }
        NpcApproachAction::TalkToClassmate(_) => {
            interact_with_classmate(g, state, classmate, Some(subject)).await;
        }
    }
}
