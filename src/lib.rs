mod dual;

#[cfg(test)]
mod tests {
    use super::dual::*;
    #[test]
    fn basic_dual() {
        let a = dual!(2.0, 3.0);
        let b = dual!(1.0, 2.0);

        assert_eq!(a + b, dual!(3.0, 5.0));
        assert_eq!(a - b, dual!(1.0, 1.0));
        assert_eq!(a * b, dual!(2.0, 7.0));
        assert_eq!(a / b, dual!(2.0, -1.0));
    }

    #[test]
    fn derivative() {
        let a: Dual<f64> = dual!(3.0, 1.0);
        // f(x) = e^arctan(sqrt(x))
        // f'(3) = (e^(pi/3))/(8sqrt(3))

        let deriv_a = a.sqrt().atan().exp();
        assert!(deriv_a.b - (std::f64::consts::FRAC_PI_3).exp() / (8.0 * 3.0_f64.sqrt()) <= std::f64::EPSILON)
    }
}
