use crate::term::TermCoords;

pub trait Visualiser {
    fn map_coords(&self, data: &[f32], max_height: u16, max_width: u16) -> Vec<TermCoords>;
}
