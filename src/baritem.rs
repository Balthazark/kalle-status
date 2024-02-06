pub mod audio;
pub mod backlight;
pub mod charge;
pub mod network;
pub mod time;

pub static BAR_HEIGHT: u32 = 62;

pub trait BarItem {
    fn get_bar_text(&mut self) -> String;
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn icon_from_percent(icons: &[char], percent: u32) -> &char {
    let act_index = (((percent as f64) / 100.0) * (icons.len() - 1) as f64) as usize;
    &icons[act_index]
}

impl Color {
    pub fn catppuccin_white() -> Self {
        Color {
            r: 205,
            g: 214,
            b: 244,
        }
    }
    pub fn catppucin_red() -> Self {
        Color {
            r: 243,
            g: 139,
            b: 168,
        }
    }
    pub fn nord_orange() -> Self {
        Color {
            r: 208,
            g: 135,
            b: 112,
        }
    }
    pub fn nord_yellow() -> Self {
        Color {
            r: 235,
            g: 203,
            b: 129,
        }
    }
    pub fn nord_green() -> Self {
        Color {
            r: 163,
            g: 190,
            b: 140,
        }
    }
    pub fn catppucin_muave() -> Self {
        Color {
            r: 203,
            g: 166,
            b: 247,
        }
    }
    pub fn catppuccin_blue() -> Self {
        Color {
            r: 137,
            g: 180,
            b: 250,
        }
    }

    pub fn as_str(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }

    pub fn apply_fg(&self) -> String {
        format!("^c{}^", self.as_str())
    }
    pub fn apply_bg(&self) -> String {
        format!("^b{}^", self.as_str())
    }
}

pub fn regtangle(x: u32, y: u32, w: u32, h: u32, col: &str) -> String {
    format!("^c{}^^r{},{},{},{}^^f{}^", col, x, y, w, h, w)
}
