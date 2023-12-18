/// Represents a prediction structure that is able to predict 
/// - Has a sequence of numbers as a vector of **i64**
pub struct Prediction {
    numbers: Vec<i64>,
}

pub trait PredictionMethods {
    fn with_numbers(numbers: &str) -> Result<Self, String> where Self: Sized;
    fn predict_next_number(&self) -> i64;
    fn predict_first_number(&self) -> i64;
}

impl PredictionMethods for Prediction {
    /// Create a new prediction struct
    ///  - Takes a string with numbers 
    ///  - Will throw an error if it could not parse the number
    /// 
    /// Returns the prediction struct
    fn with_numbers(numbers: &str) -> Result<Self, String> where Self: Sized {
        let numbers_parsed: Result<Vec<i64>, String> = numbers
        .split_whitespace()
        .map(|numb| numb.parse::<i64>().map_err(|err| err.to_string()))
        .collect();

        Ok(Self {
            numbers: numbers_parsed?,
        })
    }
    
    /// Given the sequence of numbers, predict the next number in the sequence.
    /// Returns the next number in the sequence 
    fn predict_next_number(&self) -> i64 {
        let mut increment_numbers: Vec<i64> = Vec::new();
        let mut current_numbers: Vec<i64> = self.numbers.clone();
        increment_numbers.push(current_numbers.last().unwrap().clone());

        loop {
            // Get the differences
            let diff = get_next_sequence(&current_numbers);

            // Check if the all numbers in the sequence are zeros
            if is_all_zeros(&diff) {
                break;
            }
            increment_numbers.push(diff.last().unwrap().clone());

            current_numbers = diff;
        }

        let mut result: i64 = 0;

        let mut count = 0;
        while count < increment_numbers.len(){
            let current_increment = increment_numbers[count];
            let val = result + current_increment;
            result = val;
            count+=1;
        }

        result
    }

    /// Predict a number before the first number in the sequence.
    /// Returns this number
    fn predict_first_number(&self) -> i64 {
        let mut increment_numbers: Vec<i64> = Vec::new();
        let mut current_numbers: Vec<i64> = self.numbers.clone();
        increment_numbers.push(current_numbers.first().unwrap().clone());

        loop {
            // Get the differences
            let diff = get_next_sequence(&current_numbers);

            // Check if the all numbers in the sequence are zeros
            if is_all_zeros(&diff) {
                break;
            }
            increment_numbers.push(diff.first().unwrap().clone());

            current_numbers = diff;
        }

        // Reverse the string
        increment_numbers.reverse();

        let mut result: i64 = 0;

        let mut count = 0;
        while count < increment_numbers.len(){
            let current_increment = increment_numbers[count];
            let val = current_increment - result;
            result = val;
            count+=1;
        }

        result
    }
}

/// Function that returns a vector of the differences between each number
fn get_next_sequence(numbers: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    for i in 0..numbers.len() - 1 {
        result.push(numbers[i + 1] - numbers[i])
    }
    result
}

/// Returns true if all values are 0 in the vector
fn is_all_zeros(numbers: &Vec<i64>) -> bool {
    for numb in numbers {
        if *numb != 0 as i64 {
            return false;
        }
    }
    true
}
