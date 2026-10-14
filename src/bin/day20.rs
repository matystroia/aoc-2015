use std::collections::HashSet;

use aoc_15::read_lines;
use itertools::Itertools;

fn f(n: usize) -> usize {
    let (mut x, mut ans) = (n, 1);

    let mut d = 1;
    while x.is_multiple_of(2) {
        x /= 2;
        d *= 2;
        ans += d;
    }

    if x == 1 {
        return ans * 10;
    }

    let mut d = 3;
    while d <= x.isqrt() {
        let mut dd = 1;
        while x.is_multiple_of(d) {
            dd *= d;
            ans += dd;
            x /= d;
        }
        if x == 1 {
            return ans * 10;
        }
        d += 2;
    }

    ans += x;
    if x != n {
        ans += n;
    }

    ans * 10
}

fn f3(n: usize) -> usize {
    let mut x = n;
    let mut div = HashSet::new();
    let mut prime_div = 0;

    let mut d = 1;
    if x.is_multiple_of(2) {
        prime_div += 1;
    }
    while x.is_multiple_of(2) {
        x /= 2;
        d *= 2;
        div.insert(d);
    }

    if x == 1 {
        return (div.iter().sum::<usize>() + 1) * 10;
    }

    let mut d = 3;
    while d <= x.isqrt() {
        let mut dd = 1;
        if x.is_multiple_of(d) {
            prime_div += 1;
        }
        while x.is_multiple_of(d) {
            x /= d;
            dd *= d;
            for di in div.iter().cloned().collect_vec() {
                let mul = di * dd;
                if mul < n && n.is_multiple_of(mul) {
                    div.insert(mul);
                }
            }
            div.insert(dd);
        }
        d += 2;
    }

    if x == 1 {
        return (div.iter().sum::<usize>() + 1 + if prime_div > 1 { n } else { 0 }) * 10;
    }

    for di in div.iter().cloned().collect_vec() {
        let mul = di * x;
        if mul < n && n.is_multiple_of(mul) {
            div.insert(mul);
        }
    }

    if x == n {
        return (n + 1) * 10;
    }

    div.insert(x);
    (div.iter().sum::<usize>() + n + 1) * 10
}

fn f4(n: usize) -> usize {
    let mut x = n;
    let mut div = HashSet::new();
    let mut prime_div = 0;

    let mut d = 1;
    if x.is_multiple_of(2) {
        prime_div += 1;
    }
    while x.is_multiple_of(2) {
        x /= 2;
        d *= 2;
        div.insert(d);
    }

    if x == 1 {
        return (div.iter().filter(|d| n <= *d * 50).sum::<usize>() + if n <= 50 { 1 } else { 0 })
            * 11;
    }

    let mut d = 3;
    while d <= x.isqrt() {
        let mut dd = 1;
        if x.is_multiple_of(d) {
            prime_div += 1;
        }
        while x.is_multiple_of(d) {
            x /= d;
            dd *= d;
            for di in div.iter().cloned().collect_vec() {
                let mul = di * dd;
                if mul < n && n.is_multiple_of(mul) {
                    div.insert(mul);
                }
            }
            div.insert(dd);
        }
        d += 2;
    }

    if x == 1 {
        return (div.iter().filter(|d| n <= *d * 50).sum::<usize>()
            + if n <= 50 { 1 } else { 0 }
            + if prime_div > 1 { n } else { 0 })
            * 11;
    }

    for di in div.iter().cloned().collect_vec() {
        let mul = di * x;
        if mul < n && n.is_multiple_of(mul) {
            div.insert(mul);
        }
    }

    if x == n {
        return (n + if n <= 50 { 1 } else { 0 }) * 11;
    }

    div.insert(x);
    (div.iter().filter(|d| n <= *d * 50).sum::<usize>() + n + if n <= 50 { 1 } else { 0 }) * 11
}

fn f2(n: usize) -> usize {
    let mut ans = n + 1;

    let mut d = 2;
    while d <= n / 2 {
        if n.is_multiple_of(d) {
            ans += d;
        }
        d += 1;
    }

    ans * 10
}

fn main() {
    let n = read_lines(20)[0].parse::<usize>().unwrap();

    let mut x = 1;
    let ans = loop {
        if f4(x) >= n {
            break x;
        }
        x += 1;
    };

    println!("{ans}");

    // let mut x = 2097152;
    // while x > 1 {
    //     if f3(x) >= n {
    //         println!("{x}");
    //     }
    //     x -= 1;
    // }
    //
    // println!("{}", f2(68040));
}
