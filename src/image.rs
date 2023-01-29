use std::fs::write;

use crate::color::RgbColor;
use crate::vec3::Vec3;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Image {
    pub resolution: Vec3,
    pub pixels: Vec<RgbColor>,
}

impl Image {
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let content = format!(
            "P3\n{w} {h}\n255\n{d}",
            w = self.resolution.x,
            h = self.resolution.y,
            d = self
                .pixels
                .iter()
                .map(|RgbColor { r, g, b }| format!("{r} {g} {b} "))
                .collect::<Vec<_>>()
                .join("\n")
        );
        write(path, content)
    }
}

#[cfg(test)]
mod test {
    use crate::color::RgbColor;
    use crate::image::Image;
    use crate::vec3::Vec3;

    #[test]
    fn save_image() {
        let r = Vec3::new(100., 100., 0.);
        let w = r.x as i32;
        let mut ps = vec![];
        for x in 0..w as i32 {
            for y in 0..r.y as i32 {
                ps.push(RgbColor {
                    r: if (x + y) % 2 == 0 { 255 } else { 0 },
                    g: 0,
                    b: 0,
                });
            }
        }
        let i = Image {
            resolution: r,
            pixels: ps,
        };
        i.save_ppm("data/test.ppm").ok();
    }
}
