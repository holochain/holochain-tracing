
use std::fmt;
use std::cell::RefCell;
// use syn::{self, ItemFn};

use crate::Span;

thread_local! {
    static SPANSTACK: RefCell<SpanStack> = RefCell::new(SpanStack::default());
}

lazy_static! {
    static ref NOOP_SPAN: Span = Span::noop();
}

#[derive(Default)]
struct SpanStack(Vec<Span>);

impl SpanStack {

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn push_span(&mut self, span: Span) {
        self.0.push(span);
    }

    fn push_fn<F: FnOnce(&Span) -> Span>(&mut self, f: F) {
        if let Some(top) = self.0.last() {
            let successor = f(top);
            self.0.push(successor);
        } else {
            warn!("Using push_fn, but the stack is empty!");
        }
    }

    fn pop(&mut self) -> Option<Span> {
        self.0.pop()
    }

    fn top(&self) -> Option<&Span> {
        self.0.last()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}


// #[derive(Debug)]
// pub enum SpanStackError {
//     StackNotEmpty
// }

// impl std::error::Error for SpanStackError {}

// impl fmt::Display for SpanStackError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

pub fn with_thread_span<F, T>(f: F) -> T
where F: FnOnce(&Span) -> T {
    SPANSTACK.with(|stack| {
        let stack = stack.borrow();
        let span = stack.top().unwrap_or_else(|| {
            warn!("Using with_thread_span, but no span is active for this thread.");
            &NOOP_SPAN
        });
        f(span)
    })
}

// pub fn new_span<F>(f: F) -> Span
// where F: FnOnce(&Span) -> Span {
//     SPANSTACK.with(|stack| {
//         let stack = stack.borrow();
//         let span = stack.top().unwrap_or_else(|| {
//             warn!("Using with_thread_span, but no span is active for this thread.");
//             &NOOP_SPAN
//         });
//         f(span)
//     })
// }

pub fn nested<F, G, T>(f: F, g: G) -> T
where F: FnOnce(&Span) -> Span, G: FnOnce() -> T {
    SPANSTACK.with(|stack| {
        stack.borrow_mut().push_fn(f);
    });
    let result = g();
    SPANSTACK.with(|stack| {
        let _ = stack.borrow_mut().pop();
    });
    result
}

pub fn start_thread_trace<G, T>(span: Span, g: G) -> T
where G: FnOnce() -> T {
    SPANSTACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if !stack.is_empty() {
            warn!("Called start_thread_trace, but there were still {} spans left on the stack", stack.len());
        }
        stack.push_span(span);
    });
    let result = g();
    SPANSTACK.with(|stack| {
        let _ = stack.borrow_mut().pop();
    });
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_nested() {
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 0));
        let inner = start_thread_trace(Span::noop(), || {
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
            let inner = nested(|s| s.child("1"), || {
                let one = SPANSTACK.with(|stack| stack.borrow().len());
                let two = nested(|s| s.child("2"), || {
                    SPANSTACK.with(|stack| stack.borrow().len())
                });
                let three = SPANSTACK.with(|stack| stack.borrow().len());
                (one, two, three)
            });
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
            inner
        });
        assert_eq!(inner, (2, 3, 2));
    }

}