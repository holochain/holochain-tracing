
use std::fmt;
use std::cell::RefCell;
// use syn::{self, ItemFn};

use crate::Span;

thread_local! {
    static SPANSTACK: RefCell<SpanStack> = RefCell::new(SpanStack::default());
}

type PushFn = FnOnce(&Span) -> Span;

#[derive(Default)]
struct SpanStack(Vec<Span>);

impl SpanStack {

    pub fn new(span: Span) -> Self {
        Self(vec![span])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn push<F: FnOnce(&Span) -> Span>(&mut self, f: F) {
        if let Some(top) = self.0.last() {
            self.0.push(f(top));
        }
    }

    fn pop(&mut self) -> Option<Span>{
        self.0.pop()
    }

    fn top(&mut self) -> Option<&Span>{
        self.0.last()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// impl syn::fold::Fold for SpanStack {
//     fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
//         i
//     }
// }

#[derive(Debug)]
pub enum SpanStackError {
    StackNotEmpty
}

impl std::error::Error for SpanStackError {}

impl fmt::Display for SpanStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn set_root_span(span: Span) {
    SPANSTACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        *stack = SpanStack::new(span);
        // if !stack.is_empty() {
        //     Err(SpanStackError::StackNotEmpty)
        // } else {
        //     stack.push(span);
        //     Ok(())
        // }
    })
}

// pub fn current_span() -> &Span {
//     SPANSTACK.with(|stack| {
//         stack.borrow().as_ref().top()
//     })
// }

pub fn nested<F, G, T>(f: F, g: G) -> T
where F: FnOnce(&Span) -> Span, G: FnOnce() -> T {
    SPANSTACK.with(|stack| {
        stack.borrow_mut().push(f);
    });
    let result = g();
    SPANSTACK.with(|stack| {
        let _ = stack.borrow_mut().pop();
    });
    result
}

// pub fn push<F>(f: F) 
// where F: FnOnce(&Span) -> Span {
//     SPANSTACK.with(|stack| stack.borrow_mut().as_mut().and_then(|s| s.top().map(|t| s.push(f(t)))));
// }

// pub fn pop() -> Option<Span> {
//     SPANSTACK.with(|stack| stack.borrow_mut().as_mut().and_then(|s| s.pop()))
// }

// pub fn is_empty() -> bool {
//     SPANSTACK.with(|stack| stack.borrow().as_ref().map(|s| s.is_empty()).unwrap_or_default())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_nested() {
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 0));
        set_root_span(Span::noop());
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
        let inner = nested(|s| { println!("{:?}", s); s.child("1") }, || {
            let two = SPANSTACK.with(|stack| stack.borrow().len());
            let three = nested(|s| { println!("{:?}", s); s.child("2") }, || {
                SPANSTACK.with(|stack| stack.borrow().len())
            });
            let two = SPANSTACK.with(|stack| stack.borrow().len());
            (two, three, two)
        });
        assert_eq!(inner, (2, 3, 2));
        SPANSTACK.with(|stack| assert_eq!(stack.borrow().len(), 1));
    }
}