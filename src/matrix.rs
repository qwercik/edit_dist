use std::ops::{Index, IndexMut};
use std::fmt;

pub type Selector = (usize, usize);

pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize
}

impl<T> Matrix<T> 
where T: Default + Clone
{
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            data: vec![T::default(); width * height],
            width,
            height
        }
    }

    fn get_index(&self, selector: Selector) -> Option<usize> {
        let (y, x) = selector;
        let index = y * self.width + x;

        if index < self.width * self.height {
            Some(index)
        } else {
            None
        }
    }

    pub fn get(&self, selector: Selector) -> Option<&T> {
        match self.get_index(selector) {
            Some(index) => self.data.get(index),
            None => None
        }
    }

    pub fn get_mut(&mut self, selector: Selector) -> Option<&mut T> {
        match self.get_index(selector) {
            Some(index) => self.data.get_mut(index),
            None => None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T>Index<Selector> for Matrix<T>
where T: Default + Clone 
{
    type Output = T;

    fn index(&self, selector: Selector) -> &Self::Output {
        self.get(selector)
            .expect(&format!("Invalid index ({}, {})", selector.0, selector.1))
    }
}

impl<T> IndexMut<Selector> for Matrix<T>
where T: Default + Clone
{
    fn index_mut(&mut self, selector: Selector) -> &mut Self::Output {
        self.get_mut(selector)
            .expect(&format!("Invalid index ({}, {})", selector.0, selector.1))
    }
}

impl<T> fmt::Display for Matrix<T>
where T: Default + Clone + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() - 1 {
                write!(f, "{} ", self[(y, x)])?;
            }
            write!(f, "{}\n", self[(y, self.width() - 1)])?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_index() {
        let matrix: Matrix<u32> = Matrix::new(3, 3);

        assert_eq!(matrix.get_index((0, 0)), Some(0));
        assert_eq!(matrix.get_index((0, 1)), Some(1));
        assert_eq!(matrix.get_index((0, 2)), Some(2));
        assert_eq!(matrix.get_index((1, 0)), Some(3));
        assert_eq!(matrix.get_index((1, 1)), Some(4));
        assert_eq!(matrix.get_index((1, 2)), Some(5));
        assert_eq!(matrix.get_index((2, 0)), Some(6));
        assert_eq!(matrix.get_index((2, 1)), Some(7));
        assert_eq!(matrix.get_index((2, 2)), Some(8));
    }

    #[test]
    fn test_get() {
        let matrix = Matrix::new(3, 3);
        // Default value should be 0
        assert_eq!(matrix.get((1, 1)), Some(0).as_ref());
    }

    #[test]
    fn test_get_mut() {
        let mut matrix = Matrix::new(3, 3);
        if let Some(reference) = matrix.get_mut((1, 0)) {
            *reference = 112;
        }

        assert_eq!(matrix.get((1, 0)), Some(112).as_ref());
    }

    #[test]
    fn test_index() {
        let matrix = Matrix::<u32>::new(3, 3);
        // Default value should be 0
        assert_eq!(matrix[(1, 1)], 0u32);
    }

    #[test]
    fn test_index_mut() {
        let mut matrix = Matrix::<u32>::new(3, 3);
        matrix[(1, 1)] = 112;

        assert_eq!(matrix[(1, 1)], 112u32);
    }
}
