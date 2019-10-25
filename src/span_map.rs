use rustracing::span::{FinishedSpan as RtFinishedSpan, SpanReference::*};
use rustracing_jaeger::span::SpanContextState;
use std::collections::{BTreeMap, HashMap};

pub type FinishedSpan = RtFinishedSpan<SpanContextState>;
pub type SpanMap = std::collections::HashMap<u64, FinishedSpan>;

pub fn print_span_events(span: &FinishedSpan) {
    for log in span.logs() {
        for field in log.fields() {
            println!("{}", field.value());
        }
    }
}

/// Print a single span
pub fn print_span(span_map: &SpanMap, span: &FinishedSpan, only_events: bool) {
    if only_events {
        print_span_events(span);
    } else {
        let span_id = span.context().state().span_id();
        let (span_depth, _span_offset) =
            get_span_position(span_map, span_id).expect("span not part of span_map");
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
    print_span(span_map, span, false);
}

/// Print the span_map as a tree
pub fn print_span_map(span_map: &SpanMap, only_events: bool) {
    // Create children & sibling map
    let mut sibling_map = HashMap::new();
    let mut children_map = HashMap::new();
    // BTree of root spans by start_time.
    // Many Spans can have same start_time, so tree node stores a list of span_ids.
    let mut root_span_list = BTreeMap::new();
    // For each span, browse span-references to fill children & sibling map
    for (span_id, span) in span_map {
        // Handle root span case first
        let (span_depth, span_offset) =
            get_span_position(span_map, *span_id).expect("span not part of span_map");
        let is_root = span_depth == 0 && span_offset == 0;
        if is_root {
            let maybe_entry = root_span_list.get(&span.start_time());
            let mut span_id_list: Vec<u64> = maybe_entry.unwrap_or(&Vec::new()).to_vec();
            span_id_list.push(*span_id);
            root_span_list.insert(span.start_time(), span_id_list);
        }
        // Browse span-references
        for span_ref in span.references() {
            match span_ref {
                ChildOf(parent) => {
                    let maybe_children_tree = children_map.remove(&parent.span_id());
                    let mut children_tree = maybe_children_tree.unwrap_or(BTreeMap::new());
                    let maybe_entry = children_tree.get(&span.start_time());
                    let mut span_id_list = maybe_entry.unwrap_or(&Vec::new()).to_vec();
                    span_id_list.push(*span_id);
                    children_tree.insert(span.start_time(), span_id_list);
                    //println!("Parent of {} has children: {:?}", span.operation_name(), tree);
                    children_map.insert(parent.span_id(), children_tree);
                }
                FollowsFrom(sibling) => {
                    let maybe_tree = sibling_map.remove(&sibling.span_id());
                    let mut tree = maybe_tree.unwrap_or(BTreeMap::new());
                    let maybe_previous = tree.insert(span_offset, span_id);
                    assert!(maybe_previous.is_none());
                    sibling_map.insert(sibling.span_id(), tree);
                }
            }
        }
    }
    // print span tree
    for (_start_time, span_id_list) in root_span_list.iter() {
        for span_id in span_id_list {
            print_span_tree(&children_map, &sibling_map, span_map, *span_id, only_events);
        }
    }
}

fn print_span_tree(
    children_map: &HashMap<u64, BTreeMap<std::time::SystemTime, Vec<u64>>>,
    sibling_map: &HashMap<u64, BTreeMap<u32, &u64>>,
    span_map: &SpanMap,
    span_id: u64,
    only_events: bool,
) {
    // print self
    let span = span_map.get(&span_id).expect("Span not found");
    print_span(span_map, span, only_events);
    // print children
    let maybe_children_tree = children_map.get(&span_id);
    if let Some(children_tree) = maybe_children_tree {
        for (_start_time, child_span_id_list) in children_tree.iter() {
            for child_span_id in child_span_id_list {
                print_span_tree(children_map, sibling_map, span_map, *child_span_id, only_events);
            }
        }
    }
    // print siblings
    let maybe_siblings_tree = sibling_map.get(&span_id);
    if let Some(sibling_tree) = maybe_siblings_tree {
        for (_sibling_offset, sibling_span_id) in sibling_tree.iter() {
            print_span_tree(children_map, sibling_map, span_map, **sibling_span_id, only_events);
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
                if let Some((parent_depth, _parent_offset)) =
                    get_span_position(span_map, parent.span_id())
                {
                    depth = parent_depth + 1;
                }
            }
            FollowsFrom(sibling) => {
                if let Some((sibling_depth, sibling_offset)) =
                    get_span_position(span_map, sibling.span_id())
                {
                    depth = sibling_depth;
                    offset = sibling_offset + 1;
                }
            }
        }
    }
    Some((depth, offset))
}
