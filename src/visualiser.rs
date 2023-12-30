use drawille::{Canvas, PixelColor};

pub struct Visualiser {
    canvas: Canvas,
    width: u32,
    height: u32,
    wave_centre: i32,
    draw_min: f32,
    draw_max: f32,
}

impl Visualiser {
    pub fn new(width: u32, height: u32, wave_centre: i32, wave_height: u32) -> Visualiser {
        Visualiser {
            canvas: Canvas::new(width, height),
            width,
            height,
            wave_centre,
            draw_min: -(wave_height as f32),
            draw_max: (wave_height as f32),
        }
    }
    
    /**
     * Map sample data onto the y-axis
     */
    fn map_y(&self, data: f32, sample_min: f32, sample_max: f32) -> f32 {
        let sample = data.clamp(sample_min, sample_max);
        return self.draw_min + ((self.draw_max - self.draw_min) / (sample_max - sample_min)) * (sample - sample_min);
    }

    pub fn draw(&mut self, samples: &Vec<f32>) {
        
        for (i, data) in samples.iter().enumerate() {
            if i as u32 > self.width {
                break;
            }

            let y = self.map_y(*data, -0.5, 0.5).round() as i32;
            let x = i as u32;
            self.canvas.line_colored(
                x,
                (self.wave_centre + y) as u32,
                x,
                self.wave_centre as u32,
                PixelColor::Green,
            );
        }
    }

    pub fn draw_to_terminal(&self) {
        print!("\x1B[2J");
        println!("{}", self.canvas.frame());
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }
}
