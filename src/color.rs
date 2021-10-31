use std::ops;

use crate::util::clamp;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn add(a: Color, b: Color) -> Color {
        Color {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        }
    }

    pub fn sub(a: Color, b: Color) -> Color {
        Color {
            r: a.r - b.r,
            g: a.g - b.g,
            b: a.b - b.b,
        }
    }

    pub fn add_weighted(a: Color, weight_a: f32, b: Color, weight_b: f32) -> Color {
        let sum = weight_a + weight_b;

        let r = ((a.r * weight_a) + (b.r * weight_b)) / sum;
        let g = ((a.g * weight_a) + (b.g * weight_b)) / sum;
        let b = ((a.b * weight_a) + (b.b * weight_b)) / sum;

        Color { r, g, b }
    }

    pub fn blend(a: Color, weight: f32, b: Color) -> Color {
        let weight_a = clamp(weight, 0.0, 1.0);
        let weight_b = 1.0 - weight_a;

        Color::add_weighted(a, weight_a, b, weight_b)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, v: Color) -> Color {
        Color::add(self, v)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, v: Color) -> Color {
        Color::sub(self, v)
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, f: f32) -> Color {
        Color::new(self.r * f, self.g * f, self.b * f)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, v: Color) -> Color {
        Color::new(self * v.r, self * v.g, self * v.b)
    }
}
