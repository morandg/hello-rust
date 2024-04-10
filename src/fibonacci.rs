pub struct FibonacciSolutions {
  solutions: Vec<u32>,
}

impl FibonacciSolutions {
  pub fn new() -> FibonacciSolutions {
    FibonacciSolutions {
      solutions: vec![0, 1],
    }
  }

  pub fn find_solution(&mut self, n: u32) -> u32 {
    let solutions_idx = usize::try_from(n).unwrap();

    if solutions_idx + 1 > self.solutions.len() {
      let new_solution = self.find_solution(n - 1) + self.find_solution(n - 2);
      self.solutions.push(new_solution);
    }

    self.solutions[solutions_idx]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibo_of_0_is_0() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(0, fibo_solutions.find_solution(0));
  }

    #[test]
    fn test_fibo_of_1_is_1() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(1, fibo_solutions.find_solution(1));
  }

  #[test]
  fn test_fibo_of_3_is_2() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(2, fibo_solutions.find_solution(3));
  }
}
