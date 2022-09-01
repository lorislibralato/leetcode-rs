#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                k += 1;
            }
        }

        // dbg!(k);

        let mut last_swap_idx = nums.len();
        for i in 0..k - 1 {
            // dbg!(i, &nums);
            while nums[i] == nums[i + 1] {
                let mut ii = i;
                while ii + 1 < last_swap_idx {
                    nums[ii] = nums[ii + 1];
                    ii += 1;
                }
                last_swap_idx -= 1;
            }
            if nums[i] > nums[i + 1] {
                break;
            }
        }
        k as i32
    }
}

#[test]
fn test1() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums[..2], vec![1, 2]);
}

#[test]
fn test2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
}

#[test]
fn test3() {
    let mut nums = vec![0, 0, 0, 0];
    assert_eq!(Solution::remove_duplicates(&mut nums), 1);
    assert_eq!(nums[..1], vec![0]);
}

#[test]
fn test4() {
    let mut nums = vec![0];
    assert_eq!(Solution::remove_duplicates(&mut nums), 1);
    assert_eq!(nums, vec![0]);
}

#[test]
fn test5() {
    let mut nums = vec![];
    assert_eq!(Solution::remove_duplicates(&mut nums), 0);
    assert_eq!(nums, vec![0; 0]);
}

#[test]
fn test6() {
    let mut nums = vec![1, 2, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2, 2]);
}

#[test]
fn test7() {
    let mut nums = (-50..=50).collect();
    assert_eq!(Solution::remove_duplicates(&mut nums), 101);
    assert_eq!(nums, (-50..=50).collect::<Vec<_>>());
}
