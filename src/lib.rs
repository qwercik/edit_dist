mod matrix;

use matrix::Matrix;

pub fn distance<T: PartialEq>(first_word: &[T], second_word: &[T]) -> usize {
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
        let a = "sitting".chars().collect::<Vec<char>>();
        let b = "kitten".chars().collect::<Vec<char>>();
        assert_eq!(distance(&a, &b), 3);
    }

    #[test]
    fn distance_test_2() {
        let a = "honda".chars().collect::<Vec<char>>();
        let b = "hyundai".chars().collect::<Vec<char>>();
        assert_eq!(distance(&a, &b), 3);
    }

    #[test]
    fn distance_test_3() {
        let a = "gily".chars().collect::<Vec<char>>();
        let b = "geely".chars().collect::<Vec<char>>();
        assert_eq!(distance(&a, &b), 2);
    }
}
