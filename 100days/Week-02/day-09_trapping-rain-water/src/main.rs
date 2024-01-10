fn main() {
    println!("Hello, world!");
}

fn trap_rain(heights: Vec<i32>) -> i32 {
    let prev: i32 = 0;
    let all_sum: i32 = 0;
    let sum: i32 = 0;
    let start: i32 = 0;

    for h in heights {
        if h <= prev {
            sum = sum + prev - h;
            start = prev;
        } else {
            if h > start {
                start = h;
                sumall = all_sum + sum;
                sum = 0;
            } else {
                sum = sum + h - prev;
            }
        }
        prev = h;
    }
}

#[test]
fn test_trap1() {
    assert_eq!(trap_rain(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
}

#[test]
fn test_trap2() {
    assert_eq!(trap_rain(vec![4, 2, 0, 3, 2, 5]), 9)
}
