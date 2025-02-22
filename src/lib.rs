/// Returns the minumum/maximum value of x.count_ones() for x in range_min..range_max.

pub fn get_max(range_min: u8, range_max: u8) -> u32 {
  let different_bits = range_min ^ range_max;
  let highest_different_bit = 7 - different_bits.leading_zeros();
  let y = (1u8 << highest_different_bit) - 1;
  return (y | range_min).count_ones();
}

pub fn get_min(range_min: u8, range_max: u8) -> u32 {
  if range_min == 0 {
    return 0;
  }
  let different_bits = (range_min - 1) ^ (range_max - 1);
  let highest_different_bit = 7 - different_bits.leading_zeros();
  let p2 = 1u8 << highest_different_bit;
  let mask = !(p2 - 1);
  return ((range_min & mask) | p2).count_ones();
}

pub fn range_popcnt(range_min: u8, range_max: u8) -> (u32, u32) {
  return (get_min(range_min, range_max), get_max(range_min, range_max));
}

#[cfg(test)]
mod tests {
  use super::*;

  fn range_popcnt_naive(range_min: u8, range_max: u8) -> (u32, u32) {
    return (
      (range_min..range_max)
        .map(|x| x.count_ones())
        .min()
        .unwrap(),
      (range_min..range_max)
        .map(|x| x.count_ones())
        .max()
        .unwrap(),
    );
  }

  #[test]
  fn test_range_popcnt() {
    for range_max in 1..=255 {
      for range_min in 0..range_max {
        assert_eq!(
          range_popcnt(range_min, range_max),
          range_popcnt_naive(range_min, range_max)
        );
      }
    }
  }
}
