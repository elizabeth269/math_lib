/// Adds two numbers together.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts the second number from the first number.
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers together.
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides the first number by the second number.
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

/// Raises a number to the power of another number.
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Returns the square root of a number.
pub fn sqrt(number: f64) -> Result<f64, &'static str> {
    if number < 0.0 {
        Err("Cannot compute the square root of a negative number")
    } else {
        Ok(number.sqrt())
    }
}
