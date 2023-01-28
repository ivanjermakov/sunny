use std::fs::write;

use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Image {
    pub resolution: Vec3,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let content = format!(
            "P3\n{w} {h}\n15\n{d}",
            w = self.resolution.x,
            h = self.resolution.y,
            d = self
                .pixels
                .iter()
                .map(|Color { r, g, b }| format!("{r:<3} {g:<3} {b:<3}"))
                .collect::<Vec<_>>()
                .chunks(self.resolution.x as usize)
                .map(|c| c.join("   "))
                .collect::<Vec<_>>()
                .join("\n")
        );
        write(path, content)
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::image::Image;
    use crate::vec3::Vec3;

    #[test]
    fn save_solid_image() {
        let r = Vec3::new(600., 400., 0.);
        let w = r.x as i32;
        let mut ps = vec![];
        for _ in 0..w as i32 {
            for _ in 0..r.y as i32 {
                ps.push(Color { r: 255, g: 0, b: 0 });
            }
        }
        let i = Image {
            resolution: r,
            pixels: ps,
        };
        i.save_ppm("data/test.ppm").ok();
    }
}
