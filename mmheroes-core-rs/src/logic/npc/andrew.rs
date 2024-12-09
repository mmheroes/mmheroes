use super::*;

#[derive(Debug, Clone)]
pub enum AndrewInteraction {
    /// "Обратиться к Эндрю за помощью?"
    PromptHelpFromAndrew(GameState),

    RandomReply(AndrewReply),

    /// "Я подозреваю, что <препод> ничего тебе не засчитает." или
    /// "Я подозреваю, что <препод> зачтет тебе за 1 заход _x_ заданий."
    ScorePrediction {
        subject: Subject,
        prediction: u8,
    },

    /// "Эндрю решил тебе _x_ заданий" или "У Эндрю ничего не вышло"
    AndrewSolvedProblems {
        solved_by_andrew: u8,
        no_problems_remaining: bool,
    },

    /// "Эндрю тебя игнорирует!"
    AndrewIgnoresYou,
}

use AndrewInteraction::*;

#[derive(Debug, Copy, Clone, VariantArray)]
pub enum AndrewReply {
    /// "Скажи Diamond'у, что маловато описалова!"
    TellDiamondTooLittleDescriptions,

    /// "А еще Diamond думал переписать это на JavaScript."
    DiamondThoughtAboutRewritingThisInJavaScript,

    /// "А я знаю выигрышную стратегию! Если только не замочат..."
    IKnowTheWinningStrategy,

    /// "Вообще-то, все это происходит в мае 1998 г."
    ThisIsHappeningInMay1998,

    /// "Я видел надпись на парте: ЗАКОН ВСЕМИРНОВА ТЯГОТЕНИЯ"
    ISawASignOnATable,

    /// "Загляни на mmheroes.chat.ru!"
    VisitMmheroesWebsite,

    /// "Только не предлагай Diamond'у переписать все на Прологе!"
    DontSuggestRewritingThisInProlog,

    /// "Ну когда же будет порт под Linux?"
    WhenWillLinuxPortBeReady,

    /// "VMWARE - SUXX... Но под ним идут Heroes of Mat & Mech!"
    VmwareSuxx,

    /// "Похоже, что моя стратегия обламывается..."
    SeemsLikeMyStrategyIsNotWorkingOut,

    /// "Ух ты! Гамма 3.14 - в этом что-то есть."
    Gamma314ThereIsSomethingInIt,

    /// "Может быть, Diamond'а просто заклинило на многоточиях?"
    MaybeDiamondIsCrazyAboutEllipsis,

    /// "Говорят, можно зарабатывать деньги, почти ничего не делая."
    YouCanEarnMoneyByDoingNothing,

    /// "Вот, иногда мне приходится тяжко - когда пристают всякие..."
    SometimesItsHardForMe,

    /// "Хорошо ли, что многие реплики персонажей посвящены самой игре?"
    IsItGoodThatManyRepliesAreAboutTheGame,

    /// "Помогите мне! Хочу в Inet!"
    HelpMeIWantInternet,

    /// "А что? А ничего."
    WhatNothing,

    /// "Если оно цвета бордо - значит, оно тебе снится."
    IfItsBurgundyYouAreDreaming,

    /// "Всех с ДНЕМ МАТ-МЕХА!"
    HappyMathMechDay,

    /// "Придумай свою фразу для персонажа!"
    ThinkOfAPhraseForACharacter,

    /// "120К исходников - вот что такое mmHeroes!"
    MmheroesIs120kOfSources,

    /// "120К весьма кривых исходников - вот что такое mmHeroes!"
    MmheroesIs120kOfSloppySources,
}

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    let subject = exam_in_progress.expect("Эндрю не может быть не на зачёте!");
    match g
        .set_screen_and_wait_for_action(GameScreen::AndrewInteraction(
            PromptHelpFromAndrew(state.clone()),
        ))
        .await
    {
        actions::HelpFromAndrewAction::YesAmIWorseThanEveryoneElse => {
            if state.player.charisma > g.rng.random(14) {
                let problems_remaining = state
                    .player
                    .status_for_subject(subject)
                    .problems_remaining();
                let mut solved_by_andrew =
                    (g.rng.random(problems_remaining) as f32).sqrt().floor() as u8;
                if solved_by_andrew > 2 {
                    solved_by_andrew = 0;
                }
                state
                    .player
                    .status_for_subject_mut(subject)
                    .more_problems_solved(solved_by_andrew);
                let no_problems_remaining = state
                    .player
                    .status_for_subject(subject)
                    .solved_all_problems();
                g.set_screen_and_wait_for_any_key(GameScreen::AndrewInteraction(
                    AndrewSolvedProblems {
                        solved_by_andrew,
                        no_problems_remaining,
                    },
                ))
                .await;
                state.player.stamina -= g.rng.random(2);
                misc::hour_pass(g, state, exam_in_progress).await;
            } else {
                g.set_screen_and_wait_for_any_key(GameScreen::AndrewInteraction(
                    AndrewIgnoresYou,
                ))
                .await;
                misc::decrease_health(
                    state,
                    g.rng.random_in_range(2..7),
                    CauseOfDeath::AndrewCanDefendHimself,
                )
            }
        }
        actions::HelpFromAndrewAction::IWillDoItMyself => {
            if g.rng.roll_dice(3) {
                // Баг в оригинальной реализации: прогноз берётся для текущего предмета,
                // но отображается имя случайного преподавателя.
                let random_subject = g.rng.random_variant();
                let prediction = scene_router::exams::number_of_problems_accepted(
                    &mut g.rng, state, subject, false,
                );
                g.set_screen_and_wait_for_any_key(GameScreen::AndrewInteraction(
                    ScorePrediction {
                        subject: random_subject,
                        prediction,
                    },
                ))
                .await;
            } else {
                let reply = g.rng.random_variant();
                g.set_screen_and_wait_for_any_key(GameScreen::AndrewInteraction(
                    RandomReply(reply),
                ))
                .await;
            }
        }
    }
}
