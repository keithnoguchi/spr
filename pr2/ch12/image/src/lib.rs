//! image: Index trait excercise
use std::ops::{Index, IndexMut};

pub struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

impl<P> Image<P>
where
    P: Clone + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            pixels: vec![P::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }
}

#[cfg(test)]
mod tests {
    use super::Image;

    #[test]
    fn index() {
        let image = Image::<u8>::new(10, 11);
        for row in 0..image.height() {
            assert_eq!(image[row].len(), 10);
            for column in 0..image.width() {
                assert_eq!(image[row][column], 0u8);
            }
        }
    }

    #[test]
    fn index_mut() {
        let mut image = Image::<f64>::new(200, 99);
        for row in 0..image.height() {
            for column in 0..image.width() {
                assert_eq!(image[row][column], 0.0);
                image[row][column] = (row * column) as f64;
            }
        }
        assert_eq!(image[98][199], (98 * 199) as f64);
    }

    #[test]
    fn new() {
        let image = Image::<u8>::new(10, 11);
        assert_eq!(image.width, 10);
        assert_eq!(image.pixels.len(), 110);
    }
}
