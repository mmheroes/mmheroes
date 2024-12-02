pub mod tiny_vec;
pub(crate) use tiny_vec::*;

pub mod tiny_string;
pub use tiny_string::*;

pub(crate) mod async_support;
pub(crate) mod bitset;

/// В переданной шкале пар [scale] находит первую пару, первый элемент которой строго
/// больше чем [value], и возвращает второй элемент этой пары. Если такая пара не найдена,
/// возвращает [default].
pub(crate) fn assess<'a, 'b: 'a, T: PartialOrd, U>(
    scale: &'a [(T, U)],
    value: &T,
    default: &'b U,
) -> &'a U {
    scale
        .iter()
        .find(|&(bound, _)| bound > value)
        .map_or(default, |(_, assessment)| assessment)
}

pub(crate) fn catch_unwind_mut<
    T,
    F: FnOnce(&mut T) -> R,
    Handler: FnOnce(&mut T, &str) -> R,
    R,
>(
    arg: &mut T,
    f: F,
    handler: Handler,
) -> R {
    #[cfg(feature = "std")]
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(arg))) {
        Ok(result) => result,
        Err(cause) => {
            let cause_str = if let Some(s) = cause.downcast_ref::<String>() {
                &s[..]
            } else if let Some(s) = cause.downcast_ref::<&str>() {
                *s
            } else {
                std::panic::resume_unwind(cause)
            };
            handler(arg, cause_str)
        }
    }
    #[cfg(not(feature = "std"))]
    f(arg)
}
