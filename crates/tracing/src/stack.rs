//! This module is centered around a thread-local Stack of Span references,
//! with some public functions for pushing and accessing values from the stack.
//! This is primarily used by the autotrace proc macro in holochain_tracing_macros,
//! to automatically push new child spans onto the stack when entering a new function frame.
//! Using functions like `with_top` allow the user to access the stack directly, for situations
//! like needing to take a span context and send it into another thread, or out of the process entirely.

use std::cell::RefCell;
use std::collections::BTreeSet;
use crate::span;
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

/// Internal representation of a stack of Rc<Span>
/// Keep this private! We're doing some careful management of Rc lifetimes here,
/// it would be a shame if these Rc's were to leak out, destroying the guarantees
/// of the span stack.
#[derive(Default)]
struct SpanStack {
    stack: Vec<Span>,
    guards: BTreeSet<usize>,
}

impl SpanStack {
    fn push_span(&mut self, span: Span) -> usize {
        let index = self.stack.len();
        self.guards.insert(index);
        self.stack.push(span);
        index
    }

    /// Finds the size of the stack disregarding items on top which no longer
    /// have associated SpanStackGuards.
    fn live_length(&self) -> usize {
        self.guards.iter().next_back().map(|i| i + 1).unwrap_or(0)
    }

    fn prune(&mut self, index: usize) {
        self.guards.remove(&index);
        let new_len = self.live_length();
        while self.stack.len() > new_len {
            self.stack.pop();
        }
    }

    fn top(&mut self) -> Option<&mut Span> {
        self.stack.last_mut()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.stack.len()
    }
}

/// A guard to track the lifetime of an item on the stack. Items are popped from the stack
/// when these guards are dropped.
/// Each guard corresponds to a single item in the stack, and contains Rc references
/// to each item below this item in the stack.
/// This is such that if a guard for an item is dropped, the item will not be popped from the stack
/// if there are still guards from higher-up items still on the stack.
pub struct SpanStackGuard {
    index: usize,
}

impl SpanStackGuard {
    pub fn new(span: Span) -> Self {
        let index = SPANSTACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            let index = stack.push_span(span);
            index
        });
        Self { index }
    }
}

impl Drop for SpanStackGuard {
    fn drop(&mut self) {
        SPANSTACK.with(|stack| stack.borrow_mut().prune(self.index));
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
pub fn push_span_with<F: FnOnce(&mut Span) -> Span>(f: F) -> Option<SpanStackGuard> {
    let maybe_guard = SPANSTACK
        .with(|stack| stack.borrow_mut().top().map(f))
        .map(SpanStackGuard::new);
    if maybe_guard.is_none() {
        handle_empty_stack("Using push_span_with but the span stack is empty! Using noop span.");
    }
    maybe_guard
}

/// If the stack is not empty, return the top item, else return None
pub fn with_top<A, F: FnOnce(&mut Span) -> A>(f: F) -> Option<A> {
    SPANSTACK.with(|stack| stack.borrow_mut().top().map(f))
}

/// If the stack is not empty, return the top item, else return None
pub fn with_top_or_null<A, F: FnOnce(&mut Span) -> A>(f: F) -> A {
    SPANSTACK.with(|stack| {
        match stack.borrow_mut().top() {
            Some(top) => f(top),
            None => {
                handle_empty_stack("Using with_top but the span stack is empty! Using noop span.");
                f(&mut Span::noop())
            }
        }
    })
}


/// If the stack is not empty, return the top item, else return None
pub fn top_follower<S: Into<std::borrow::Cow<'static, str>>>(name: S) -> Span {
    SPANSTACK.with(|stack| {
        stack
            .borrow_mut()
            .top()
            .map(|s| s.follower(name))
            .unwrap_or_else(|| {
                handle_empty_stack("Using with_top but the span stack is empty! Using noop span.");
                span::NOOP_SPAN.follower("noop")
            })
    })
}

pub fn is_empty() -> bool {
    SPANSTACK.with(|stack| stack.borrow().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    fn with_stack<A, F: FnOnce(&mut SpanStack) -> A>(f: F) -> A {
        SPANSTACK.with(|s| f(&mut s.borrow_mut()))
    }

    #[test]
    fn test_push() {
        with_stack(|stack| assert_eq!(stack.len(), 0));
        {
            let _g0 = push_span(Span::noop());
            with_stack(|stack| assert_eq!(stack.len(), 1));
            {
                let _g1 = push_span_with(|s| s.child("1"));
                with_stack(|stack| assert_eq!(stack.len(), 2));
                {
                    let _g2 = push_span_with(|s| s.child("2"));
                    with_stack(|stack| assert_eq!(stack.len(), 3));
                }
                with_stack(|stack| assert_eq!(stack.len(), 2));
            }
            with_stack(|stack| assert_eq!(stack.len(), 1));
        }
        with_stack(|stack| assert_eq!(stack.len(), 0));
    }

    #[test]
    fn test_weird_drops() {
        with_stack(|stack| assert_eq!(stack.len(), 0));
        {
            let _g0 = push_span(Span::noop());
            with_stack(|stack| assert_eq!(stack.len(), 1));
            {
                let _g1 = push_span_with(|s| s.child("1"));
                with_stack(|stack| assert_eq!(stack.len(), 2));
                {
                    let _g2 = push_span_with(|s| s.child("2"));
                    with_stack(|stack| assert_eq!(stack.len(), 3));
                    {
                        let _g3 = push_span_with(|s| s.child("3"));
                        with_stack(|stack| assert_eq!(stack.len(), 4));
                        drop(_g0);
                        with_stack(|stack| assert_eq!(stack.len(), 4));
                        drop(_g2);
                        with_stack(|stack| assert_eq!(stack.len(), 4));
                    }
                }
                with_stack(|stack| assert_eq!(stack.len(), 2));
            }
            with_stack(|stack| assert_eq!(stack.len(), 0));
        }
        with_stack(|stack| assert_eq!(stack.len(), 0));
    }

}
