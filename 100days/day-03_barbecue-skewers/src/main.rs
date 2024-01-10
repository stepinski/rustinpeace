
fn main() {
    println!("Hello, world!");
    cat_skweres(&[
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ]);
}


fn cat_skweres(skews: &[&str]) -> Vec<i32> {
    let mut nveg = 0;
    let mut veg = 0;
    
    for skew in skews {
        let fnd = skew.find("x");

        if fnd.is_none() {
            veg += 1;
        } else {
            nveg += 1;
        }
    }
    return vec![veg, nveg];
}

#[test]
fn test_skweres_01() {
    assert_eq!(
        cat_skweres(&[
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ]),
        [2, 3]
    )
}

#[test]
fn test_skweres_02() {
    assert_eq!(
        cat_skweres(&[
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ]),
        [3, 2]
    )
}
