
use std::fmt;
use std::cell::RefCell;

use crate::Span;

thread_local! {
    static SPANSTACK: RefCell<SpanStack> = RefCell::new(SpanStack::default());
}

#[derive(Default)]
struct SpanStack(Vec<Span>);

impl SpanStack {
    fn push(&mut self, span: Span) {
        self.0.push(span)
    }

    fn pop(&mut self) -> Option<Span>{
        self.0.pop()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug)]
enum SpanStackError {
    StackNotEmpty
}

impl std::error::Error for SpanStackError {};

impl fmt::Display for SpanStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn set_root_span(span: Span) -> Result<(), SpanStackError> {
    SPANSTACK.with(|stack| {
        let stack = stack.get_mut();
        if !stack.is_empty() {
            Err(SpanStackError::StackNotEmpty)
        } else {
            stack.push(span);
            Ok(())
        }
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn asdf() {

    }
}