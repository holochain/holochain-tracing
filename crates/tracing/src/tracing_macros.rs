#[cfg(feature = "experimental-jaeger")]
#[macro_export]
macro_rules! follow_span {
    ($level:expr, $context:expr) => {
        if let Some(ref context) = $context {
            //let jaeger_context = ::serde_json::json!(context).to_string();
            let follow_span = ::tracing::span!(
                target: module_path!(),
                parent: None,
                $level,
                "follower",
                jaeger_follower = true
            );
            $crate::tracing::__follow(&follow_span, context);
            follow_span
        } else {
            ::tracing::Span::none()
        }
    };
}

#[cfg(feature = "experimental-jaeger")]
#[macro_export]
macro_rules! span_wrap_encode {
    ($level:expr, $data:expr) => {{
        let span = ::tracing::span!(target: module_path!(), $level, "out_follower");
        let _g = span.enter();
        $crate::tracing::span_context(&span)
            .map(|context| context.wrap(()))
            .unwrap_or_else(|| $crate::SpanWrap::new((), None))
            .map(|_| $data)
    }};
}

#[cfg(not(feature = "experimental-jaeger"))]
#[macro_export]
macro_rules! follow_span {
    ($level:expr, $context:expr) => {{
        let _ = $level;
        let _ = $context;
        ::tracing::Span::none()
    }};
}

#[cfg(not(feature = "experimental-jaeger"))]
#[macro_export]
macro_rules! span_wrap_encode {
    ($level:expr, $data:expr) => {{
        let _ = $level;
        $crate::SpanWrap::new($data, None)
    }};
}
