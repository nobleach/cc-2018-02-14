fn main() {
    println!("Hello, world!");
}

fn lcm(n: u64) -> u64 {
    let mut ans = 1;
    for i in 1..(n + 1) {
        ans = (ans * i) / gcd(ans, i)
    }
    ans
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(4), 12);
    assert_eq!(lcm(10), 2520);
    assert_eq!(lcm(20), 232792560);
}
