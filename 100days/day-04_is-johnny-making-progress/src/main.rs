
fn main() {
    println!("Hello, world!");
    let mut input: [i32; 4] = [3, 4, 1, 2];
    println!("{}",progress_days(&mut input));
}

fn progress_days(days: &mut [i32]) -> i32 {
    let mut count: i32 = 0;
    for wind in days.windows(2){
        if wind[0] < wind[1] {
            count += 1;
        }
    }
    return count;
}

#[test]
fn test_prog_1() {
    let mut input: [i32; 4] = [3, 4, 1, 2];
    assert_eq!(progress_days(&mut input), 2)
}

#[test]
fn test_prog_2() {
    let mut input: [i32; 5] = [10, 11, 12, 9, 10];
    assert_eq!(progress_days(&mut input), 3)
}

#[test]
fn test_prog_3() {
    let mut input: [i32; 6] = [6, 5, 4, 3, 2, 9];
    assert_eq!(progress_days(&mut input), 1)
}

#[test]
fn test_prog_4() {
    let mut input: [i32; 2] = [9, 9];
    assert_eq!(progress_days(&mut input), 0)
}
