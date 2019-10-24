use rustracing_jaeger::span::SpanContextState;
use rustracing::span::{SpanReference::*, FinishedSpan as RtFinishedSpan};
use std::collections::{HashMap, BTreeMap};


pub type FinishedSpan = RtFinishedSpan<SpanContextState>;
pub type SpanMap = std::collections::HashMap<u64, FinishedSpan>;

/// Print a single span
pub fn print_span(span_map: &SpanMap, span: &FinishedSpan) {
    let span_id = span.context().state().span_id();
    let (span_depth, _span_offset) = get_span_position(span_map, span_id).expect("span not part of span_map");
    let mut spacing = String::new();
    for _ in 0..span_depth {
        spacing.push_str("\t");
    }
    let mut tags = String::new();
    for tag in span.tags() {
        tags.push_str(&format!("{{{} = {:?}}} ", tag.name(), tag.value()));
    }
    println!("{}[{}] {}", spacing, span.operation_name(), tags);
    for log in span.logs() {
        for field in log.fields() {
            println!("{}!{}! {}", spacing, field.name(), field.value());
        }
    }
}

/// Print a single span with it's hierachy
pub fn print_span_stack(span_map: &SpanMap, span_id: u64) {
    let maybe_span = span_map.get(&span_id);
    let span = match maybe_span {
        None => return,
        Some(s) => s,
    };
    for span_ref in span.references() {
        match span_ref {
            ChildOf(parent) => {
                print_span_stack(span_map, parent.span_id());
            }
            FollowsFrom(sibling) => {
                print_span_stack(span_map, sibling.span_id());
            }
        }
    }
    print_span(span_map, span);
}

/// Print the span_map as a tree
pub fn print_span_map(span_map: &SpanMap) {
    // Create children & sibling map
    let mut sibling_map = HashMap::new();
    let mut children_map = HashMap::new();
    let mut root_tree = BTreeMap::new();
    for (span_id, span) in span_map {
        let (span_depth, span_offset) = get_span_position(span_map, *span_id).expect("span not part of span_map");
        if span_depth == 0 {
            root_tree.insert(span.start_time(), span_id);
            continue;
        }
        for span_ref in span.references() {
            match span_ref {
                ChildOf(parent) => {
                    let maybe_tree = children_map.remove(&parent.span_id());
                    let mut tree = maybe_tree.unwrap_or( BTreeMap::new());
                    tree.insert(span.start_time(), span_id);
                    //println!("Parent of {} has children: {:?}", span.operation_name(), tree);
                    children_map.insert(parent.span_id(), tree);
                }
                FollowsFrom(sibling) => {
                    let maybe_tree = sibling_map.remove(&sibling.span_id());
                    let mut tree = maybe_tree.unwrap_or( BTreeMap::new());
                    tree.insert(span_offset, span_id);
                    sibling_map.insert(sibling.span_id(), tree);
                }
            }
        }
    }
    // print tree
    for (span_offset, span_id) in root_tree.iter() {
        print_span_tree(&children_map, &sibling_map, span_map, **span_id);
    }
}

fn print_span_tree(
    children_map: &HashMap<u64, BTreeMap<std::time::SystemTime, &u64>>,
    sibling_map: &HashMap<u64, BTreeMap<u32, &u64>>,
    span_map: &SpanMap,
    span_id: u64,
) {
    // print self
    let span = span_map.get(&span_id).expect("Span not found");
    print_span(span_map, span);
    // print children
    let maybe_children_tree = children_map.get(&span_id);
    if let Some(children_tree) = maybe_children_tree {
        for (_start_time, child_span_id) in children_tree.iter() {
            print_span_tree(children_map, sibling_map, span_map, **child_span_id);
        }
    }
    // print siblings
    let maybe_siblings_tree = sibling_map.get(&span_id);
    if let Some(sibling_tree) = maybe_siblings_tree {
        for (_sibling_offset, sibling_span_id) in sibling_tree.iter() {
            print_span_tree(children_map, sibling_map, span_map, **sibling_span_id);
        }
    }
}

/// gives the depth and sibling offset of a span
pub fn get_span_position(span_map: &SpanMap, span_id: u64) -> Option<(u32, u32)> {
    let maybe_span = span_map.get(&span_id);
    let span = match maybe_span {
        None => return None,
        Some(s) => s,
    };
    let mut depth = 0;
    let mut offset = 0;
    for span_ref in span.references() {
        match span_ref {
            ChildOf(parent) => {
                if let Some((parent_depth, _parent_offset)) = get_span_position(span_map, parent.span_id()) {
                    depth = parent_depth + 1;
                }
            }
            FollowsFrom(sibling) => {
                if let Some((sibling_depth, sibling_offset)) = get_span_position(span_map, sibling.span_id()) {
                    depth = sibling_depth;
                    offset = sibling_offset + 1;
                }
            }
        }
    }
    Some((depth, offset))
}
