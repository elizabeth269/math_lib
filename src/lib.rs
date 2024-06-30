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

use std::f64::consts::PI;

/// Converts degrees to radians.
pub fn to_radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

/// Computes the sine of a number (in radians).
pub fn sin(radians: f64) -> f64 {
    radians.sin()
}

/// Computes the cosine of a number (in radians).
pub fn cos(radians: f64) -> f64 {
    radians.cos()
}

/// Computes the tangent of a number (in radians).
pub fn tan(radians: f64) -> f64 {
    radians.tan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(1.0, 0.0).is_err());
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2.0, 3.0), 8.0);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(9.0).unwrap(), 3.0);
    }

    #[test]
    fn test_sqrt_negative() {
        assert!(sqrt(-1.0).is_err());
    }

    #[test]
    fn test_to_radians() {
        assert_eq!(to_radians(180.0), std::f64::consts::PI);
    }

    #[test]
    fn test_sin() {
        assert_eq!(sin(to_radians(30.0)), 0.5);
    }

    #[test]
    fn test_cos() {
        assert_eq!(cos(to_radians(60.0)), 0.5);
    }

    #[test]
    fn test_tan() {
        assert!((tan(to_radians(45.0)) - 1.0).abs() < 1e-10);
    }
}
