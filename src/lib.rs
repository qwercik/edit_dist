mod matrix;

use matrix::Matrix;

/// Calculate Levenshtein distance for two words
///
/// # Arguments
/// * `first_word` - First word
/// * `second_word` - Second word
///
/// # Examples
/// ```
/// use edit_distance::levenshtein;
/// let dist = levenshtein(
///     "lorem".char(),
///     "ipsum".char()
/// );
/// ```
pub fn levenshtein<T: PartialEq>(
    first_word: impl Iterator<Item = T>,
    second_word: impl Iterator<Item = T>,
) -> usize {
    let first_word: Vec<T> = first_word.collect();
    let second_word: Vec<T> = second_word.collect();

    let mut matrix = Matrix::<usize>::new(first_word.len() + 1, second_word.len() + 1);
    for y in 0..matrix.height() {
        matrix[(y, 0)] = y;
    }
    for x in 0..matrix.width() {
        matrix[(0, x)] = x;
    }

    for y in 1..matrix.height() {
        for x in 1..matrix.width() {
            let the_same_letter = first_word[x - 1] == second_word[y - 1];
            let cost = if the_same_letter { 0 } else { 1 };

            let values: Vec<usize> = vec![
                matrix[(y - 1, x - 1)] + cost,
                matrix[(y - 1, x)] + 1,
                matrix[(y, x - 1)] + 1,
            ];

            matrix[(y, x)] = values.into_iter().min().unwrap();
        }
    }

    matrix[(matrix.height() - 1, matrix.width() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_test_1() {
        let a = "sitting".chars();
        let b = "kitten".chars();
        assert_eq!(levenshtein(a, b), 3);
    }

    #[test]
    fn distance_test_2() {
        let a = "honda".chars();
        let b = "hyundai".chars();
        assert_eq!(levenshtein(a, b), 3);
    }

    #[test]
    fn distance_test_3() {
        let a = "gily".chars();
        let b = "geely".chars();
        assert_eq!(levenshtein(a, b), 2);
    }
}
