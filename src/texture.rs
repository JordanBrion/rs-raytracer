use super::color::*;

struct SolidColor {
    color_value: Color,
}

trait Texture {
    fn color(&self) -> Color;
}

impl Texture for SolidColor {
    fn color(&self) -> Color {
        self.color_value
    }
}
