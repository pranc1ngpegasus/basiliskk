use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;
use std::io;
use tracing::field::{Field, Visit};
use tracing::{Level, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

#[allow(dead_code)]
pub struct JsonLogLayer<W> {
    make_writer: W,
}

impl<W> JsonLogLayer<W>
where
    W: for<'a> MakeWriter<'a> + 'static,
{
    pub fn new(make_writer: W) -> JsonLogLayer<W> {
        JsonLogLayer { make_writer }
    }
}

impl<S, W> Layer<S> for JsonLogLayer<W>
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    W: for<'a> MakeWriter<'a> + 'static,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let field_set = event.metadata().fields();
        if field_set.field("message").is_none()
            && field_set.field("error").is_none()
            && field_set.field("return").is_none()
        {
            return;
        }

        let mut entry = LogEntry::new(*event.metadata().level());

        match (event.metadata().file(), event.metadata().line()) {
            (Some(file), Some(line)) => {
                entry.caller = format!("{}:{}", file, line);
            }
            (Some(file), None) => {
                entry.caller = file.to_string();
            }
            (_, _) => {}
        };

        event.record(&mut entry);

        let _ = write_json_line(self.make_writer.make_writer(), entry);
    }
}

fn write_json_line(mut w: impl io::Write, entry: impl Serialize) -> io::Result<()> {
    let mut buf = match serde_json::ser::to_vec(&entry) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };
    buf.append(&mut vec![b'\n']);
    w.write_all(&buf)
}

#[serde_as]
#[derive(Serialize)]
struct LogEntry {
    #[serde_as(as = "DisplayFromStr")]
    level: Level,
    message: String,
    #[serde(with = "ts_milliseconds")]
    timestamp: DateTime<Utc>,
    time: String,
    caller: String,
}

impl LogEntry {
    pub fn new(level: Level) -> LogEntry {
        let now = Utc::now();
        LogEntry {
            level,
            message: String::new(),
            timestamp: now,
            time: now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            caller: String::new(),
        }
    }
}

impl Visit for LogEntry {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => {
                self.message = format!("{:?}", value);
            }
            "return" => {
                self.message = format!("{:?}", value);
            }
            "error" => {
                self.message = format!("{:?}", value);
            }
            _ => {}
        }
    }
}
