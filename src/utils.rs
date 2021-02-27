pub fn mean(x: &Vec<f64>) -> f64 {
  let sum: f64 = x.iter().sum();
  let n: f64 = x.len() as f64;
  return sum / n;
}

pub fn cumsum(x: &Vec<f64>) -> Vec<f64> {
  let result: Vec<f64> = x.iter()
  .scan(0f64, |acc, &a| {
      *acc = *acc + a;
      Some(*acc)
  }).collect();
  return result;
}

pub fn minmax(x: &Vec<f64>) -> (f64, f64) {
  return x.iter().fold((x[0], x[0]), |acc, &x| (acc.0.min(x), acc.1.max(x)));
}