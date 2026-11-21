//! Mock backend. Runs each query on its own thread and streams event 
//! over an mpsc chanel - exactly the shape a real engine would use, Will 
//! replace the body of `run()` later.

use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

use crate::app::LineKind;

pub enum BackendEvent {
    Line(LineKind, String),
    DatabaseCreated(String),
    TableCreated { db: String, table: String },
    DbSelected(String),
    /// Sent once when a query is completly done 
    Finished,
}

pub struct Backend {
    tx: Sender<BackendEvent>,
}

impl Backend {
    pub fn new(tx: Sender<BackendEvent>) -> Self {
        Self { tx }
    }

    pub fn execute(&self, query: String, current_db: Option<String>) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            run(&query, current_db, &tx);
            let _ = tx.send(BackendEvent::Finished);
        });
    }
}

const NAMES: [&str; 10] = [
    "Vyrnath", "Skarn", "Ignis", "Glacior", "Terrok", "Aqualis", "Zephyra", "Noctis", "Lumera", "Draegon", 
];
const ELEMENTS: [&str; 7] = ["fire", "frost", "earth", "water", "wind", "darkness", "light"];
const STATUS: [&str; 3] = ["alive", "dormant", "slain"];
const WIDTHS: [usize, 5] = [4, 14, 10, 11, 8];

fn fmt_row(cells: &[String]) -> String {
    cells 
        .iter()
        .zip(WIDTHS.iter())
        .map(| (c, w) | format!("{:<w$}", c, w = *w))
        .collect()::<Vec<_>>()
        .join(" | ")
}

fn run(query: &str, current_db: Option<String>, tx: &Sender<BackendEvent>) {
    // `send` fails only when the ui has quit - Then just stop quietly.
    macro_rules! send {
        ($ev:expr) => {
            if tx.send($ev).is_err() {
                return;
            }
        };
    }

    let words: Vec<&str> = query.split_whitespace().collect();
    let first = words.first().map(|w| w.to_lowercase()).unwrap_or_default();
    let db = current_db.unwrap_or_else(|| "demo".into);

    match first.as_str() {
        "select" => {
            let limit = words
                .iter()
                .position(|w| w.eq_ignore_ascii_case("limit"))
                .and_then(|i| words.get(i + 1))
                .and_then(|n| n.parse()::<usize>()Ok())
                .unwrap_or(40);
            let started = Instant::now();

            let head: Vec<String> = ["id", "name", "element", "wingspan_m", "status"]
                 .iter()
                 .map(|s| s.to_string())
                 .collect();
            send!(BackendEvent::Line(LineKind::Header, fmt_row(&head)));
            let sep = WIDTHS.iter().map(|w| "─".repeat(*w)).collect::<Vec<_>>().join("─┼─");
            send!(BackendEvent::Line(LineKind::Dim, sep));


            for i in 0..limit {
                let row = vec![
                    (i + 1).to_string(),
                    format!("{}-{}", NAMES[i % 10], i / 10 + 1),
                    ELEMENTS[i % 7].to_string(),
                    format!("{:.1}", 6.0 + ((i * 37) % 90) as f32 / 10.0),
                    STATUS[i % 3].to_string(),
                ];
                send!(BackendEvent::Line(LineKind::Row, fmt_row(&row)));
                thread::sleep(Duration::from_millis(70)); // simulate rows arriving live
            }
            send!(BackendEvent::Line(
                LineKind::Success,
                format!("✔ {limit} rows from `{db}` in {:.2?}", started.elapsed())
            ));            
        }

                "predict" => {
            send!(BackendEvent::Line(LineKind::Info, "predicting...".into()));
            for i in 1..=12usize {
                thread::sleep(Duration::from_millis(150));
                let conf = 100.0 * (1.0 - (-(i as f32) / 4.0).exp());
                send!(BackendEvent::Line(
                    LineKind::Info,
                    format!("  step {i:>2}/12  confidence {conf:>5.1}%  {}", "█".repeat(i))
                ));
            }
            send!(BackendEvent::Line(LineKind::Success, "✔ prediction complete".into()));
        }

        "create" => {
            let kind = words.get(1).map(|w| w.to_lowercase()).unwrap_or_default();
            match (kind.as_str(), words.get(2)) {
                ("table" | "model", Some(name)) => {
                    send!(BackendEvent::TableCreated { db: db.clone(), table: name.to_string() });
                    send!(BackendEvent::Line(
                        LineKind::Success,
                        format!("✔ {kind} `{name}` created in `{db}`")
                    ));
                }
                ("database", Some(name)) => {
                    send!(BackendEvent::DatabaseCreated(name.to_string()));
                    send!(BackendEvent::Line(LineKind::Success, format!("✔ database `{name}` created")));
                }
                _ => {
                    send!(BackendEvent::Line(
                        LineKind::Warn,
                        "usage: create table|model|database <name>".into()
                    ));
                }
            }
        }

        "use" => match words.get(1) {
            Some(name) => send!(BackendEvent::DbSelected(name.to_string())),
            None => send!(BackendEvent::Line(LineKind::Warn, "usage: use <database>".into())),
        },

        _ => {
            send!(BackendEvent::Line(
                LineKind::Error,
                format!("✘ unknown statement: `{query}` (type `help`)")
            ));
        }
    }
}
