use std::collections::HashMap;

fn memoized_recursive_solution(i: usize, j: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if i == 1 && j == 1 {
        return 1;
    }
    if i == 0 || j == 0 {
        return 0;
    }
    if let Some(&cached) = memo.get(&(i, j)) {
        return cached;
    }
    let result =
        memoized_recursive_solution(i - 1, j, memo) + memoized_recursive_solution(i, j - 1, memo);
    memo.insert((i, j), result);
    result
}

fn can_sum(target: usize, numbers: &[usize]) -> bool {
    let mut reachable = vec![false; target + 1];
    reachable[target] = true;
    for remainder in (0..=target).rev() {
        if reachable[remainder] {
            if remainder == 0 {
                return true;
            }
            for &num in numbers {
                if num <= remainder {
                    let new_remainder = remainder - num;
                    reachable[new_remainder] = true;
                }
            }
        }
    }

    reachable[0]
}

fn cansum(target: usize, numbers: &[usize]) -> bool {
    let mut flag: bool = false;
    if target==0 {
        flag = true;
    }
    for &num in numbers {
        let remainder = target - num;
        if remainder 
    }
}


fn main() {}

