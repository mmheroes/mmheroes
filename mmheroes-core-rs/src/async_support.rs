use core::any::Any;
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, ContextBuilder, Poll, RawWaker, RawWakerVTable, Waker};

// TODO: Использовать функцию Waker::noop() когда она стабилизируется
const NOOP: RawWaker = {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        // Cloning just returns a new no-op raw waker
        |_| NOOP,
        // `wake` does nothing
        |_| {},
        // `wake_by_ref` does nothing
        |_| {},
        // Dropping does nothing as we don't allocate anything
        |_| {},
    );
    RawWaker::new(core::ptr::null(), &VTABLE)
};

fn make_noop_waker() -> Waker {
    unsafe { Waker::from_raw(NOOP) }
}

pub(crate) struct AwaitingInputExecutor<'a, F, Input> {
    future: Pin<&'a mut F>,
    phantom_data: PhantomData<Input>,
}

impl<'a, Result, Input: Any, F: Future<Output = Result>>
    AwaitingInputExecutor<'a, F, Input>
{
    pub(crate) fn new(pinned_future: Pin<&'a mut F>) -> Self {
        Self {
            future: pinned_future,
            phantom_data: PhantomData,
        }
    }

    pub(crate) fn resume(&mut self, input: Input) -> Poll<Result> {
        let waker = make_noop_waker();
        let mut wrapped_input = Some(input);
        let mut context = ContextBuilder::from_waker(&waker)
            .ext(&mut wrapped_input)
            .build();
        self.future.as_mut().poll(&mut context)
    }
}

struct ReadInputFuture<T> {
    _phantom: PhantomData<T>,
}

impl<T: Any> Future for ReadInputFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match cx.ext().downcast_mut::<Option<T>>() {
            Some(input) => input
                .take()
                .map(|t| Poll::Ready(t))
                .unwrap_or(Poll::Pending),
            _ => Poll::Pending,
        }
    }
}

pub(crate) async fn read_input<T: Any>() -> T {
    ReadInputFuture::<T> {
        _phantom: PhantomData::default(),
    }
    .await
}

#[cfg(test)]
mod tests {
    use crate::async_support::{read_input, AwaitingInputExecutor};
    use assert_matches::assert_matches;
    use core::cell::RefCell;
    use core::pin::pin;
    use core::task::Poll;

    #[test]
    fn test_basic() {
        let input_collector = RefCell::new(Vec::new());
        let future = pin!(async {
            input_collector.borrow_mut().push(">");
            for _ in 0..2 {
                let input = read_input::<&str>().await;
                input_collector.borrow_mut().push(input);
            }
            read_input::<&str>().await
        });
        let mut executor = AwaitingInputExecutor::new(future);
        assert_eq!(input_collector.borrow().as_slice(), &[] as &[&str]);
        assert_matches!(executor.resume("Hello"), Poll::Pending);
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello"]);
        assert_matches!(executor.resume("World"), Poll::Pending);
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
        assert_matches!(executor.resume("!"), Poll::Ready("!"));
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
    }

    #[test]
    #[should_panic]
    fn test_panics_if_resumed_after_finishing() {
        let future = pin!(async { 42 });
        let mut executor = AwaitingInputExecutor::new(future);
        let _ = executor.resume(());
        let _ = executor.resume(());
    }
}
