pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo_of_0_is_0() {
        assert_eq!(0, fibonacci(0));
    }

    #[test]
    fn test_fibo_of_1_is_1()
    {
        assert_eq!(1, fibonacci(1));
    }

    #[test]
    fn test_fibo_of_3_is_2() {
        assert_eq!(2, fibonacci(3));
    }
}
