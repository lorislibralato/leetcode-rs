#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
		let mut res: i32 = 0;

        let negative_num = (dividend < 0 || divisor < 0) && !(dividend < 0 && divisor < 0);
        let mut dividend = if dividend < 0 { dividend.abs() } else { dividend };
        let mut divisor = if divisor < 0 { divisor.abs() } else { divisor };

        while dividend >= divisor {
            dividend -= divisor;
            res += 1;
        }

        if negative_num {
            res.abs()
        } else {
            res
        }
    }
}

#[test]
fn test1() {
	assert_eq!(Solution::divide(10, 3), 3)
}

#[test]
fn test2() {
	assert_eq!(Solution::divide(7, -3), -2)
}

#[test]
fn test3() {
	assert_eq!(Solution::divide(-2147483648, -1), 2147483647
)
}