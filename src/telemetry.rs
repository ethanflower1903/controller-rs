use crate::{Error, Result};

///  Fetch an opentelemetry::trace::TraceId as hex through the full tracing stack
pub fn get_trace_id() -> String {
    use opentelemetry::trace::{SpanContext, TraceContextExt, Tracer};
    use tracing_opentelemetry::OpenTelemetrySpanExt;
    tracing::Span::current().context().span().span_context().trace_id().to_hex()
}
