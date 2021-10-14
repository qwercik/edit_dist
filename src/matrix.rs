use std::fmt;
use std::ops::{Index, IndexMut};

/// Type being used for selecting cells from a matrix
pub type Selector = (usize, usize);

/// 2D matrix with given size of T-typed elements
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    /// Create a new instance of Matrix, with given size
    ///
    /// # Arguments
    /// * `width` - Width of a matrix
    /// * `height` - Height of a matrix
    ///
    /// # Examples
    /// ```
    /// let matrix = Matrix::new(3, 7);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    /// Get raw 1D index for current matrix and given selector
    ///
    /// # Arguments
    /// * `selector` - Selector to the specific cell
    ///
    /// # Examples
    /// ```
    /// let idx = self.get_index((2, 3));
    /// ```
    fn get_index(&self, selector: Selector) -> Option<usize> {
        let (y, x) = selector;
        let index = y * self.width + x;

        if index < self.width * self.height {
            Some(index)
        } else {
            None
        }
    }

    /// Get read-only reference to the specific cell
    ///
    /// # Arguments
    /// * `selector` - Selector to the specific cell
    ///
    /// # Examples
    /// ```
    /// let matrix = Matrix::new(7, 3);
    /// // ...
    /// match matrix.get((4, 2)) {
    ///     Ok(reference) => {
    ///         // ...
    ///     },
    ///     None => {
    ///         /// ...
    ///     }
    /// }
    /// ```
    pub fn get(&self, selector: Selector) -> Option<&T> {
        match self.get_index(selector) {
            Some(index) => self.data.get(index),
            None => None,
        }
    }

    /// Get a mutable reference to the specific cell
    ///
    /// # Arguments
    /// * `selector` - Selector to the specific cell
    ///
    /// # Examples
    /// ```
    /// let mut matrix = Matrix::new(7, 3);
    /// // ...
    /// match matrix.get_mut((4, 2)) {
    ///     Ok(reference) => {
    ///         // ...
    ///     },
    ///     None => {
    ///         /// ...
    ///     }
    /// }
    /// ```
    pub fn get_mut(&mut self, selector: Selector) -> Option<&mut T> {
        match self.get_index(selector) {
            Some(index) => self.data.get_mut(index),
            None => None,
        }
    }

    /// Get width of a matrix
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get height of a matrix
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<Selector> for Matrix<T>
where
    T: Default + Clone,
{
    type Output = T;

    /// Return read-only reference to the specific cell or panic in case of error
    ///
    /// # Arguments
    /// * `selector` - Selector of the specific cell
    ///
    /// # Examples
    /// ```
    /// let matrix = Matrix::new(3, 7);
    /// // ...
    /// let value = matrix[(0, 0)]
    /// ```
    fn index(&self, selector: Selector) -> &Self::Output {
        self.get(selector)
            .unwrap_or_else(|| panic!("Invalid index ({}, {})", selector.0, selector.1))
    }
}

impl<T> IndexMut<Selector> for Matrix<T>
where
    T: Default + Clone,
{
    /// Return read-only reference to the specific cell or panic in case of error
    ///
    /// # Arguments
    /// * `selector` - Selector of the specific cell
    ///
    /// # Examples
    /// ```
    /// let matrix = Matrix::new(3, 7);
    /// // ...
    /// let value = matrix[(0, 0)]
    /// ```
    fn index_mut(&mut self, selector: Selector) -> &mut Self::Output {
        self.get_mut(selector)
            .unwrap_or_else(|| panic!("Invalid index ({}, {})", selector.0, selector.1))
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Default + Clone + fmt::Display,
{
    /// Implement displaying a matrix
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() - 1 {
                write!(f, "{} ", self[(y, x)])?;
            }
            writeln!(f, "{}", self[(y, self.width() - 1)])?;
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
