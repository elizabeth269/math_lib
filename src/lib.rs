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
