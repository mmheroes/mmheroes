use core::any::Any;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

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

pub(crate) struct PromptingExecutor<'future_data, F, Input, Output> {
    future: F,
    shared_future_data: &'future_data RefCell<Option<FutureData<Input, Output>>>,
}

impl<
        'future_data,
        Result,
        Input: Any + Unpin,
        Output: Any + Unpin,
        F: Future<Output = Result>,
    > PromptingExecutor<'future_data, F, Input, Output>
{
    pub(crate) fn new(
        future: F,
        shared_future_data: &'future_data RefCell<Option<FutureData<Input, Output>>>,
    ) -> Self {
        Self {
            future,
            shared_future_data,
        }
    }

    pub(crate) fn resume_with_input(
        self: Pin<&mut Self>,
        input: Input,
    ) -> Prompt<Output, Result> {
        self.resume(Some(input))
    }

    #[allow(dead_code)]
    pub(crate) fn resume_without_input(self: Pin<&mut Self>) -> Prompt<Output, Result> {
        self.resume(None)
    }

    pub(crate) fn resume(
        mut self: Pin<&mut Self>,
        maybe_input: Option<Input>,
    ) -> Prompt<Output, Result> {
        let waker = make_noop_waker();

        self.shared_future_data
            .borrow_mut()
            .replace(match maybe_input {
                None => FutureData::NoInput,
                Some(input) => FutureData::Input(input),
            });

        let mut context = Context::from_waker(&waker);

        let pinned_future = unsafe {
            self.as_mut()
                .map_unchecked_mut(|executor| &mut executor.future)
        };
        let poll_result = pinned_future.poll(&mut context);

        let updated_data = self
            .shared_future_data
            .borrow_mut()
            .take()
            .expect("Missing FutureData");

        match poll_result {
            Poll::Ready(result) => Prompt::Completed(result),
            Poll::Pending => match updated_data {
                FutureData::Output(output) => Prompt::WaitingForInput(output),
                FutureData::NoInput => Prompt::AlreadyPrompted,
                _ => {
                    panic!("The future is pending but it didn't set the output")
                }
            },
        }
    }
}

pub(crate) enum FutureData<Input, Output> {
    NoInput,
    Input(Input),
    Output(Output),
}

struct MyFuture<'a, Input, Output> {
    maybe_output: Option<Output>,
    shared_future_data: &'a RefCell<Option<FutureData<Input, Output>>>,
}

impl<Input: Any + Unpin, Output: Any + Unpin> Future for MyFuture<'_, Input, Output> {
    type Output = Input;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Input> {
        let mut borrowed_data = self.shared_future_data.borrow_mut();
        let data = borrowed_data.as_mut().expect("Missing FutureData");
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
}

pub(crate) async fn prompt<Input: Any + Unpin, Output: Any + Unpin>(
    output: Output,
    shared_future_data: &RefCell<Option<FutureData<Input, Output>>>,
) -> Input {
    MyFuture::<Input, Output> {
        maybe_output: Some(output),
        shared_future_data,
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
        let future_data = RefCell::new(None);
        let mut executor = PromptingExecutor::new(
            async {
                input_collector.borrow_mut().push(">");
                for i in 0..2 {
                    let input: &str = prompt(i, &future_data).await;
                    input_collector.borrow_mut().push(input);
                }
                prompt::<&str, i32>(999, &future_data).await
            },
            &future_data,
        );
        let mut pinned_executor = pin!(executor);
        assert_eq!(input_collector.borrow().as_slice(), &[] as &[&str]);
        assert_matches!(
            pinned_executor.as_mut().resume_without_input(),
            Prompt::WaitingForInput(0)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">"]);
        assert_matches!(
            pinned_executor.as_mut().resume_without_input(),
            Prompt::AlreadyPrompted
        );
        assert_eq!(input_collector.borrow().as_slice(), [">"]);
        assert_matches!(
            pinned_executor.as_mut().resume_with_input("Hello"),
            Prompt::WaitingForInput(1)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello"]);
        assert_matches!(
            pinned_executor.as_mut().resume_without_input(),
            Prompt::AlreadyPrompted
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello"]);
        assert_matches!(
            pinned_executor.as_mut().resume_with_input("World"),
            Prompt::WaitingForInput(999)
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
        assert_matches!(
            pinned_executor.as_mut().resume_with_input("!"),
            Prompt::Completed("!")
        );
        assert_eq!(input_collector.borrow().as_slice(), [">", "Hello", "World"]);
    }

    #[test]
    fn premature_input_is_discarded() {
        let input_collector = RefCell::new(Vec::new());
        let future_data = RefCell::new(None);
        let executor = PromptingExecutor::new(
            async {
                let input: &str = prompt(42, &future_data).await;
                input_collector.borrow_mut().push(input);
            },
            &future_data,
        );
        let mut pinned_executor = pin!(executor);
        assert_matches!(
            pinned_executor.as_mut().resume_with_input("Hello"),
            Prompt::WaitingForInput(42)
        );
        assert_eq!(input_collector.borrow().as_slice(), &[] as &[&str]);
        assert_matches!(
            pinned_executor.as_mut().resume_with_input("Hello"),
            Prompt::Completed(())
        );
        assert_eq!(input_collector.borrow().as_slice(), ["Hello"]);
    }

    #[test]
    #[should_panic]
    fn test_panics_if_resumed_after_finishing() {
        let future_data = RefCell::new(None);
        let mut executor =
            PromptingExecutor::<_, (), ()>::new(async { 42 }, &future_data);
        let mut pinned_executor = pin!(executor);
        let _ = pinned_executor.as_mut().resume_with_input(());
        let _ = pinned_executor.as_mut().resume_with_input(());
    }
}
