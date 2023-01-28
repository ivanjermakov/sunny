#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color::mono(0);
    pub const WHITE: Color = Color::mono(255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub const fn mono(k: u8) -> Color {
        Color { r: k, g: k, b: k }
    }

    pub fn brightness(&self, b: f32) -> Color {
        Color {
            r: (self.r as f32 * b).clamp(0., 255.) as u8,
            g: (self.g as f32 * b).clamp(0., 255.) as u8,
            b: (self.b as f32 * b).clamp(0., 255.) as u8,
        }
    }
}
