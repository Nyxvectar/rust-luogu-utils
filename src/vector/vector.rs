/**
 * Author:  Nyxvectar Yan
 * Repo:    luogu-utils
 * Created: 01/30/2025
 */

mod vector {
    use std::fmt;
    use std::ops::{Add, Mul, Sub};

    #[derive (Debug, Clone, PartialEq, Eq)]
    pub struct BigInt {
        digits: Vec<u64>,
        is_negative: bool,
    }

    impl BigInt {
        const BASE: u64 = 1_000_000_000_000_000_000;
        const BASE_STR: usize = 18;

        pub fn from_u64 (n: u64) -> Self {
            if n == 0 {
                BigInt {
                    digits: Vec::new (),
                    is_negative: false,
                }
            } else {
                BigInt {
                    digits: vec![n],
                    is_negative: false,
                }
            }
        }

        pub fn from_str (s: &str) -> Result<Self, String> {
            let mut s = s.trim ();
            let mut is_negative = false;

            if s.starts_with ('-') {
                is_negative = true;
                s = &s [1..];
                if s.is_empty () {
                    return Err ("无效的数字字符串：仅包含负号".to_string ());
                }
            }

            let s = s.trim_start_matches ('0');
            if s.is_empty () {
                return Ok (BigInt {
                    digits: Vec::new (),
                    is_negative: false,
                });
            }

            let mut digits = Vec::new();
            let mut remaining = s;

            while !remaining.is_empty () {
                let take = remaining.len ().min (Self::BASE_STR);
                let (part, rest) = if remaining.len () > take {
                    (&remaining [0..remaining.len () - take], &remaining [remaining.len () - take..])
                } else {
                    ("", remaining)
                };
                remaining = part;

                let num = rest
                    .parse::<u64>()
                    .map_err (|_| format!("无效的数字部分: {}", rest))?;
                digits.push (num);
            }

            Ok(BigInt {
                digits,
                is_negative,
            })
        }

        pub fn is_zero (&self) -> bool {
            self.digits.is_empty ()
        }

        fn trim_leading_zeros (&mut self) {
            while let Some (&0) = self.digits.last () {
                self.digits.pop ();
            }
        }

        fn greater_than_unsigned (&self, other: &Self) -> bool {
            if self.digits.len () != other.digits.len () {
                return self.digits.len () > other.digits.len ();
            }

            for i in (0..self.digits.len()).rev() {
                if self.digits[i] > other.digits[i] {
                    return true;
                } else if self.digits[i] < other.digits[i] {
                    return false;
                }
            }
            false
        }
    }

    impl Add for &BigInt {
        type Output = BigInt;

        fn add (self, rhs: Self) -> Self::Output {
            if self.is_negative != rhs.is_negative {
                if self.is_negative {
                    let mut self_pos = self.clone ();
                    self_pos.is_negative = false;
                    rhs - &self_pos
                } else {
                    let mut rhs_pos = rhs.clone ();
                    rhs_pos.is_negative = false;
                    self - &rhs_pos
                }
            } else {
                let mut result = Vec::new ();
                let mut carry = 0;
                let max_len = self.digits.len ().max (rhs.digits.len ());

                for i in 0..max_len {
                    let a = self.digits.get(i).copied().unwrap_or(0);
                    let b = rhs.digits.get(i).copied().unwrap_or(0);

                    let sum = a as u128 + b as u128 + carry as u128;
                    result.push((sum % BigInt::BASE as u128) as u64);
                    carry = (sum / BigInt::BASE as u128) as u64;
                }

                if carry > 0 {
                    result.push(carry);
                }

                let mut big_int = BigInt {
                    digits: result,
                    is_negative: self.is_negative,
                };
                big_int.trim_leading_zeros();
                big_int
            }
        }
    }




}
