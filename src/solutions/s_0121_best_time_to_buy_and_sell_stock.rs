#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return  0;
        }
        let mut low = prices[0];
        let mut res = 0;


        for i in 0..prices.len() {
            if prices[i] < low {
                low = prices[i];
            }

            res = std::cmp::max(res, prices[i] - low);
        }

        res
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    assert_eq!(Solution::max_profit(vec![1,2]), 1);
}

#[test]
fn test2() {
    assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
}

#[test]
fn test3() {
    let mut big = Vec::from_iter(1..100_00);
    big.reverse();
    assert_eq!(Solution::max_profit(big), 0);
}

#[test]
fn test4() {
    assert_eq!(Solution::max_profit(vec![3, 10, 2, 5]), 7);
}
