pub struct FibonacciSolutions {
  solutions: Vec<u128>,
}

impl FibonacciSolutions {
  pub fn new() -> FibonacciSolutions {
    FibonacciSolutions {
      solutions: vec![0, 1],
    }
  }

  pub fn find_solution(&mut self, n: u128) -> Result<u128, String> {
    match usize::try_from(n) {
      Err(error) => Err(format!("Could not get index of the solution vector: {}", error)),
      Ok(solutions_idx) => self.compute_solution(solutions_idx, n),
    }
  }

  fn compute_solution(&mut self, solutions_idx: usize, n: u128) -> Result<u128, String> {
    if solutions_idx + 1 > self.solutions.len() {
      let fibo1 = self.find_solution(n - 1)?;
      let fibo2 = self.find_solution(n - 2)?;
      match fibo1.checked_add(fibo2) {
          Some(new_solution) => self.solutions.push(new_solution),
          None => return Err(format!("Fibo of {} overflow!", n)),
      }
    }
    Ok(self.solutions[solutions_idx])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibo_of_0_is_0() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(Ok(0), fibo_solutions.find_solution(0));
  }

    #[test]
    fn test_fibo_of_1_is_1() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(Ok(1), fibo_solutions.find_solution(1));
  }

  #[test]
  fn test_fibo_of_3_is_2() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert_eq!(Ok(2), fibo_solutions.find_solution(3));
  }
  #[test]
  fn test_fibo_of_big_number_overflows() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert!(matches!(fibo_solutions.find_solution(200), Err(_)));
  }

  #[test]
  fn test_fibo_of_big_number_overflows_vector_size() {
    let mut fibo_solutions = FibonacciSolutions::new();
    assert!(matches!(fibo_solutions.find_solution(u128::MAX), Err(_)));
  }
}
