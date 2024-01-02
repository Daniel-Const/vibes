use crate::term::Terminal;

pub struct Visualiser {
    draw_char: char,
    fill: bool,
}

impl Visualiser {
    pub fn new(draw_char: char, fill: bool) -> Visualiser {
        Visualiser { draw_char, fill }
    }

    pub fn simple(&self, data: &[f32], term: &mut Terminal, scale_factor: f32) {
        for (i, d) in data.iter().enumerate() {
            if i > term.cols.into() {
                break;
            }

            let y: u16 = ((term.rows / 2) as f32 + (*d * term.rows as f32) * scale_factor) as u16;

            if self.fill {
                let range = match y {
                    y if y < term.rows / 2 => y..(term.rows / 2),
                    y if y >= term.rows / 2 => (term.rows / 2)..y,
                    _ => 0..0,
                };

                for dy in range {
                    let _ = term.draw_at(i.try_into().unwrap(), dy, self.draw_char);
                }
            } else {
                let _ = term.draw_at(i.try_into().unwrap(), y, self.draw_char);
            }
        }
    }
}
