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

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Prompt<Data, Result> {
    WaitingForInput(Data),
    AlreadyPrompted,
    Completed(Result),
}

pub(crate) struct PromptingExecutor<F, Input, Output> {
    future: F,
    phantom_input: PhantomData<Input>,
    phantom_output: PhantomData<Output>,
}

impl<Result, Input: Any + Unpin, Output: Any + Unpin, F: Future<Output = Result>>
    PromptingExecutor<F, Input, Output>
{
    pub(crate) fn new(future: F) -> Self {
        Self {
            future,
            phantom_input: PhantomData,
            phantom_output: PhantomData,
        }
    }

    pub(crate) fn resume_with_input(
        self: &mut Pin<&mut Self>,
        input: Input,
    ) -> Prompt<Output, Result> {
        self.resume(Some(input))
    }

    pub(crate) fn resume_without_input(
        self: &mut Pin<&mut Self>,
    ) -> Prompt<Output, Result> {
        self.resume(None)
    }

    pub(crate) fn resume(
        self: &mut Pin<&mut Self>,
        maybe_input: Option<Input>,
    ) -> Prompt<Output, Result> {
        let waker = make_noop_waker();
        let mut data = match maybe_input {
            None => FutureData::NoInput,
            Some(input) => FutureData::Input(input),
        };
        let mut context = ContextBuilder::from_waker(&waker).ext(&mut data).build();
        let pinned_future = unsafe {
            self.as_mut()
                .map_unchecked_mut(|executor| &mut executor.future)
        };
        match pinned_future.poll(&mut context) {
            Poll::Ready(result) => Prompt::Completed(result),
            Poll::Pending => match data {
                FutureData::Output(output) => Prompt::WaitingForInput(output),
                FutureData::NoInput => Prompt::AlreadyPrompted,
                _ => {
                    panic!("The future is pending but it didn't set the output")
                }
            },
        }
    }
}

enum FutureData<Input, Output> {
    NoInput,
    Input(Input),
    Output(Output),
}

struct MyFuture<Input, Output> {
    maybe_output: Option<Output>,
    _phantom: PhantomData<Input>,
}

impl<Input: Any + Unpin, Output: Any + Unpin> Future for MyFuture<Input, Output> {
    type Output = Input;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Input> {
        match cx.ext().downcast_mut::<FutureData<Input, Output>>() {
            Some(data) => {
                match self.maybe_output.take() {
                    Some(new_output) => {
                        // The future is being polled for the first time. Serve the output
                        // and discard input if there is one — no input is expected for now.
                        match core::mem::replace(data, FutureData::Output(new_output)) {
                            FutureData::NoInput | FutureData::Input(_) => Poll::Pending,
                            FutureData::Output(_) => unreachable!(),
                        }
                    }
                    None => {
                        // This means that the future is polled NOT for the first time.
                        // We've already served the output.
                        match core::mem::replace(data, FutureData::NoInput) {
                            FutureData::Input(input) => Poll::Ready(input),
                            FutureData::NoInput => Poll::Pending,
                            FutureData::Output(_) => unreachable!(),
                        }
                    }
                }
            }
            _ => panic!("Missing FutureData"),
        }
    }
}

pub(crate) async fn prompt<Input: Any + Unpin, Output: Any + Unpin>(
    output: Output,
) -> Input {
    MyFuture::<Input, Output> {
        maybe_output: Some(output),
        _phantom: PhantomData,
    }
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use core::cell::RefCell;
    use core::pin::pin;

    #[test]
    fn test_basic() {
        let input_collector = RefCell::new(Vec::new());
        let mut executor = PromptingExecutor::new(async {
            input_collector.borrow_mut().push(">");
            for i in 0..2 {
                let input: &str = prompt(i).await;
                input_collector.borrow_mut().push(input);
            }
            prompt::<&str, i32>(999).await
        });
        let mut pinned_executor = pin!(executor);
        assert_eq!(input_collector.borrow().as_slice(), &[] as &[&str]);
        assert_matches!(
            pinned_executor.resume_without_input(),
            Prompt::WaitingForInput(0)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">"]);
        assert_matches!(
            pinned_executor.resume_without_input(),
            Prompt::AlreadyPrompted
        );
        assert_eq!(input_collector.borrow().as_slice(), [">"]);
        assert_matches!(
            pinned_executor.resume_with_input("Hello"),
            Prompt::WaitingForInput(1)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello"]);
        assert_matches!(
            pinned_executor.resume_without_input(),
            Prompt::AlreadyPrompted
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello"]);
        assert_matches!(
            pinned_executor.resume_with_input("World"),
            Prompt::WaitingForInput(999)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
        assert_matches!(
            pinned_executor.resume_with_input("!"),
            Prompt::Completed("!")
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
    }

    #[test]
    fn premature_input_is_discarded() {
        let input_collector = RefCell::new(Vec::new());
        let executor = PromptingExecutor::new(async {
            let input: &str = prompt(42).await;
            input_collector.borrow_mut().push(input);
        });
        let mut pinned_executor = pin!(executor);
        assert_matches!(
            pinned_executor.resume_with_input("Hello"),
            Prompt::WaitingForInput(42)
        );
        assert_eq!(input_collector.borrow().as_slice(), &[] as &[&str]);
        assert_matches!(
            pinned_executor.resume_with_input("Hello"),
            Prompt::Completed(())
        );
        assert_eq!(input_collector.borrow().as_slice(), ["Hello"]);
    }

    #[test]
    #[should_panic]
    fn test_panics_if_resumed_after_finishing() {
        let future = pin!(async { 42 });
        let mut executor = PromptingExecutor::<_, (), ()>::new(future);
        let mut pinned_executor = pin!(executor);
        let _ = pinned_executor.resume_with_input(());
        let _ = pinned_executor.resume_with_input(());
    }
}
