use super::*;
use strum::VariantArray;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerjInteraction {
    /// "На, глотни кефирчику."
    HaveSomeKefir,

    /// "Я знаю, где срезать в парке на физ-ре!"
    IKnowWhereToCutInThePark,

    RandomReply(SerjReply),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum SerjReply {
    /// "Помнится, когда-то была еще графическая версия mmHeroes..."
    GuiMmheroes,

    /// "Я был бета-тестером первой версии mmHeroes (тогда еще CRWMM19)!"
    IWasABetaTester,

    /// "Как здорово, что Diamond написал новую версию!"
    HowGreatThatDiamondWroteANewVersion,

    /// "Ты уже получил деньги у Паши?"
    HaveYouAlreadyGotStipendFromPasha,

    /// "Попробуй для начала легкие зачеты."
    TryEasyExamsFirst,

    /// "Ты еще не получил зачет по английскому?"
    HaventYouPassedEnglishExam,

    /// "Хочешь отдыхать, где угодно? Заимей деньги!"
    WantToRestAnywhereGetMoney,

    /// "Не в деньгах счастье. Но они действуют успокаивающе."
    MoneyCantBuyHappiness,

    /// "На Всемирнове всегда толпа народу."
    AlwaysCrowdedOnVsemirnov,

    /// "Влащенко - дама весьма оригинальная."
    VlaschenkoIsOriginalLady,

    /// "Интересно, когда будет готова следующая версия?"
    WhenWillNewVersionBeReady,

    /// "Здоровье в кафе повышается в зависимости от наличия денег."
    HealthInCafe,

    /// "Если бы я знал адрес хорошего proxy..."'
    IfOnlyIKnewProxyAddress,

    /// "STAR временно накрылся. Хорошо бы узнать адрес другого proxy..."
    StarIsKaput,

    /// "Я подозреваю, что Гриша знает адресок теркомовского proxy."
    GrishaKnowsProxyAddress,

    /// "А Diamond все свободное время дописывает свою игрушку!"
    DiamondSpendsAllHisFreeTimeOnTheGame,

    /// "В следующем семестре информатику будет вести Терехов-младший."
    NextTermTerekhovJrWillTeachCS,

    /// "Diamond хочет переписать это все на Java."
    DiamondWantsToRewriteItInJava,

    /// "Миша проконсультирует тебя о стратегии."
    MishaWillTellYouTheStrategy,

    /// "Поговори с Diamond'ом, он много ценного скажет."
    TalkWithDiamondHeKnowsALot,

    /// "Борись до конца!"
    FightUntilTheEnd,

    /// "У Дубцова иногда бывает халява."
    SometimesThereIsFreebieWithDubtsov,
}

use crate::logic::Subject::PhysicalEducation;
use SerjInteraction::*;

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    let serj_leaves = state.player.charisma < g.rng.random(CharismaLevel(9));

    if g.rng.random(state.player.charisma.0) > g.rng.random_in_range(2..5)
        && state.player.charisma.0 * 2 + 20 > state.player.health
    {
        g.set_screen_and_wait_for_any_key(GameScreen::SerjInteraction(
            state.clone(),
            HaveSomeKefir,
            serj_leaves,
        ))
        .await;

        state.player.health += state.player.charisma.0;
        state.player.health += g.rng.random(state.player.charisma.0);

        if let Some(current_subject) = exam_in_progress {
            let knowledge = &mut state
                .player
                .status_for_subject_mut(current_subject)
                .knowledge;
            if *knowledge > BrainLevel(3) {
                *knowledge -= g.rng.random(3);
            }
        }
    } else if g.rng.random(state.player.charisma.0) > g.rng.random_in_range(2..8) {
        if state.player.status_for_subject(PhysicalEducation).knowledge < BrainLevel(10) {
            g.set_screen_and_wait_for_any_key(GameScreen::SerjInteraction(
                state.clone(),
                IKnowWhereToCutInThePark,
                serj_leaves,
            ))
            .await;
            state
                .player
                .status_for_subject_mut(PhysicalEducation)
                .knowledge += 30;
        }
    } else {
        let reply = g.rng.random_variant();
        g.set_screen_and_wait_for_any_key(GameScreen::SerjInteraction(
            state.clone(),
            RandomReply(reply),
            serj_leaves,
        ))
        .await;
    }

    if serj_leaves {
        state.classmates[Serj].current_location =
            match state.classmates[Serj].current_location {
                ClassmateLocation::Nowhere => panic!("Invalid location"),
                ClassmateLocation::Location(Location::Mausoleum) => {
                    ClassmateLocation::Nowhere
                }
                ClassmateLocation::Exam(_) | ClassmateLocation::Location(_) => {
                    ClassmateLocation::Location(Location::Mausoleum)
                }
            }
    }
}
