use super::*;
use strum::VariantArray;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiamondInteraction {
    /// "Хочешь по-тестить новую версию Heroes of MAT-MEX?"
    WannaTestNewMMHEROES,

    /// "Ну и ладушки! Вот тебе дискетка..."
    HereIsTheFloppy,

    /// "Извини, что побеспокоил."
    SorryForBothering,

    Reply(DiamondReply),
}

use crate::logic::actions::MmheroesFloppyAction;
use DiamondInteraction::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, VariantArray)]
pub enum DiamondReply {
    /// "Коля поможет с алгеброй."
    KolyaWillHelpWithAlgebra,

    /// "Миша расскажет всем, какой ты хороший."
    MishaWillTellEveryoneHowGoodYouAre,

    /// "Паша - твой староста."
    PashaIsYourHeadman,

    /// "С DJuGом лучше не сталкиваться."
    BetterAvoidDJuG,

    /// "RAI не отстанет, лучше решить ему чего-нибудь."
    RAIWontLeaveYouAlone,

    /// "Коля все время сидит в мавзолее и оттягивается."
    KolyaIsAlwaysInMausoleum,

    /// "Следи за своим здоровьем!!!"
    WatchYourHealth,

    /// "Если встретишь Сашу - ОБЯЗАТЕЛЬНО заговори с ним."
    IfYouMeetSashaTalkToHim,

    /// "Если плохо думается, попробуй поговорить с RAI."
    IfTroubleThinkingTalkWithRAI,

    /// "Идя к Коле, будь уверен, что можешь пить с ним."
    BeSureYouCanDrinkBeforeGoingToKolya,

    /// "Получая зачет по английскому, будь готов к неожиданностям."
    ExpectSurprisesOnEnglishExam,

    /// "Иногда разговоры с Сержем приносят ощутимую пользу."
    TalksWithSerj,

    /// "Эндрю может помочь, но не всегда..."
    AndrewCanHelpButNotAlways,

    /// "Кузьменко иногда знает о Климове больше, чем сам Климов."
    KuzmenkoKnowsAboutKlimov,

    /// "Не спеши слать гневные письма о багах:
    /// загляни на mmheroes.chat.ru,
    /// может быть, все уже в порядке!"
    DontRushWritingBugReports,

    /// "Серж тоже иногда забегает в мавзолей."
    SerjSometimesAppearsInMausoleum,

    /// "Не переучи топологию, а то Подкорытов-младший не поймет."
    DontOverstudyTopology,

    /// "Можешь устроиться в ТЕРКОМ по знакомству."
    YouCanGetAJobInTERKOM,

    /// "Гриша работает ( ;*) ) в ТЕРКОМе."
    GrishaWorksAtTERKOM,

    /// "В ТЕРКОМЕ можно заработать какие-то деньги."
    YouCanEarnMoneyAtTERKOM,

    /// "Гриша иногда бывает в Мавзолее."
    GrishaSometimesAppearsInMausoleum,

    /// "Не нравится расписание? Подумай о чем-нибудь парадоксальном."
    DontLikeTimetable,

    /// "NiL дает деньги за помощь, но..."
    NiLPaysForHelpBut,

    /// "Честно, не знаю, когда будет готов порт под Linux..."
    DontKnowWhenLinuxPortWillBeReady,

    /// "Срочно! Нужны новые фишки для "Зачетной недели" !"
    NeedNewFeaturesForMMHEROES,

    /// "Пожелания, идеи, bug report'ы шлите на mmheroes@chat.ru !"
    SendIdeasAndBugReports,

    /// "Встретишь Костю Буленкова - передай ему большой привет!"
    SendGreetingsToKostyaBulenkov,

    /// "Большое спасибо Ване Павлику за mmheroes.chat.ru !"
    ThanksVanyaPavlik,
}

pub(super) async fn interact(
    g: &mut InternalGameState<'_>,
    state: &mut GameState,
    exam_in_progress: Option<Subject>,
) {
    if !state.player().has_mmheroes_floppy()
        && state.location() == Location::ComputerClass
        && g.rng.roll_dice(8)
    {
        match g
            .set_screen_and_wait_for_action::<MmheroesFloppyAction>(
                GameScreen::DiamondInteraction(WannaTestNewMMHEROES, false),
            )
            .await
        {
            MmheroesFloppyAction::WantToTestNewMMHEROES => {
                g.set_screen_and_wait_for_any_key(GameScreen::DiamondInteraction(
                    HereIsTheFloppy,
                    false,
                ))
                .await;
                state.player.set_has_mmheroes_floppy();
            }
            MmheroesFloppyAction::DontWantToTestNewMMHEROES => {
                g.set_screen_and_wait_for_any_key(GameScreen::DiamondInteraction(
                    SorryForBothering,
                    false,
                ))
                .await
            }
        }
        return;
    }

    let reply = g.rng.random_variant();

    let diamond_leaves = exam_in_progress.is_none() && g.rng.roll_dice(2);
    g.set_screen_and_wait_for_any_key(GameScreen::DiamondInteraction(
        Reply(reply),
        diamond_leaves,
    ))
    .await;
    if diamond_leaves {
        state.classmates[Diamond].current_location = ClassmateLocation::Nowhere;
    }
}
