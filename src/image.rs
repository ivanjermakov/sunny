use std::fs::write;

use crate::color::Color;
use crate::vec3::Vec3;

// TODO: add metadata (reflection_depth, pass_count)
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Image {
    pub resolution: Vec3,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        let mut content: Vec<u8> = vec![];
        let header = format!(
            "P6\n{w} {h}\n255\n",
            w = self.resolution.x,
            h = self.resolution.y
        )
        .as_bytes()
        .to_vec();
        content.extend(header);
        for px in &self.pixels {
            content.extend([
                (px.r * 255.) as u8,
                (px.g * 255.) as u8,
                (px.b * 255.) as u8,
            ])
        }
        write(path, content)
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;
    use crate::image::Image;
    use crate::vec3::Vec3;

    #[test]
    fn save_image() {
        let r = Vec3::new(100., 100., 0.);
        let w = r.x as i32;
        let mut ps = vec![];
        for x in 0..w as i32 {
            for y in 0..r.y as i32 {
                ps.push(Color {
                    r: if (x + y) % 2 == 0 { 1. } else { 0. },
                    g: 0.,
                    b: 0.,
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
