use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::{
    app::{App, Areas, Focus, LineKind},
    art,
    theme::Theme,
};

const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub fn draw(f: &mut Frame, app: &mut App) {
    let t = app.theme();
    let area = f.area();

    // Paint the whole screen first (Color::Reset bg => terminal transparency).
    f.render_widget(Block::default().style(t.base()), area);

    let wide = area.width >= 100 && area.height >= 30;
    let header_h = if wide { 8 } else { 3 };

    let [header, body, input] = Layout::vertical([
        Constraint::Length(header_h),
        Constraint::Min(3),
        Constraint::Length(7),
    ])
    .areas(area);

    let tables_w = (area.width / 4).clamp(18, 32);
    let [tables, results] =
        Layout::horizontal([Constraint::Length(tables_w), Constraint::Min(10)]).areas(body);

    app.areas = Areas { tables, results, input };

    draw_header(f, header, app, &t, wide);
    draw_tables(f, tables, app, &t);
    draw_results(f, results, app, &t);
    draw_input(f, input, app, &t);
}

// ───────────────────────── helpers ─────────────────────────

fn panel<'a>(title: &str, focused: bool, t: &Theme) -> Block<'a> {
    let (style, kind) = if focused {
        (Style::new().fg(t.focus).add_modifier(Modifier::BOLD), BorderType::Double)
    } else {
        (Style::new().fg(t.border), BorderType::Rounded)
    };
    Block::bordered()
        .border_type(kind)
        .border_style(style)
        .title(Span::styled(
            format!(" {title} "),
            Style::new().fg(t.primary).add_modifier(Modifier::BOLD),
        ))
        .style(t.base())
}

fn kind_style(t: &Theme, kind: LineKind) -> Style {
    match kind {
        LineKind::Info | LineKind::Row => Style::new().fg(t.fg),
        LineKind::Success => Style::new().fg(t.success),
        LineKind::Error => Style::new().fg(t.error).add_modifier(Modifier::BOLD),
        LineKind::Warn => Style::new().fg(t.warn),
        LineKind::Header => Style::new().fg(t.primary).add_modifier(Modifier::BOLD),
        LineKind::Dim => Style::new().fg(t.dim),
        LineKind::Echo => Style::new().fg(t.secondary).add_modifier(Modifier::BOLD),
    }
}

/// Horizontal gradient across one line of text.
fn gradient_line(text: &str, t: &Theme) -> Line<'static> {
    let n = t.gradient.len();
    let w = text.chars().count().max(1);
    Line::from(
        text.chars()
            .enumerate()
            .map(|(i, ch)| {
                let idx = (i * n / w).min(n - 1);
                Span::styled(ch.to_string(), Style::new().fg(t.gradient[idx]).add_modifier(Modifier::BOLD))
            })
            .collect::<Vec<_>>(),
    )
}

/// Animated flame colour: hot on the left (the breath), flickering over time.
fn flame_color(t: &Theme, row: usize, col: usize, width: usize, tick: u64) -> ratatui::style::Color {
    let n = t.gradient.len();
    let heat = (n - 1) - col * (n - 1) / width.max(1);
    let h = (col as u64).wrapping_mul(73_856_093)
        ^ (row as u64).wrapping_mul(19_349_663)
        ^ (tick / 2).wrapping_mul(83_492_791);
    let flicker = ((h >> 4) % 3) as isize - 1; // -1, 0, +1
    let wave = ((tick as usize / 3 + row + col / 6) % 3) as isize - 1;
    t.gradient[(heat as isize + flicker + wave).clamp(0, n as isize - 1) as usize]
}

fn status_span(app: &App, t: &Theme) -> Span<'static> {
    if app.is_running() {
        let ch = SPINNER[(app.tick as usize) % SPINNER.len()];
        Span::styled(format!("{ch} running"), Style::new().fg(t.warn))
    } else {
        Span::styled("● idle", Style::new().fg(t.success))
    }
}

// ───────────────────────── header ─────────────────────────

fn info_lines(app: &App, t: &Theme) -> Vec<Line<'static>> {
    let label = |s: &str| Span::styled(format!(" {s:<11}"), Style::new().fg(t.dim));
    let val = |s: String| Span::styled(s, Style::new().fg(t.fg));
    let tables: usize = app.catalog.iter().map(|d| d.tables.len()).sum();
    vec![
        Line::from(vec![
            label("Theme"),
            Span::styled(
                format!("{} · {}", app.theme_kind.name(), app.theme_kind.epithet()),
                Style::new().fg(t.accent).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![label("Version"), val(format!("v{}", env!("CARGO_PKG_VERSION")))]),
        Line::from(vec![label("Database"), val(app.current_db.clone().unwrap_or_else(|| "—".into()))]),
        Line::from(vec![
            label("Catalog"),
            val(format!("{} db · {} tables/models", app.catalog.len(), tables)),
        ]),
        Line::from(vec![
            label("Background"),
            val(if app.transparent { "transparent".into() } else { "solid".into() }),
        ]),
        Line::from(vec![label("Status"), status_span(app, t)]),
    ]
}

fn draw_header(f: &mut Frame, area: Rect, app: &App, t: &Theme, wide: bool) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(t.border))
        .style(t.base());
    let inner = block.inner(area);
    f.render_widget(block, area);

    if wide {
        let [title_a, art_a, info_a] = Layout::horizontal([
            Constraint::Length(58),
            Constraint::Length(22),
            Constraint::Min(10),
        ])
        .areas(inner);

        let title: Vec<Line> = art::TITLE.lines().map(|l| gradient_line(l, t)).collect();
        f.render_widget(Paragraph::new(title), title_a);

        // vertical "span line" separators
        let sep = Block::default().borders(Borders::LEFT).border_style(Style::new().fg(t.border));
        let art_inner = sep.inner(art_a);
        f.render_widget(sep.clone(), art_a);
        let art_lines: Vec<Line> = art::HEADER_ART
            .lines()
            .enumerate()
            .map(|(i, l)| Line::styled(l.to_string(), Style::new().fg(t.gradient[(i + 1).min(5)])))
            .collect();
        f.render_widget(Paragraph::new(art_lines), art_inner);

        let info_inner = sep.inner(info_a);
        f.render_widget(sep, info_a);
        f.render_widget(Paragraph::new(info_lines(app, t)), info_inner);
    } else {
        let bar = || Span::styled(" │ ", Style::new().fg(t.border));
        let line = Line::from(vec![
            Span::styled(" WYVERN ", Style::new().fg(t.primary).add_modifier(Modifier::BOLD)),
            bar(),
            Span::styled(app.theme_kind.name().to_string(), Style::new().fg(t.accent)),
            bar(),
            Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")), Style::new().fg(t.fg)),
            bar(),
            Span::styled(
                format!("db: {}", app.current_db.clone().unwrap_or_else(|| "—".into())),
                Style::new().fg(t.fg),
            ),
            bar(),
            status_span(app, t),
        ]);
        f.render_widget(Paragraph::new(line), inner);
    }
}

// ───────────────────────── tables panel ─────────────────────────

fn draw_tables(f: &mut Frame, area: Rect, app: &mut App, t: &Theme) {
    let mut lines: Vec<Line> = Vec::new();
    for db in &app.catalog {
        let current = app.current_db.as_deref() == Some(db.name.as_str());
        let marker = if current { "●" } else { "○" };
        lines.push(Line::from(Span::styled(
            format!("{marker} {}", db.name),
            Style::new()
                .fg(if current { t.accent } else { t.primary })
                .add_modifier(Modifier::BOLD),
        )));
        let n = db.tables.len();
        for (i, table) in db.tables.iter().enumerate() {
            let branch = if i + 1 == n { "└─" } else { "├─" };
            lines.push(Line::from(vec![
                Span::styled(format!("  {branch} "), Style::new().fg(t.dim)),
                Span::styled(table.clone(), Style::new().fg(t.fg)),
            ]));
        }
        if n == 0 {
            lines.push(Line::styled("  (empty)", Style::new().fg(t.dim)));
        }
    }

    let h = area.height.saturating_sub(2) as usize;
    let max = lines.len().saturating_sub(h);
    app.tables_scroll = app.tables_scroll.min(max);

    let block = panel("Databases & Tables", app.focus == Focus::Tables, t);
    f.render_widget(
        Paragraph::new(lines).block(block).scroll((app.tables_scroll as u16, 0)),
        area,
    );
}

// ───────────────────────── results panel ─────────────────────────

fn draw_results(f: &mut Frame, area: Rect, app: &mut App, t: &Theme) {
    let focused = app.focus == Focus::Results;
    let h = area.height.saturating_sub(2) as usize;
    let total = app.results.len();

    let max_off = total.saturating_sub(h);
    app.results_scroll = app.results_scroll.min(max_off);
    let end = total - app.results_scroll;
    let start = end.saturating_sub(h);

    let title = if app.is_running() {
        format!("Result Display {} streaming", SPINNER[(app.tick as usize) % SPINNER.len()])
    } else {
        "Result Display".to_string()
    };
    let mut block = panel(&title, focused, t);
    if !app.show_splash && total > 0 {
        let hint = if app.results_scroll > 0 {
            format!(" ↑ scrolled · {end}/{total} · End = follow ")
        } else {
            format!(" {total} lines ")
        };
        block = block.title_bottom(Line::styled(hint, Style::new().fg(t.dim)).right_aligned());
    }
    let inner = block.inner(area);
    f.render_widget(block, area);

    if app.show_splash {
        draw_splash(f, inner, app, t);
        return;
    }

    let lines: Vec<Line> = app.results[start..end]
        .iter()
        .map(|l| Line::styled(l.text.clone(), kind_style(t, l.kind)))
        .collect();
    f.render_widget(Paragraph::new(lines), inner);

    if total > h {
        let mut state = ScrollbarState::new(total).viewport_content_length(h).position(start);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .thumb_style(Style::new().fg(t.primary))
                .track_style(Style::new().fg(t.border)),
            area.inner(Margin { vertical: 1, horizontal: 0 }),
            &mut state,
        );
    }
}

fn draw_splash(f: &mut Frame, inner: Rect, app: &App, t: &Theme) {
    let art_lines: Vec<&str> = art::WYVERN_FIRE.lines().collect();
    let art_w = art_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let w = inner.width as usize;
    let pad_x = w.saturating_sub(art_w) / 2;
    let content_h = art_lines.len() + 4;
    let pad_y = (inner.height as usize).saturating_sub(content_h) / 2;

    let mut lines: Vec<Line> = vec![Line::raw(""); pad_y];

    for (row, l) in art_lines.iter().enumerate() {
        let mut spans = vec![Span::raw(" ".repeat(pad_x))];
        for (col, ch) in l.chars().enumerate() {
            if ch == ' ' {
                spans.push(Span::raw(" "));
            } else {
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::new().fg(flame_color(t, row, col, art_w, app.tick)),
                ));
            }
        }
        lines.push(Line::from(spans));
    }

    let center = |s: &str, style: Style| {
        let pad = w.saturating_sub(s.chars().count()) / 2;
        Line::from(Span::styled(format!("{}{}", " ".repeat(pad), s), style))
    };
    lines.push(Line::raw(""));
    lines.push(center(
        "W  Y  V  E  R  N",
        Style::new().fg(t.primary).add_modifier(Modifier::BOLD),
    ));
    lines.push(center(
        &format!("custom query language shell · v{}", env!("CARGO_PKG_VERSION")),
        Style::new().fg(t.dim),
    ));
    lines.push(center(
        "type `help` to begin · try `select * from dragons`",
        Style::new().fg(t.fg),
    ));

    f.render_widget(Paragraph::new(lines), inner);
}

// ───────────────────────── input panel ─────────────────────────

fn draw_input(f: &mut Frame, area: Rect, app: &mut App, t: &Theme) {
    let focused = app.focus == Focus::Input;
    let hints = " Enter run · ↑↓ history · Tab focus · PgUp/PgDn scroll · ^L clear results · ^K clear input · ^T theme · ^B transparency ";
    let block = panel("Input", focused, t)
        .title_bottom(Line::styled(hints, Style::new().fg(t.dim)));
    let inner = block.inner(area);
    f.render_widget(block, area);
    if inner.height == 0 || inner.width < 4 {
        return;
    }

    // Bottom row = live prompt. Rows above = scrollable log of previous inputs.
    let log_h = inner.height as usize - 1;
    let total = app.history.len();
    let max_off = total.saturating_sub(log_h);
    app.input_scroll = app.input_scroll.min(max_off);
    let end = total - app.input_scroll;
    let start = end.saturating_sub(log_h);
    let shown = &app.history[start..end];

    let log_lines: Vec<Line> = shown
        .iter()
        .map(|s| {
            Line::from(vec![
                Span::styled("❯ ", Style::new().fg(t.dim)),
                Span::styled(s.clone(), Style::new().fg(t.dim)),
            ])
        })
        .collect();
    let log_rect = Rect {
        x: inner.x,
        y: inner.y + (log_h - shown.len()) as u16,
        width: inner.width,
        height: shown.len() as u16,
    };
    f.render_widget(Paragraph::new(log_lines), log_rect);

    if app.input_scroll > 0 {
        let tag = format!(" ↑ {} older ", total - end);
        let w = tag.chars().count() as u16;
        if inner.width > w && log_h > 0 {
            let r = Rect { x: inner.x + inner.width - w, y: inner.y, width: w, height: 1 };
            f.render_widget(Paragraph::new(Span::styled(tag, Style::new().fg(t.warn))), r);
        }
    }

    // live prompt line with horizontal scrolling
    let prompt = format!("{} ❯ ", app.current_db.as_deref().unwrap_or("wyvern"));
    let prompt_w = prompt.chars().count();
    let avail = (inner.width as usize).saturating_sub(prompt_w).max(1);
    let first = if app.cursor >= avail { app.cursor - avail + 1 } else { 0 };
    let last = (first + avail).min(app.input.len());
    let visible: String = app.input[first..last].iter().collect();

    let row = Rect { x: inner.x, y: inner.y + inner.height - 1, width: inner.width, height: 1 };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(prompt, Style::new().fg(t.primary).add_modifier(Modifier::BOLD)),
            Span::styled(visible, Style::new().fg(t.fg)),
        ])),
        row,
    );

    if focused {
        let cx = inner.x + (prompt_w + app.cursor - first) as u16;
        f.set_cursor_position((cx.min(inner.x + inner.width - 1), row.y));
    }
}

