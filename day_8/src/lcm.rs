/// Function that calculated the greatest common devisor - gcd
/// - Takes two numbers as parameters 
/// - Uses a while loop to calculate the number
fn gcd(numb1: u64, numb2: u64) ->u64{
    let mut a: u64 = numb1;
    let mut b: u64 = numb2;

    // If the second number is already 0, we return the first number as gcd
    if b == 0 {
        return a;
    }

    // Use Euclid's algorithm to find GCD.
    while a % b != 0 {
        // Temporary store the new value 
        let temp = b.clone();

        // Update B to be the remainder of a mod b 
        b = a % b;

        // Set temp value to a 
        a = temp;
    }

    // The gcd will be B 
    b
}

/// Function that calculates the least common multiple 
/// Uses the formula => LCM(a, b) = (a * b) / GCD(a, b) 
fn lcm(a: u64, b: u64) -> u64 {
    // If either a or b are 0, we return 0 
    if a == 0 || b == 0 {
        0
    } else {
        // Uses the formula to calculate the LCM
        a * b / gcd(a, b)
    }
}

/// Given a vector of numbers, finds the least common multiple between the numbers
/// Returns the number 
pub fn find_lcm(numbers: Vec<u64>) -> u64 {
    let mut result = 1;

    // Iterating through each number and updating the result by
    // taking the lcm of the result and the number. 
    for &num in &numbers {
        result = lcm(result, num);
    }

    // Returns the result 
    result
}