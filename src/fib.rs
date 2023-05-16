// 0 1 1 2 3 5 8 13

use std::io::Write;

fn fib_upto(last_one: u32) -> Vec<u32> {
    let mut last: u32 = 0;
    let mut next: u32 = 1;
    let mut res: Vec<u32> = vec![last, next];
    loop {
        let tmp = next;
        next = last + next;
        last = tmp;
        if next > last_one {
            break;
        }
        res.push(next);
    }
    return res;
}

pub fn chatgpt_exercise() {
    use std::io::{stdin, stdout};
    let mut text = String::new();
    print!("Generate Fibonacci sequence up to a given number: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut text).expect("Failed to read input");
    match text.trim().parse::<u32>() {
        Ok(number) => println!("Fibonacci up to {}: {:?}", number, fib_upto(number)),
        Err(_) => println!("Not a number {}", text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_upto_8() {
        let res = fib_upto(8);
        assert_eq!(res, vec![0, 1, 1, 2, 3, 5, 8]);
    }
    // Not a fibonacci number:
    #[test]
    fn fib_upto_43() {
        let res = fib_upto(43);
        assert_eq!(res, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34])
    }
}
