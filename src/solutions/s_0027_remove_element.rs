#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut max = nums.len();
        let mut i = 0;
        while max > i {
            if nums[i] == val {
                nums.swap(i, max - 1);
                max -= 1;
            } else {
                i += 1;
            }
        }

        max as i32
    }
}

fn test_vec(mut vec: Vec<i32>, val: i32, expect: Vec<i32>) {
    let k = Solution::remove_element(&mut vec, val);
    assert_eq!(
        k as usize,
        vec.len() - vec.iter().filter(|v| **v == val).count()
    );
    let temp = &mut vec[..k as usize];
    temp.sort();
    assert_eq!(temp, expect);
}

#[test]
fn test1() {
    test_vec(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]);
}

#[test]
fn test2() {
    test_vec(vec![3, 2, 2, 3], 3, vec![2, 2]);
}
