pub struct Prediction {
    numbers: Vec<i64>,
}

pub trait PredictionMethods {
    fn with_numbers(numbers: &str) -> Result<Self, String> where Self: Sized;
    fn predict_next_number(&self) -> i64;
}

impl PredictionMethods for Prediction {
    fn with_numbers(numbers: &str) -> Result<Self, String> where Self: Sized {
        let numbers_parsed: Result<Vec<i64>, String> = numbers
        .split_whitespace()
        .map(|numb| numb.parse::<i64>().map_err(|err| err.to_string()))
        .collect();

        Ok(Self {
            numbers: numbers_parsed?,
        })
    }

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
}

fn get_next_sequence(numbers: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    for i in 0..numbers.len() - 1 {
        result.push(numbers[i + 1] - numbers[i])
    }
    result
}

fn is_all_zeros(numbers: &Vec<i64>) -> bool {
    for numb in numbers {
        if *numb != 0 as i64 {
            return false;
        }
    }
    true
}
