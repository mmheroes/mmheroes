use crate::logic::{
    timetable, BrainLevel, CauseOfDeath, Duration, GameScreen, GameState, HealthLevel,
    InternalGameState, Location, Subject, Time,
};
use strum::VariantArray;

#[derive(Debug, Clone, strum::EnumDiscriminants)]
#[strum_discriminants(name(DreamTheme))]
pub enum DreamScreen {
    SubjectRelated(Subject),
    Stupid(StupidDream),
    Djug(DjugDream),
}

#[derive(Debug, Clone)]
pub enum StupidDream {
    Phase1(StupidDreamSubject, StupidDreamScenario),
    Phase2,
}

#[derive(Debug, Clone, Copy, VariantArray)]
pub enum StupidDreamSubject {
    /// "Розовые слоники с блестящими крылышками…"
    PinkElephants,

    /// "Зеленые человечки с длинными антеннами…"
    GreenMen,

    /// "Овечки с ослепительно-белой шерстью…"
    Sheep,
}

#[derive(Debug, Clone, Copy, VariantArray)]
pub enum StupidDreamScenario {
    /// "…и считают определитель матрицы 10 на 10"
    ComputingDeterminant,

    /// "…и ищут Жорданову форму матрицы"
    ComputingJordanMatrix,

    /// "…и возводят матрицы в 239-ю степень"
    RaiseMatrixToPower,

    /// "…и решают линейную систему уравнений с параметрами"
    SolvingLinearSystem,

    /// "…и доказывают неприводимость многочлена 10-й степени над Z"
    ProvingIrreducibilityOfPolynomial,

    /// "…и доказывают сходимость неопределенного интеграла с параметрами"
    ProvingConvergenceOfIntegral,

    /// "…и считают сумму ряда с параметрами"
    ComputingSumOfSeries,

    /// "…и дифференцируют, дифференцируют, дифференцируют"
    Differentiate,

    /// "…и берут интергалы не отдают их"
    TakingIntegrals,

    /// "…и решают задачи по математической болтологии"
    SolvingMathematicalProblems,
}

#[derive(Debug, Clone)]
pub enum DjugDream {
    /// "Здравствуйте! ..."
    Phase1,

    /// "Оно большое ..."
    Phase2,

    /// "Оно пыхтит! ..."
    Phase3,

    /// "Оно медленно ползет прямо на тебя!!! ..."
    Phase4,

    /// "Оно говорит человеческим голосом:"
    Phase5(DjugQuote),

    Phase6,
}

#[derive(Debug, Clone, Copy, VariantArray)]
pub enum DjugQuote {
    /// "Молодой человек. Когда-нибудь Вы вырастете
    /// и будете работать на большой машине.
    /// Вам надо будет нажать кнопку жизни,
    /// а Вы нажмете кнопку смерти ..."
    DeathButton,

    /// "Это в средневековье ученые спорили,
    /// сколько чертей может поместиться
    /// на кончике иглы..."
    HowManyDevilsFitANeedleTip,

    /// "Задачи можно решать по-разному.
    /// Можно устно, можно на бумажке,
    /// можно - играя в крестики-нолики...
    /// А можно - просто списать ответ в конце задачника!"
    DifferentWaysOfSolvingProblems,
}

pub(in crate::logic) async fn sleep(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
) {
    assert_eq!(
        state.location(),
        Location::Dorm,
        "Спать можно только в общаге"
    );
    if die_if_time_out(state) {
        return;
    }
    state.player.health = core::cmp::min(state.player.health, HealthLevel(40));
    let health_gain = (state.player.health.0 + g.rng.random_in_range(15..35)).min(50)
        - state.player.health.0;
    assert!(health_gain >= 0, "negative health_gain ({})", health_gain,);
    state.player.health += health_gain;
    let sleep_duration = 7 + g.rng.random(health_gain / 4);
    state.adjust_time(Duration(sleep_duration as i8));

    if state.current_time() >= Time(24) {
        state.set_current_time(state.current_time() % 24);
        state.next_day();
        die_if_time_out(state);
    }

    for subject in Subject::math_subjects() {
        state.set_sasha_has_lecture_notes(subject, true);
    }
    state.player.set_invited_to_party(false);

    let mut dream = DreamTheme::SubjectRelated;
    if state.player.brain <= 2 {
        state.player.brain = BrainLevel(2);
        dream = DreamTheme::Stupid;
    }
    if state.player.stamina <= 0 {
        state.player.health = HealthLevel(0);
        state.player.cause_of_death = Some(CauseOfDeath::TurnedToVegetable);
    }
    if state.player.knows_djug() {
        dream = DreamTheme::Djug;
    }

    if g.rng.roll_dice(2) {
        match dream {
            DreamTheme::Stupid => {
                stupid_dream(g).await;
            }
            DreamTheme::Djug => {
                djug_dream(g).await;
            }
            DreamTheme::SubjectRelated => {
                if g.rng.roll_dice(3) {
                    g.set_screen_and_wait_for_any_key(GameScreen::Dreaming(
                        DreamScreen::SubjectRelated(state.player.last_exam()),
                    ))
                    .await;
                }
            }
        }

        if matches!(dream, DreamTheme::Stupid | DreamTheme::Djug) {
            state.player.health = HealthLevel(g.rng.random_in_range(10..20));
        }
    }

    state.player.set_knows_djug(false);

    state.set_current_time(state.current_time().max(Time(5)));

    if state.player.garlic > 0 {
        state.player.garlic -= 1;
        state.player.charisma += g.rng.random(2);
    }
}

async fn dream_phase(g: &mut InternalGameState<'_>, dream: DreamScreen) {
    g.set_screen_and_wait_for_any_key(GameScreen::Dreaming(dream))
        .await;
}

async fn stupid_dream(g: &mut InternalGameState<'_>) {
    use DreamScreen::Stupid;
    use StupidDream::*;
    let subject = g.rng.random_variant();
    let scenario = g.rng.random_variant();
    dream_phase(g, Stupid(Phase1(subject, scenario))).await;
    dream_phase(g, Stupid(Phase2)).await;
}

async fn djug_dream(g: &mut InternalGameState<'_>) {
    use DjugDream::*;
    use DreamScreen::Djug;
    dream_phase(g, Djug(Phase1)).await;
    dream_phase(g, Djug(Phase2)).await;
    dream_phase(g, Djug(Phase3)).await;
    dream_phase(g, Djug(Phase4)).await;
    let quote = g.rng.random_variant();
    dream_phase(g, Djug(Phase5(quote))).await;
    dream_phase(g, Djug(Phase6)).await;
}

fn die_if_time_out(state: &mut GameState) -> bool {
    let last_day_index = timetable::NUM_DAYS as u8;
    assert!(state.current_day_index() <= last_day_index);
    let time_out = state.current_day_index() == last_day_index;
    if time_out {
        state.player.cause_of_death = Some(CauseOfDeath::TimeOut);
    };
    time_out
}
