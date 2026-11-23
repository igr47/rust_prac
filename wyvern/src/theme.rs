use ratatui::style::{Color, Style};

/// The seven elemental wyvern themes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeKind {
    Fire,
    Frost,
    Earth,
    Water,
    Wind,
    Darkness,
    Light,
}

impl ThemeKind {
    pub const ALL: [ThemeKind; 7] = [
        ThemeKind::Fire,
        ThemeKind::Frost,
        ThemeKind::Earth,
        ThemeKind::Water,
        ThemeKind::Wind,
        ThemeKind::Darkness,
        ThemeKind::Light,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ThemeKind::Fire => "fire",
            ThemeKind::Frost => "frost",
            ThemeKind::Earth => "earth",
            ThemeKind::Water => "water",
            ThemeKind::Wind => "wind",
            ThemeKind::Darkness => "darkness",
            ThemeKind::Light => "light",
        }
    }

    /// Flavour name shown in the header.
    pub fn epithet(self) -> &'static str {
        match self {
            ThemeKind::Fire => "Ember Wyvern",
            ThemeKind::Frost => "Rime Wyvern",
            ThemeKind::Earth => "Stone Wyvern",
            ThemeKind::Water => "Tide Wyvern",
            ThemeKind::Wind => "Gale Wyvern",
            ThemeKind::Darkness => "Umbral Wyvern",
            ThemeKind::Light => "Dawn Wyvern",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "fire" | "flame" | "ember" => Some(ThemeKind::Fire),
            "frost" | "ice" => Some(ThemeKind::Frost),
            "earth" | "stone" => Some(ThemeKind::Earth),
            "water" | "tide" => Some(ThemeKind::Water),
            "wind" | "air" | "gale" => Some(ThemeKind::Wind),
            "darkness" | "dark" | "shadow" => Some(ThemeKind::Darkness),
            "light" | "holy" | "dawn" => Some(ThemeKind::Light),
            _ => None,
        }
    }

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|k| *k == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
}

type Rgb = (u8, u8, u8);

fn rgb((r, g, b): Rgb) -> Color {
    Color::Rgb(r, g, b)
}

/// Resolved colours for one theme.
#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: Color, // Color::Reset when transparent
    pub fg: Color,
    pub dim: Color,
    pub border: Color,
    pub focus: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub error: Color,
    pub warn: Color,
    /// Cool -> hot / dark -> bright ramp, used for ASCII art and the title.
    pub gradient: [Color; 6],
}

impl Theme {
    pub fn new(kind: ThemeKind, transparent: bool) -> Self {
        // (bg, fg, dim, border, focus, primary, secondary, accent, gradient)
        let (bg, fg, dim, border, focus, primary, secondary, accent, grad): (
            Rgb, Rgb, Rgb, Rgb, Rgb, Rgb, Rgb, Rgb, [Rgb; 6],
        ) = match kind {
            ThemeKind::Fire => (
                (18, 8, 6), (255, 232, 210), (150, 105, 85), (140, 55, 20), (255, 140, 40),
                (255, 120, 30), (255, 75, 45), (255, 210, 70),
                [(110, 10, 10), (200, 30, 10), (255, 80, 20), (255, 140, 30), (255, 200, 60), (255, 240, 150)],
            ),
            ThemeKind::Frost => (
                (7, 14, 26), (220, 240, 255), (110, 140, 170), (50, 100, 150), (150, 220, 255),
                (120, 200, 255), (160, 170, 255), (255, 255, 255),
                [(40, 90, 170), (60, 140, 210), (100, 190, 240), (150, 220, 255), (200, 240, 255), (245, 252, 255)],
            ),
            ThemeKind::Earth => (
                (20, 15, 9), (232, 218, 188), (135, 120, 90), (100, 80, 45), (170, 200, 80),
                (150, 190, 70), (190, 130, 70), (230, 190, 90),
                [(70, 50, 25), (110, 80, 40), (150, 110, 55), (120, 150, 60), (160, 190, 80), (210, 220, 120)],
            ),
            ThemeKind::Water => (
                (5, 12, 28), (215, 235, 250), (100, 130, 160), (25, 80, 140), (60, 180, 255),
                (40, 150, 255), (0, 200, 200), (130, 240, 235),
                [(10, 40, 120), (20, 90, 180), (30, 140, 230), (0, 190, 220), (70, 225, 225), (170, 245, 240)],
            ),
            ThemeKind::Wind => (
                (12, 20, 22), (225, 240, 238), (120, 145, 145), (70, 110, 110), (170, 240, 210),
                (150, 230, 200), (180, 210, 240), (245, 252, 255),
                [(70, 120, 120), (100, 170, 160), (140, 215, 190), (180, 235, 225), (215, 245, 245), (250, 255, 255)],
            ),
            ThemeKind::Darkness => (
                (10, 6, 16), (220, 210, 235), (120, 100, 145), (80, 45, 120), (190, 90, 255),
                (160, 80, 225), (110, 60, 170), (235, 60, 110),
                [(40, 15, 70), (80, 30, 130), (130, 50, 190), (170, 70, 220), (215, 60, 150), (240, 60, 100)],
            ),
            ThemeKind::Light => (
                (252, 248, 234), (50, 40, 25), (140, 125, 95), (200, 165, 70), (190, 130, 0),
                (170, 110, 0), (200, 90, 20), (180, 60, 10),
                [(120, 70, 0), (150, 95, 0), (180, 125, 0), (205, 150, 10), (225, 120, 20), (200, 70, 10)],
            ),
        };

        let light = kind == ThemeKind::Light;
        let (success, error, warn) = if light {
            ((20, 130, 60), (190, 30, 30), (170, 110, 0))
        } else {
            ((80, 200, 120), (240, 80, 80), (240, 180, 50))
        };

        Theme {
            bg: if transparent { Color::Reset } else { rgb(bg) },
            fg: rgb(fg),
            dim: rgb(dim),
            border: rgb(border),
            focus: rgb(focus),
            primary: rgb(primary),
            secondary: rgb(secondary),
            accent: rgb(accent),
            success: rgb(success),
            error: rgb(error),
            warn: rgb(warn),
            gradient: grad.map(rgb),
        }
    }

    /// Base style: foreground + (possibly transparent) background.
    pub fn base(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }
}

