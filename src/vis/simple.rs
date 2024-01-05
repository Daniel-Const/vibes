use super::visualiser::Visualiser;
use crate::term::TermCoords;

pub struct Simple {
    scale: f32,
    fill: bool,
}

impl Simple {
    pub fn new(fill: bool) -> Simple {
        Simple { scale: 6.0, fill }
    }
}

impl Visualiser for Simple {
    fn map_coords(&self, data: &[f32], max_height: u16, max_width: u16) -> Vec<TermCoords> {
        let mut coords: Vec<TermCoords> = vec![];
        for (x, d) in data.iter().enumerate() {
            if x as u16 > max_width {
                break;
            }

            let mut y: u16 =
                ((max_height / 2) as f32 + (*d * max_height as f32) * self.scale) as u16;

            y = y.clamp(0, max_height);

            if self.fill {
                // Iterate from y to middle or middle to y depending on location
                let range = match y {
                    y if y < max_height / 2 => y..(max_height / 2),
                    y if y >= max_height / 2 => (max_height / 2)..y,
                    _ => 0..0,
                };

                for dy in range {
                    coords.push(TermCoords(x.try_into().unwrap(), dy));
                }
            } else {
                coords.push(TermCoords(x.try_into().unwrap(), y))
            }
        }

        return coords;
    }
}
