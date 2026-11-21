use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Plot, PlotBounds};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct Candle {
    index: usize,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

struct MarketApp {
    candles: VecDeque<Candle>,
    last_tick: Instant,
    tick_count: usize,
    follow_latest: bool,
}

impl Default for MarketApp {
    fn default() -> Self {
        let mut candles = VecDeque::new();
        let mut price = 100.0;
        for i in 0..60 {
            let change = ((i as f64 * 0.3).sin() * 1.5) + ((i % 5) as f64 - 2.0) * 0.4;
            let open = price;
            let close = open + change;
            let high = open.max(close) + 0.6;
            let low = open.min(close) - 0.6;
            candles.push_back(Candle { index: i, open, high, low, close });
            price = close;
        }

        Self {
            candles,
            last_tick: Instant::now(),
            tick_count: 0,
            follow_latest: true,
        }
    }
}

impl MarketApp {
    /// Simulates incoming ticks every 150ms and rolls over to a new candle every 10 ticks.
    fn simulate_market_feed(&mut self) {
        if self.last_tick.elapsed() < Duration::from_millis(150) {
            return;
        }
        self.last_tick = Instant::now();
        self.tick_count += 1;

        let delta = ((self.tick_count as f64 * 0.7).cos()) * 0.45;

        if self.tick_count % 10 == 0 {
            // Start a new candle
            let last_close = self.candles.back().map(|c| c.close).unwrap_or(100.0);
            let next_idx = self.candles.back().map(|c| c.index + 1).unwrap_or(0);
            let close = last_close + delta;
            self.candles.push_back(Candle {
                index: next_idx,
                open: last_close,
                high: last_close.max(close),
                low: last_close.min(close),
                close,
            });
            // Keep memory bounded
            if self.candles.len() > 500 {
                self.candles.pop_front();
            }
        } else if let Some(last) = self.candles.back_mut() {
            // Update active candle in-place
            last.close += delta;
            last.high = last.high.max(last.close);
            last.low = last.low.min(last.close);
        }
    }
}

impl eframe::App for MarketApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.simulate_market_feed();
        // Request continuous repaints for live streaming data
        ui.request_repaint_after(Duration::from_millis(50));

        egui::Panel::top("controls").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Live Candlestick Chart");
                ui.separator();
                ui.checkbox(&mut self.follow_latest, "Auto-scroll to latest");
                if let Some(last) = self.candles.back() {
                    ui.separator();
                    ui.label(format!("Last Price: ${:.2}", last.close));
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let mut bull_boxes = Vec::new();
            let mut bear_boxes = Vec::new();

            let bull_color = Color32::from_rgb(38, 166, 154);
            let bear_color = Color32::from_rgb(239, 83, 80);

            for c in &self.candles {
                let is_bull = c.close >= c.open;
                let (body_low, body_high) = if is_bull {
                    (c.open, c.close)
                } else {
                    (c.close, c.open)
                };
                let color = if is_bull { bull_color } else { bear_color };

                // BoxSpread: (lower_whisker, quartile1, median, quartile3, upper_whisker)
                let elem = BoxElem::new(
                    c.index as f64,
                    BoxSpread::new(c.low, body_low, c.close, body_high, c.high),
                )
                .box_width(0.65)
                .whisker_width(0.0) // 0.0 turns the boxplot T-whiskers into clean vertical wicks
                .fill(color)
                .stroke(Stroke::new(1.5, color));

                if is_bull {
                    bull_boxes.push(elem);
                } else {
                    bear_boxes.push(elem);
                }
            }

            let plot = Plot::new("candlestick_plot")
                .allow_drag(true)
                .allow_zoom(true)
                .y_axis_formatter(|mark, _range| format!("${:.2}", mark.value));

            plot.show(ui, |plot_ui| {
                // Disable auto-follow if the user manually drags the chart
                if plot_ui.response().dragged() {
                    self.follow_latest = false;
                }

                let mut bounds = plot_ui.plot_bounds();

                // Lock X-axis to the latest 50 candles when auto-following
                if self.follow_latest {
                    if let Some(last) = self.candles.back() {
                        let max_x = last.index as f64 + 2.0;
                        let min_x = (max_x - 50.0).max(0.0);
                        bounds.set_x(&PlotBounds::from_min_max([min_x, 0.0], [max_x, 0.0]));
                    }
                }

                // Dynamically fit Y-axis strictly to candles visible in the current X window
                let (min_x, max_x) = (bounds.min()[0], bounds.max()[0]);
                let visible: Vec<&Candle> = self
                    .candles
                    .iter()
                    .filter(|c| (c.index as f64) >= min_x && (c.index as f64) <= max_x)
                    .collect();

                if !visible.is_empty() {
                    let min_y = visible.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
                    let max_y = visible.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
                    let pad = (max_y - min_y).max(0.5) * 0.08;
                    bounds.set_y(&PlotBounds::from_min_max([0.0, min_y - pad], [0.0, max_y + pad]));
                    plot_ui.set_plot_bounds(bounds);
                }

                // Render bullish and bearish candles as two separate BoxPlot series
                plot_ui.box_plot(BoxPlot::new("Bullish", bull_boxes).allow_hover(false));
                plot_ui.box_plot(BoxPlot::new("Bearish", bear_boxes).allow_hover(false));
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_plot Candlesticks",
        options,
        Box::new(|_cc| Ok(Box::new(MarketApp::default()))),
    )
}

