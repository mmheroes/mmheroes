#![allow(clippy::missing_safety_doc)]

use crate::ui::high_scores;
use crate::ui::Milliseconds;
use crate::ui::*;

use crate::logic::{create_game, Game, GameMode, Money, StateHolder, Time};

use crate::ui::high_scores::{HighScore, SCORE_COUNT};
use crate::ui::renderer::RendererRequestConsumer;
use crate::util::TinyString;
use core::ffi::c_void;
use core::mem::{align_of_val, size_of_val, MaybeUninit};

pub type AllocatorContext = *mut c_void;

/// Функция, принимающая в качестве первого аргумента некоторый контекст,
/// в качестве второго аргумента размер выделяемого блока памяти,
/// а в качестве третьего — выравнивание.
pub type Allocator = unsafe extern "C" fn(AllocatorContext, usize, usize) -> *mut c_void;

/// Функция, принимающая в качестве первого аргумента некоторый контекст,
/// в качестве второго — указатель на освобождаемый блок памяти,
/// а в качестве третьего — размер освобождаемого блока.
pub type Deallocator = unsafe extern "C" fn(AllocatorContext, *mut c_void, usize);

pub type RendererRequestCallback = extern "C" fn(*mut c_void, FfiRendererRequest);

pub struct FfiRendererRequestConsumer {
    context: *mut c_void,
    callback: RendererRequestCallback,
}

impl RendererRequestConsumer for FfiRendererRequestConsumer {
    fn consume_request(&mut self, request: RendererRequest) {
        (self.callback)(self.context, request.into())
    }
}

struct FfiGame<G: 'static> {
    state_holder: StateHolder,
    game: MaybeUninit<G>,
    game_ui:
        MaybeUninit<GameUI<'static, G, FfiRendererRequestConsumer, InputRecorderSink>>,
}

impl<G: Game> FfiGame<G> {
    unsafe fn cast_mut<'a>(
        raw_ptr: *mut c_void,
        _constructor: impl FnOnce() -> G,
    ) -> &'a mut Self {
        &mut *(raw_ptr as *mut Self)
    }

    unsafe fn cast_ref<'a>(
        raw_ptr: *const c_void,
        _constructor: impl FnOnce() -> G,
    ) -> &'a Self {
        &*(raw_ptr as *const Self)
    }
}

macro_rules! game_or_return {
    (const $memory:ident, $retval:stmt) => {
        if ($memory.is_null()) {
            $retval
        } else {
            FfiGame::cast_ref($memory, || {
                #[allow(unreachable_code)]
                create_game(unreachable!(), unreachable!())
            })
        }
    };
    (mut $memory:ident, $retval:stmt) => {
        if ($memory.is_null()) {
            $retval
        } else {
            FfiGame::cast_mut($memory, || {
                #[allow(unreachable_code)]
                create_game(unreachable!(), unreachable!())
            })
        }
    };
}

/// Записывает текущий игровой день и время в аргументы `out_day` и `out_time`
/// и возвращает `true` если они доступны, иначе не трогает аргументы и возвращает
/// `false`.
///
/// Игровой день и время могут быть недоступны, например, если игра ещё не началась.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_get_current_time(
    game: *const c_void,
    out_day: &mut u8,
    out_time: &mut Time,
) -> bool {
    let game = game_or_return!(const game, return false);
    let borrowed_state = game.state_holder.observable_state();
    if let Some(state) = borrowed_state.screen().state() {
        *out_day = state.current_day().index() as u8;
        *out_time = state.current_time();
        true
    } else {
        false
    }
}

#[repr(C)]
pub struct FfiHighScore {
    name: *const u8,
    name_len: usize,
    score: Money,
}

impl FfiHighScore {
    unsafe fn name<'a>(&self) -> &'a str {
        let name_buf = core::slice::from_raw_parts(self.name, self.name_len);
        core::str::from_utf8(name_buf).expect("Name is not valid UTF-8")
    }
}

unsafe fn get_high_scores(ptr: *const FfiHighScore) -> Option<[HighScore; SCORE_COUNT]> {
    if ptr.is_null() {
        None
    } else {
        let mut scores = high_scores::default_high_scores();
        let slice = core::slice::from_raw_parts(ptr, SCORE_COUNT);
        for (i, score) in slice.iter().enumerate() {
            let name = score.name();
            scores[i].0 = TinyString::from(name);
            scores[i].1 = score.score;
        }
        Some(scores)
    }
}

/// Выделяет память для объекта, используя переданный аллокатор,
/// а затем инициализирует объект и возвращает на него указатель.
///
/// Аллокатор должен возвращать корректно выровненный указатель на блок памяти
/// достаточного размера. Нарушение любого из этих условий — неопределённое поведение.
///
/// Размер и выравнивание передаются в качестве аргументов аллокатору.
///
/// Параметр `high_scores` — указатель (возможно нулевой) на массив из
/// `MMHEROES_SCORE_COUNT` элементов.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_create(
    mode: GameMode,
    seed: u64,
    high_scores: *const FfiHighScore,
    allocator_context: AllocatorContext,
    allocator: Allocator,
    renderer_request_callback_context: *mut c_void,
    renderer_request_callback: RendererRequestCallback,
    input_recorder_sink: InputRecorderSink,
) -> *mut c_void {
    use core::ptr::{null_mut, NonNull};

    let scores = get_high_scores(high_scores);

    let renderer_request_consumer = FfiRendererRequestConsumer {
        context: renderer_request_callback_context,
        callback: renderer_request_callback,
    };

    let ffi_game = FfiGame {
        state_holder: StateHolder::new(mode),
        game: MaybeUninit::uninit(),
        game_ui: MaybeUninit::uninit(),
    };

    let memory: *mut FfiGame<_> = allocator(
        allocator_context,
        size_of_val(&ffi_game),
        align_of_val(&ffi_game),
    ) as *mut _;
    let mut memory = match NonNull::new(memory) {
        Some(memory) => memory,
        None => return null_mut(),
    };
    memory.write(ffi_game);

    let state_holder = &memory.as_ref().state_holder;
    let game = memory.as_mut().game.write(create_game(seed, state_holder));
    memory.as_mut().game_ui.write(GameUI::new(
        state_holder,
        core::pin::Pin::new_unchecked(game),
        seed,
        scores,
        renderer_request_consumer,
        Some(input_recorder_sink),
    ));

    memory.as_ptr() as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_destroy(
    game: *mut c_void,
    allocator_context: AllocatorContext,
    deallocator: Deallocator,
) {
    let game = game_or_return!(mut game, return);
    let size = size_of_val(game);
    game.game_ui.assume_init_drop();
    game.game.assume_init_drop();
    let game_ptr: *mut FfiGame<_> = game as *mut _;
    game_ptr.drop_in_place();
    deallocator(allocator_context, game_ptr as *mut c_void, size);
}

/// Записывает в аргумент `out` `MMHEROES_SCORE_COUNT` элементов.
/// `out` не должен быть нулевым указателем.
/// Результат, записанный в `out`, не должен жить дольше, чем экземпляр
/// соответствующего `GameUI`.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_get_high_scores(
    game: *const c_void,
    out: *mut FfiHighScore,
) {
    assert!(!out.is_null());
    let game = game_or_return!(const game, return);
    let out_slice = core::slice::from_raw_parts_mut(out, SCORE_COUNT);
    for (high_score, out) in game
        .game_ui
        .assume_init_ref()
        .high_scores
        .iter()
        .zip(out_slice.iter_mut())
    {
        *out = FfiHighScore {
            name: high_score.0.as_ptr(),
            name_len: high_score.0.len(),
            score: high_score.1,
        }
    }
}
/// `new_high_scores` — ненулевой указатель на массив из `MMHEROES_SCORE_COUNT` элементов.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_set_high_scores(
    game: *mut c_void,
    new_high_scores: *const FfiHighScore,
) {
    assert!(!new_high_scores.is_null());
    let game = game_or_return!(mut game, return);
    let new_high_scores = core::slice::from_raw_parts(new_high_scores, SCORE_COUNT);
    for (high_score, new_high_score) in game
        .game_ui
        .assume_init_mut()
        .high_scores
        .iter_mut()
        .zip(new_high_scores.iter())
    {
        let name_buf =
            core::slice::from_raw_parts(new_high_score.name, new_high_score.name_len);
        let name = core::str::from_utf8(name_buf).expect("Name is not valid UTF-8");
        high_score.0 = TinyString::from(name);
        high_score.1 = new_high_score.score;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FfiRendererRequest {
    ClearScreen,
    Flush,
    WriteStr {
        buf: *const u8,
        length: usize,
    },
    MoveCursor {
        line: u8,
        column: u8,
    },
    SetColor {
        foreground: Color,
        background: Color,
    },
    Sleep {
        milliseconds: Milliseconds,
    },
}

impl<'a> From<RendererRequest<'a>> for FfiRendererRequest {
    fn from(request: RendererRequest<'a>) -> Self {
        match request {
            RendererRequest::ClearScreen => FfiRendererRequest::ClearScreen,
            RendererRequest::Flush => FfiRendererRequest::Flush,
            RendererRequest::WriteStr(s) => FfiRendererRequest::WriteStr {
                buf: s.as_ptr(), // FIXME: Lifetime!!!
                length: s.len(),
            },
            RendererRequest::MoveCursor { line, column } => {
                FfiRendererRequest::MoveCursor { line, column }
            }
            RendererRequest::SetColor {
                foreground,
                background,
            } => FfiRendererRequest::SetColor {
                foreground,
                background,
            },
            RendererRequest::Sleep(milliseconds) => {
                FfiRendererRequest::Sleep { milliseconds }
            }
        }
    }
}

/// Воспроизводит игру с помощью входных данных, записанных ранее с помощью
/// `InputRecorder`.
///
/// В случае ошибки возвращает `false`, иначе — `true`.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_replay(
    game: *mut c_void,
    recorded_input: *const u8,
    recorded_input_len: usize,
) -> bool {
    assert!(!recorded_input.is_null());
    let game = game_or_return!(mut game, return false);
    let slice = core::slice::from_raw_parts(recorded_input, recorded_input_len);
    let s = match core::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let mut parser = recording::InputRecordingParser::new(s);
    parser
        .parse_all(|input| game.game_ui.assume_init_mut().continue_game(input))
        .is_ok()
}

/// Продолжает игру до следующего запроса на нажатие клавиши.
///
/// При первом вызове этой функции неважно, что передаётся в параметре `input`.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_continue(game: *mut c_void, input: Input) -> bool {
    let game = game_or_return!(mut game, return false);
    game.game_ui.assume_init_mut().continue_game(input)
}

#[no_mangle]
pub unsafe extern "C" fn mmheroes_flush_input_recorder(game: *mut c_void) -> bool {
    let game = game_or_return!(mut game, return false);
    game.game_ui
        .assume_init_mut()
        .flush_input_recorder()
        .is_ok()
}

#[repr(C)]
pub struct InputRecorderSink {
    context: *mut c_void,
    sink: Option<unsafe extern "C" fn(*mut c_void, *const u8, usize) -> bool>,
    display: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>,
}

impl core::fmt::Write for InputRecorderSink {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Some(sink) = self.sink {
            unsafe {
                if sink(self.context, s.as_ptr(), s.len()) {
                    Ok(())
                } else {
                    Err(core::fmt::Error)
                }
            }
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Display for InputRecorderSink {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(display) = self.display {
            unsafe {
                if display(self.context, f as *mut core::fmt::Formatter as *mut c_void) {
                    Ok(())
                } else {
                    Err(core::fmt::Error)
                }
            }
        } else {
            Ok(())
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mmheroes_rust_display(
    string: *const u8,
    len: usize,
    formatter: *mut c_void,
) -> bool {
    let formatter = (formatter as *mut core::fmt::Formatter).as_mut().unwrap();
    let bytes = core::slice::from_raw_parts(string, len);
    let str = core::str::from_utf8(bytes).unwrap();
    write!(formatter, "{}", str).is_ok()
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;
    use std::ptr::null_mut;

    unsafe extern "C" fn allocator(
        _context: AllocatorContext,
        size: usize,
        alignment: usize,
    ) -> *mut c_void {
        std::alloc::alloc_zeroed(Layout::from_size_align(size, alignment).unwrap())
            as *mut c_void
    }

    unsafe extern "C" fn deallocator(
        _context: AllocatorContext,
        memory: *mut c_void,
        size: usize,
    ) {
        std::alloc::dealloc(memory as *mut u8, Layout::from_size_align(size, 8).unwrap())
    }

    fn high_scores() -> [FfiHighScore; high_scores::SCORE_COUNT] {
        macro_rules! ffi_high_score {
            ($name:literal, $score:literal) => {
                FfiHighScore {
                    name: $name.as_ptr(),
                    name_len: $name.len(),
                    score: Money($score),
                }
            };
        }
        [
            ffi_high_score!("Оля", 142),
            ffi_high_score!("Вероника", 192),
            ffi_high_score!("Наташа", 144),
            ffi_high_score!("Катя", 113),
            ffi_high_score!("Рита", 120),
        ]
    }

    unsafe extern "C" fn sink(context: *mut c_void, data: *const u8, len: usize) -> bool {
        let log = (context as *mut String).as_mut().unwrap();
        match core::str::from_utf8(core::slice::from_raw_parts(data, len)) {
            Ok(s) => {
                log.push_str(s);
                true
            }
            Err(_) => false,
        }
    }

    unsafe extern "C" fn display(context: *mut c_void, formatter: *mut c_void) -> bool {
        let log = (context as *mut String).as_mut().unwrap();
        mmheroes_rust_display(log.as_ptr(), log.len(), formatter)
    }

    #[test]
    fn test_ffi() {
        unsafe {
            let scores = high_scores();

            let mut requests = <Vec<FfiRendererRequest>>::new();

            extern "C" fn renderer_request_callback(
                context: *mut c_void,
                renderer_request: FfiRendererRequest,
            ) {
                unsafe { (*(context as *mut Vec<_>)).push(renderer_request) }
            }

            let mut log = String::new();

            let sink = InputRecorderSink {
                context: &mut log as *mut String as *mut c_void,
                sink: Some(sink),
                display: Some(display),
            };

            let game = mmheroes_game_create(
                GameMode::Normal,
                0,
                scores.as_ptr(),
                null_mut(),
                allocator,
                &mut requests as *mut _ as *mut c_void,
                renderer_request_callback,
                sink,
            );

            let mut scores =
                [const { MaybeUninit::<FfiHighScore>::uninit() }; SCORE_COUNT];
            mmheroes_game_get_high_scores(
                game,
                core::mem::transmute(scores.as_mut_ptr()),
            );

            assert_eq!(scores[0].assume_init_ref().name(), "Оля");
            assert_eq!(scores[1].assume_init_ref().name(), "Вероника");
            assert_eq!(scores[2].assume_init_ref().name(), "Наташа");
            assert_eq!(scores[3].assume_init_ref().name(), "Катя");
            assert_eq!(scores[4].assume_init_ref().name(), "Рита");

            mmheroes_continue(game, Input::Enter);

            assert_eq!(requests.len(), 29);

            let mut day = 255u8;
            let mut time = Time(255);
            assert!(!mmheroes_game_get_current_time(game, &mut day, &mut time));

            mmheroes_continue(game, Input::Enter);
            mmheroes_continue(game, Input::Enter);
            mmheroes_continue(game, Input::Enter);
            mmheroes_continue(game, Input::KeyDown);
            mmheroes_continue(game, Input::KeyDown);
            mmheroes_continue(game, Input::Enter);

            assert!(mmheroes_game_get_current_time(game, &mut day, &mut time));
            assert_eq!(day, 0);
            assert_eq!(time, Time(9));

            assert_eq!(log, "4r2↓");
            mmheroes_flush_input_recorder(game);
            assert_eq!(log, "4r2↓r");

            mmheroes_game_destroy(game, null_mut(), deallocator);
        }
    }
}
