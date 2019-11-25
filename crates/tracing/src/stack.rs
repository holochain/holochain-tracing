
// use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
// use syn::{self, ItemFn};

use crate::Span;

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

    fn pop(&mut self) -> Option<Rc<Span>> {
        self.0.pop()
    }

    fn top(&self) -> Option<&Span> {
        self.0.last().map(|s| (*s).as_ref())
    }

    // fn is_empty(&self) -> bool {
    //     self.0.is_empty()
    // }
}


pub struct SpanStackGuard {
    _span: Rc<Span>
}

impl SpanStackGuard {
    pub fn new(span: Span) -> Self {
        let span = Rc::new(span);
        SPANSTACK.with(|stack| {
            stack.borrow_mut().push_span(span.clone());
        });
        Self { _span: span }
    }
}

impl Drop for SpanStackGuard {
    fn drop(&mut self) {
        SPANSTACK.with(|stack| {
            stack.borrow_mut().pop()
        });
    }
}

pub fn push_root_span(span: Span) -> SpanStackGuard {
    SpanStackGuard::new(span)
}

pub fn push_span_with<F: FnOnce(&Span) -> Span>(f: F) -> SpanStackGuard {
    let new_span = SPANSTACK.with(|stack| {
        stack.borrow().top().map(f).unwrap_or_else(|| {
            warn!("Using push_span_with but the span stack is empty! Using noop span.");
            NOOP_SPAN.child("hey")
        })
    });
    SpanStackGuard::new(new_span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_push() {
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 0));
        {
            let g0 = push_root_span(Span::noop());
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
            {
                let g1 = push_span_with(|s| s.child("1"));
                SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 2));
                {
                    let g2 = push_span_with(|s| s.child("2"));
                    SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 3));
                }
                SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 2));
            }
            SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
        }
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 0));
    }

}