fn is_not_zero(num: int) -> bool {
  num != 0
}

fn div_by_five(num: int) -> bool {
  if is_not_zero(num) {
    num % 5 == 0
  } else {
    false
  }
}

fn div_by_three(num: int) -> bool {
  if is_not_zero(num) {
    num % 3 == 0
  } else {
    false
  }
}

//Tests for is_not_zero
#[test]
fn test_is_not_zero_with_zero() {
  if is_not_zero(0) {
    fail!("Zero is zero");
  }
}

#[test]
fn test_is_not_zero_with_one() {
  if !is_not_zero(1) {
    fail!("One is not zero");
  }
}

//Tests for div_by_three

#[test]
fn test_div_by_three_with_one() {
  if div_by_three(1) {
    fail!("One is not divisible by three");
  }
}

#[test]
fn test_div_by_three_with_three() {
  if !div_by_three(3) {
    fail!("Three should be divisible by three");
  }
}

#[test]
fn test_div_by_three_with_zero() {
  if div_by_three(0) {
    fail!("Zero is not divisible by three");
  }
}

#[test]
fn test_div_by_three_with_fifteen() {
  if !div_by_three(15) {
    fail!("Fifteen is divisible by three");
  }
}

//Tests for div_by_five

#[test]
fn test_div_by_five_with_one() {
  if div_by_five(1) {
    fail!("One is not divisible by five");
  }
}

#[test]
fn test_div_by_five_with_five() {
  if !div_by_five(5) {
    fail!("Five should be divisible by five");
  }
}

#[test]
fn test_div_by_five_with_zero() {
  if div_by_five(0) {
    fail!("Zero is not divisible by five");
  }
}

#[test]
fn test_div_by_five_with_fifteen() {
  if !div_by_five(15) {
    fail!("Fifteen is divisible by five");
  }
}
