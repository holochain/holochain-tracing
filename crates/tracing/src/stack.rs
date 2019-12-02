//! This module is centered around a thread-local Stack of Span references,
//! with some public functions for pushing and accessing values from the stack.
//! This is primarily used by the autotrace proc macro in holochain_tracing_macros,
//! to automatically push new child spans onto the stack when entering a new function frame.
//! Using functions like `with_top` allow the user to access the stack directly, for situations
//! like needing to take a span context and send it into another thread, or out of the process entirely.

use std::cell::RefCell;
use std::rc::Rc;

use crate::Span;

/// This enum defines how to handle situations where we expect there to be a Span
/// on the stack, but there is none.
#[allow(dead_code)]
enum Mode {
    /// Panic when finding an empty stack. Useful for quickly discovering gaps in tracing coverage
    Panic,
    /// Emit a warning and a full backtrace. Note, this is very noisy and slow!
    Backtrace,
    /// Ignore cases of an empty stack, and just return a null span.
    Noop,
}

const MODE: Mode = Mode::Noop;

thread_local! {
    static SPANSTACK: RefCell<SpanStack> = RefCell::new(SpanStack::default());
}

lazy_static! {
    static ref NOOP_SPAN: Span = Span::noop();
}

#[derive(Default)]
struct SpanStack(Vec<Rc<Span>>);

impl SpanStack {
    // fn len(&self) -> usize {
    //     self.0.len()
    // }

    fn push_span(&mut self, span: Rc<Span>) {
        self.0.push(span);
    }

    // fn push_fn<F: FnOnce(&Span) -> Rc<Span>>(&mut self, f: F) {
    //     if let Some(top) = self.0.last() {
    //         let successor = f(top);
    //         self.0.push(successor);
    //     } else {
    //         warn!("Using push_fn, but the stack is empty!");
    //     }
    // }

    fn pop(&mut self) {
        let _ = self.0.pop();
    }

    fn top(&self) -> Option<&Span> {
        self.0.last().map(|s| (*s).as_ref())
    }

    // fn is_empty(&self) -> bool {
    //     self.0.is_empty()
    // }
}

/// A guard to track the lifetime of an item on the stack. Items are popped from the stack
/// when these guards are dropped.
/// Each guard corresponds to a single item in the stack, and contains Rc references
/// to each item below this item in the stack.
/// This is such that if a guard for an item is dropped, the item will not be popped from the stack
/// if there are still guards from higher-up items still on the stack.
pub struct SpanStackGuard {
    _spans: Vec<Rc<Span>>,
}

impl SpanStackGuard {
    pub fn new(span: Span) -> Self {
        let span = Rc::new(span);
        let _spans = SPANSTACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            stack.push_span(span.clone());
            stack.0.clone()
        });
        Self { _spans }
    }
}

impl Drop for SpanStackGuard {
    fn drop(&mut self) {
        SPANSTACK.with(|stack| stack.borrow_mut().pop());
    }
}

fn handle_empty_stack(msg: &'static str) {
    match MODE {
        Mode::Panic => panic!(msg),
        Mode::Backtrace => {
            warn!("{}, backtrace:\n{:?}", msg, backtrace::Backtrace::new());
        }
        Mode::Noop => (),
    };
}

/// Push a span onto the stack. The value will automatically be popped when the returned guard
/// is dropped, as well as the guards of any subsequently pushed spans
pub fn push_span(span: Span) -> SpanStackGuard {
    SpanStackGuard::new(span)
}

/// Applies a function to the top of the span stack and pushes the value onto the stack.
/// If the stack is empty, the function will not be executed and None will be returned.
pub fn push_span_with<F: FnOnce(&Span) -> Span>(f: F) -> Option<SpanStackGuard> {
    let maybe_guard = SPANSTACK
        .with(|stack| stack.borrow().top().map(f))
        .map(SpanStackGuard::new);
    if maybe_guard.is_none() {
        handle_empty_stack("Using push_span_with but the span stack is empty! Using noop span.");
    }
    maybe_guard
}

/// If the stack is not empty, return the top item, else return None
pub fn with_top<A, F: FnOnce(Option<&Span>) -> Option<A>>(f: F) -> Option<A> {
    SPANSTACK.with(|stack| f(stack.borrow().top()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_push() {
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 0));
        {
            let _g0 = push_span(Span::noop());
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 1));
            {
                let _g1 = push_span_with(|s| s.child("1"));
                SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 2));
                {
                    let _g2 = push_span_with(|s| s.child("2"));
                    SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 3));
                }
                SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 2));
            }
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 1));
        }
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().0.len(), 0));
    }
}
