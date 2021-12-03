pub fn one() {
    let input = include_str!("../inputs/one.txt");
    let nums: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    println!("simple increase: {}", count_simple_incr(&nums));
    println!("sliding window: {}", count_sliding_window(&nums));
}

fn count_sliding_window(nums: &[u32]) -> i32 {
    let nums: Vec<u32> = nums.windows(3).map(|w| w.iter().sum()).collect();
    count_incr(&nums)
}

fn count_simple_incr(nums: &[u32]) -> i32 {
    count_incr(nums)
}

fn count_incr(nums: &[u32]) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let mut r = nums[0];
    let mut c = 0;
    for n in nums[1..].iter() {
        if *n > r {
            c += 1;
        }
        r = *n;
    }

    c
}
