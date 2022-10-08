
fn main() {
    let nums = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut count = 0;

    for window in nums.windows(2) {
       if window[0] < window[1] {
           count += 1;
       }
    }

    println!("{}", count);
}
