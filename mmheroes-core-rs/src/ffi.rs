#![allow(clippy::missing_safety_doc)]

use crate::ui::high_scores;
use crate::ui::recording::InputRecorder;
use crate::ui::Milliseconds;
use crate::ui::*;

use crate::logic::{Game, GameMode, Money, Time};

use crate::ui::high_scores::SCORE_COUNT;
use crate::ui::renderer::RendererRequestConsumer;
use crate::util::TinyString;
use core::ffi::c_void;
use core::mem::{align_of, align_of_val, size_of, size_of_val};

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

macro_rules! ffi_constructor {
    ($name:tt, $(<$($lifetime:lifetime),*>)? ($($arg_name:ident: $args:ty),*) -> $retty:ty) => {
        /// Выделяет память для объекта, используя переданный аллокатор,
        /// а затем инициализирует объект и возвращает на него указатель.
        ///
        /// Аллокатор должен возвращать корректно выровненный указатель на блок памяти
        /// достаточного размера. Нарушение любого из этих условий — неопределённое поведение.
        ///
        /// Размер и выравнивание передаются в качестве аргументов аллокатору.
        #[no_mangle]
        pub unsafe extern "C" fn $name $($(<$lifetime>),*)?(
            $($arg_name: $args,)*
            allocator_context: AllocatorContext,
            allocator: Allocator
        ) -> *mut $retty {
            use core::ptr::{null_mut, write};

            let memory = allocator(
                allocator_context,
                size_of::<$retty>(),
                align_of::<$retty>()
            ) as *mut $retty;
            if memory.is_null() {
                return null_mut();
            }

            write(memory, <$retty>::new($($arg_name),*));

            memory
        }
    };
}

macro_rules! ffi_destructor {
    ($name:ident, ($arg:ident: $ty:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            $arg: *mut $ty,
            deallocator_context: AllocatorContext,
            deallocator: Deallocator,
        ) {
            if $arg.is_null() {
                return;
            }
            $arg.drop_in_place();
            deallocator(
                deallocator_context,
                $arg as *mut c_void,
                size_of_val(&*$arg),
            )
        }
    };
}

ffi_constructor!(mmheroes_game_create, (mode: GameMode, seed: u64) -> Game);
ffi_destructor!(mmheroes_game_destroy, (game: Game));

/// Записывает текущий игровой день и время в аргументы `out_day` и `out_time`
/// и возвращает `true` если они доступны, иначе не трогает аргументы и возвращает
/// `false`.
///
/// Игровой день и время могут быть недоступны, например, если игра ещё не началась.
#[no_mangle]
pub extern "C" fn mmheroes_game_get_current_time(
    game: &mut Game,
    out_day: &mut u8,
    out_time: &mut Time,
) -> bool {
    if let Some(state) = game.screen().state() {
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
pub unsafe extern "C" fn mmheroes_game_ui_create(
    game: &mut Game,
    high_scores: *const FfiHighScore,
    allocator_context: AllocatorContext,
    allocator: Allocator,
    renderer_request_callback_context: *mut c_void,
    renderer_request_callback: RendererRequestCallback,
) -> *mut GameUI<FfiRendererRequestConsumer> {
    use core::ptr::{null_mut, write};

    let scores = if high_scores.is_null() {
        None
    } else {
        let mut scores = crate::ui::high_scores::default_high_scores();
        let slice = core::slice::from_raw_parts(high_scores, high_scores::SCORE_COUNT);
        for (i, score) in slice.iter().enumerate() {
            let name_buf = core::slice::from_raw_parts(score.name, score.name_len);
            let name = core::str::from_utf8(name_buf).expect("Name is not valid UTF-8");
            scores[i].0 = TinyString::from(name);
            scores[i].1 = score.score;
        }
        Some(scores)
    };

    let renderer_request_consumer = FfiRendererRequestConsumer {
        context: renderer_request_callback_context,
        callback: renderer_request_callback,
    };

    let game = GameUI::new(game, scores, renderer_request_consumer);

    let memory: *mut GameUI<_> =
        allocator(allocator_context, size_of_val(&game), align_of_val(&game)) as *mut _;
    if memory.is_null() {
        return null_mut();
    }

    write(memory, game);

    memory
}

ffi_destructor!(
    mmheroes_game_ui_destroy,
    (game_ui: GameUI<FfiRendererRequestConsumer>)
);

/// Записывает в аргумент `out` `MMHEROES_SCORE_COUNT` элементов.
/// `out` не должен быть нулевым указателем.
/// Результат, записанный в `out`, не должен жить дольше, чем экземпляр
/// соответствующего `GameUI`.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_ui_get_high_scores(
    game_ui: &GameUI<FfiRendererRequestConsumer>,
    out: *mut FfiHighScore,
) {
    assert!(!out.is_null());
    let out_slice = core::slice::from_raw_parts_mut(out, SCORE_COUNT);
    for (high_score, out) in game_ui.high_scores.iter().zip(out_slice.iter_mut()) {
        *out = FfiHighScore {
            name: high_score.0.as_ptr(),
            name_len: high_score.0.len(),
            score: high_score.1,
        }
    }
}
/// `new_high_scores` — ненулевой указатель на массив из `MMHEROES_SCORE_COUNT` элементов.
#[no_mangle]
pub unsafe extern "C" fn mmheroes_game_ui_set_high_scores(
    game_ui: &mut GameUI<FfiRendererRequestConsumer>,
    new_high_scores: *const FfiHighScore,
) {
    assert!(!new_high_scores.is_null());
    let new_high_scores = core::slice::from_raw_parts(new_high_scores, SCORE_COUNT);
    for (high_score, new_high_score) in
        game_ui.high_scores.iter_mut().zip(new_high_scores.iter())
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
    game_ui: &mut GameUI<FfiRendererRequestConsumer>,
    recorded_input: *const u8,
    recorded_input_len: usize,
) -> bool {
    assert!(!recorded_input.is_null());
    let slice = core::slice::from_raw_parts(recorded_input, recorded_input_len);
    let s = match core::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let mut parser = recording::InputRecordingParser::new(s);
    parser
        .parse_all(|input| game_ui.continue_game(input))
        .is_ok()
}

/// Продолжает игру до следующего запроса на нажатие клавиши.
///
/// При первом вызове этой функции неважно, что передаётся в параметре `input`.
#[no_mangle]
pub extern "C" fn mmheroes_continue(
    game_ui: &mut GameUI<FfiRendererRequestConsumer>,
    input: Input,
) -> bool {
    game_ui.continue_game(input)
}

#[repr(C)]
pub struct InputRecorderSink {
    context: *mut c_void,
    sink: fn(*mut c_void, *const u8, usize) -> bool,
}

impl core::fmt::Write for InputRecorderSink {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if (self.sink)(self.context, s.as_ptr(), s.len()) {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

ffi_constructor!(
    mmheroes_input_recorder_create,
    <'a> (sink: &'a mut InputRecorderSink) -> InputRecorder<'a, InputRecorderSink>);

ffi_destructor!(
    mmheroes_input_recorder_destroy,
    (recorder: InputRecorder<InputRecorderSink>)
);

#[no_mangle]
pub unsafe extern "C" fn mmheroes_input_recorder_record(
    recorder: &mut InputRecorder<InputRecorderSink>,
    input: Input,
) -> bool {
    recorder.record_input(input).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn mmheroes_input_recorder_flush(
    recorder: &mut InputRecorder<InputRecorderSink>,
) -> bool {
    recorder.flush().is_ok()
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

    #[test]
    fn test_ffi() {
        unsafe {
            let game = mmheroes_game_create(GameMode::Normal, 0, null_mut(), allocator);

            let scores = high_scores();

            let mut requests = <Vec<FfiRendererRequest>>::new();

            extern "C" fn renderer_request_callback(
                context: *mut c_void,
                renderer_request: FfiRendererRequest,
            ) {
                unsafe { (&mut *(context as *mut Vec<_>)).push(renderer_request) }
            }

            let game_ui = mmheroes_game_ui_create(
                &mut *game,
                scores.as_ptr(),
                null_mut(),
                allocator,
                &mut requests as *mut _ as *mut c_void,
                renderer_request_callback,
            );

            let scores = &(&*game_ui).high_scores;
            assert_eq!(scores[0].0, "Оля");
            assert_eq!(scores[1].0, "Вероника");
            assert_eq!(scores[2].0, "Наташа");
            assert_eq!(scores[3].0, "Катя");
            assert_eq!(scores[4].0, "Рита");

            mmheroes_continue(&mut *game_ui, Input::Enter);

            assert_eq!(requests.len(), 29);

            let mut day = 255u8;
            let mut time = Time(255);
            assert!(!mmheroes_game_get_current_time(
                &mut *game, &mut day, &mut time
            ));

            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::Enter);
            mmheroes_continue(&mut *game_ui, Input::KeyDown);
            mmheroes_continue(&mut *game_ui, Input::KeyDown);
            mmheroes_continue(&mut *game_ui, Input::Enter);

            assert!(mmheroes_game_get_current_time(
                &mut *game, &mut day, &mut time
            ));
            assert_eq!(day, 0);
            assert_eq!(time, Time(9));

            mmheroes_game_ui_destroy(game_ui, null_mut(), deallocator);

            mmheroes_game_destroy(game, null_mut(), deallocator);
        }
    }
}
