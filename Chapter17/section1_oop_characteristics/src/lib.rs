pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn new() -> AveragedCollection {
    AveragedCollection {
      list: vec![],
      average: std::f64::NAN
    }
  }

  pub fn from_vec(list: Vec<i32>) -> AveragedCollection {
    let mut result = AveragedCollection {
      list,
      average: std::f64::NAN
    };
    result.update_average();
    result
  }

  pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      },
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_ok() {
    let mut averaged_collection = AveragedCollection::from_vec(vec![2, 4]);
    assert_eq!(3.0, averaged_collection.average());
    averaged_collection.add(6);
    assert_eq!(4.0, averaged_collection.average());
  }

  #[test]
  fn remove_ok() {
    let mut averaged_collection = AveragedCollection::from_vec(vec![2, 4]);
    assert_eq!(3.0, averaged_collection.average());
    averaged_collection.remove();
    assert_eq!(2.0, averaged_collection.average());
    averaged_collection.remove();
    assert!(averaged_collection.average().is_nan());
  }

  #[test]
  fn empty_average_ok() {
    let mut averaged_collection = AveragedCollection::new();
    assert!(averaged_collection.average().is_nan());
    averaged_collection.add(6);
    assert_eq!(6.0, averaged_collection.average());
    averaged_collection.remove();
    assert!(averaged_collection.average().is_nan());
    averaged_collection.remove();
    assert!(averaged_collection.average().is_nan());
  }
}

