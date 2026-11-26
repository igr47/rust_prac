use eframe::egui;
use egui::{Color32, Stroke};
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Line, Plot, PlotBounds, PlotPoints, HoverPosition};
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
    // Moving average settings
    ma_short_period: usize,
    ma_long_period: usize,
    show_ma_short: bool,
    show_ma_long: bool,
    // Crosshair / hover state
    hover_pos: Option<[f64; 2]>,
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
            ma_short_period: 5,
            ma_long_period: 20,
            show_ma_short: true,
            show_ma_long: true,
            hover_pos: None,
        }
    }
}

impl MarketApp {
    fn simulate_market_feed(&mut self) {
        if self.last_tick.elapsed() < Duration::from_millis(150) {
            return;
        }
        self.last_tick = Instant::now();
        self.tick_count += 1;

        let delta = ((self.tick_count as f64 * 0.7).cos()) * 0.45;

        if self.tick_count % 10 == 0 {
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
            if self.candles.len() > 500 {
                self.candles.pop_front();
            }
        } else if let Some(last) = self.candles.back_mut() {
            last.close += delta;
            last.high = last.high.max(last.close);
            last.low = last.low.min(last.close);
        }
    }

    /// Compute a simple moving average over the candle closes.
    /// Returns points (index, average) only where a full window is available.
    fn compute_ma(&self, period: usize) -> Vec<[f64; 2]> {
        if period == 0 || self.candles.len() < period {
            return Vec::new();
        }
        let mut points = Vec::with_capacity(self.candles.len().saturating_sub(period - 1));
        let closes: Vec<f64> = self.candles.iter().map(|c| c.close).collect();
        let mut sum: f64 = closes[..period].iter().sum();
        points.push([self.candles[period - 1].index as f64, sum / period as f64]);

        for i in period..closes.len() {
            sum += closes[i] - closes[i - period];
            points.push([self.candles[i].index as f64, sum / period as f64]);
        }
        points
    }
}

impl eframe::App for MarketApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.simulate_market_feed();
        ui.request_repaint_after(Duration::from_millis(50));

        egui::Panel::top("controls").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Live Candlestick Chart");
                ui.separator();
                ui.checkbox(&mut self.follow_latest, "Auto-scroll");
                ui.separator();
                ui.checkbox(&mut self.show_ma_short, "MA Short");
                ui.add(egui::DragValue::new(&mut self.ma_short_period).range(2..=50).prefix("(").suffix(")"));
                ui.separator();
                ui.checkbox(&mut self.show_ma_long, "MA Long");
                ui.add(egui::DragValue::new(&mut self.ma_long_period).range(2..=200).prefix("(").suffix(")"));
            });
            ui.horizontal(|ui| {
                if let Some(last) = self.candles.back() {
                    ui.label(format!("Last: ${:.2}", last.close));
                    ui.separator();
                    ui.label(format!("O: {:.2} H: {:.2} L: {:.2} C: {:.2}", 
                        last.open, last.high, last.low, last.close));
                }
                if let Some(pos) = self.hover_pos {
                    ui.separator();
                    ui.label(format!("Hover: x={:.0}  y=${:.2}", pos[0], pos[1]));
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

                let elem = BoxElem::new(
                    c.index as f64,
                    BoxSpread::new(c.low, body_low, c.close, body_high, c.high),
                )
                .box_width(0.65)
                .whisker_width(0.0)
                .fill(color)
                .stroke(Stroke::new(1.5, color));

                if is_bull {
                    bull_boxes.push(elem);
                } else {
                    bear_boxes.push(elem);
                }
            }

            // Pre-compute MA lines outside the closure to avoid borrow issues
            let ma_short_points = if self.show_ma_short {
                self.compute_ma(self.ma_short_period)
            } else {
                Vec::new()
            };
            let ma_long_points = if self.show_ma_long {
                self.compute_ma(self.ma_long_period)
            } else {
                Vec::new()
            };

            // Capture hover position for the label formatter via a cell
            let hover_cell = std::cell::Cell::new(self.hover_pos);
            // let hover_pos = &mut self.hover_pos;

            let plot = Plot::new("candlestick_plot")
                .allow_drag(true)
                .allow_zoom(true)
                .show_axes([true, true])
                .x_axis_label("Candle Index")
                .y_axis_label("Price ($)")
                .y_axis_formatter(|mark, _range| format!("${:.2}", mark.value))
                .label_formatter(|hover| {
                    let (name, value) = match hover {
                    HoverPosition::NearDataPoint {
                        plot_name,
                        position,
                        ..
                    } => (*plot_name, *position),
                    HoverPosition::Elsewhere { position } => ("", *position),
                    };

                    // Cell::set works through a shared reference — no &mut needed
                    hover_cell.set(Some([value.x, value.y]));

                    let _ = name; // silence unused warning
                    Some(format!("x = {:.0}\ny = ${:.2}", value.x, value.y))
                    });

            let response = plot.show(ui, |plot_ui| {
                if plot_ui.response().dragged() {
                    self.follow_latest = false;
                }

                let mut bounds = plot_ui.plot_bounds();

                if self.follow_latest {
                    if let Some(last) = self.candles.back() {
                        let max_x = last.index as f64 + 2.0;
                        let min_x = (max_x - 50.0).max(0.0);
                        bounds.set_x(&PlotBounds::from_min_max([min_x, 0.0], [max_x, 0.0]));
                    }
                }

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

                // Draw moving average lines first so candles render on top
                if !ma_short_points.is_empty() {
                    let line = Line::new( format!("MA({})", self.ma_short_period),PlotPoints::from(ma_short_points))
                        //.name(format!("MA({})", self.ma_short_period))
                        .color(Color32::from_rgb(255, 193, 7))
                        .width(2.0);
                    plot_ui.line(line);
                }
                if !ma_long_points.is_empty() {
                    let line = Line::new(format!("MA({})", self.ma_long_period), PlotPoints::from(ma_long_points))
                        //.name(format!("MA({})", self.ma_long_period))
                        .color(Color32::from_rgb(33, 150, 243))
                        .width(2.0);
                    plot_ui.line(line);
                }

                plot_ui.box_plot(BoxPlot::new("Bullish", bull_boxes).allow_hover(false));
                plot_ui.box_plot(BoxPlot::new("Bearish", bear_boxes).allow_hover(false));
            });

            // Update hover state from the cell after the plot is drawn
            self.hover_pos = hover_cell.get();

            // Optional: if the pointer leaves the plot area, clear hover
            if response.response.hover_pos().is_none() {
                self.hover_pos = None;
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui_plot Candlesticks with MA",
        options,
        Box::new(|_cc| Ok(Box::new(MarketApp::default()))),
    )
}
