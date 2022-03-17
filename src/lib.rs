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
        let a = dual!(2.0, 1.0);
        let da = (a * a + a).b;
        assert_eq!(da, 5.0);
    }
}
