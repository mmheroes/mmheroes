use crate::ui::{Color, Milliseconds, WaitingState, TERMINAL_WIDTH};

use core::fmt::Write;

#[derive(Debug)]
pub enum RendererRequest<'a> {
    ClearScreen,
    Flush,
    WriteStr(&'a str),
    MoveCursor {
        line: u8,
        column: u8,
    },
    SetColor {
        foreground: Color,
        background: Color,
    },
    Sleep(Milliseconds),
}

pub type Line = u8;

pub type Column = u8;

pub trait RendererRequestConsumer {
    fn consume_request(&mut self, request: RendererRequest);
}

pub(in crate::ui) struct Renderer<C> {
    request_consumer: C,
    line: Line,
    column: Column,
    pub(in crate::ui) waiting_state: Option<WaitingState>,
}

impl<C: RendererRequestConsumer> Renderer<C> {
    pub(in crate::ui) fn new(request_callback: C) -> Self {
        Renderer {
            request_consumer: request_callback,
            line: 0,
            column: 0,
            waiting_state: None,
        }
    }

    pub(in crate::ui) fn request_consumer(&self) -> &C {
        &self.request_consumer
    }

    pub(in crate::ui) fn clear_screen(&mut self) {
        self.column = 0;
        self.line = 0;
        self.request_consumer
            .consume_request(RendererRequest::ClearScreen)
    }

    pub(in crate::ui) fn flush(&mut self) {
        self.request_consumer
            .consume_request(RendererRequest::Flush)
    }

    pub(in crate::ui) fn write_str(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }

        for c in s.chars() {
            if c == '\n' {
                self.column = 0;
                self.line += 1;
            } else {
                if self.column >= TERMINAL_WIDTH as u8 {
                    self.column = 0;
                    self.line += 1;
                }
                self.column += 1;
            }
        }
        self.request_consumer
            .consume_request(RendererRequest::WriteStr(s));
    }

    pub(in crate::ui) fn move_cursor_to(&mut self, line: Line, column: Column) {
        self.line = line;
        self.column = column;
        self.request_consumer
            .consume_request(RendererRequest::MoveCursor { line, column })
    }

    pub(in crate::ui) fn get_cursor_position(&mut self) -> (Line, Column) {
        (self.line, self.column)
    }

    pub(in crate::ui) fn set_color(&mut self, foreground: Color, background: Color) {
        self.request_consumer
            .consume_request(RendererRequest::SetColor {
                foreground,
                background,
            })
    }

    pub(in crate::ui) fn sleep_ms(&mut self, ms: Milliseconds) {
        self.request_consumer
            .consume_request(RendererRequest::Sleep(ms))
    }

    pub(in crate::ui) fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) {
        struct Adapter<'r, C: RendererRequestConsumer>(&'r mut Renderer<C>);

        impl<C: RendererRequestConsumer> core::fmt::Write for Adapter<'_, C> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.0.write_str(s);
                Ok(())
            }
        }

        let mut adapter = Adapter(self);
        adapter.write_fmt(fmt).expect("Invalid format")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RRC {
        strings: Vec<String>,
    }

    impl RRC {
        fn new() -> Self {
            Self {
                strings: Vec::new(),
            }
        }
    }

    impl RendererRequestConsumer for RRC {
        fn consume_request(&mut self, request: RendererRequest) {
            if let RendererRequest::WriteStr(s) = request {
                self.strings.push(String::from(s))
            }
        }
    }

    #[allow(clippy::write_literal)]
    #[test]
    fn test_write() {
        let mut r = Renderer::new(RRC::new());
        write!(r, "Hello, world!\n{} {}", 123, "string");
        assert_eq!(r.request_consumer.strings, ["Hello, world!\n123 string"]);
    }
}
