use drawille::{Canvas, PixelColor};

pub struct Visualiser {
    canvas: Canvas,
    width: u32,
    height: u32
}

impl Visualiser {
    pub fn new(width: u32, height: u32) -> Visualiser {
        Visualiser {
            canvas: Canvas::new(width, height),
            width,
            height
        }
    }
    
    /**
     * Map sample data onto the y-axis
     * Map into range [0, 1] then multiply by canvas height
     */
    fn map_y(&self, data: f32, sample_min: f32, sample_max: f32) -> f32 {
        // TODO: Find a better mapping algorithm
        let y = 1.0 / (sample_max - sample_min) * (data - sample_min);
        return y * self.height as f32;
    }

    pub fn draw(&mut self, samples: &Vec<f32>) {
        let sample_min = samples
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        
        let sample_max = samples.
            iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        
        for (i, data) in samples.iter().enumerate() {
            if i as u32 > self.width {
                break;
            }
            let y = self.map_y(*data, *sample_min, *sample_max) as u32;
            let x = i as u32;
            self.canvas.line_colored(
                x,
                self.height - y,
                x,
                self.height,
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
