use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;

// 提取输入校验函数
fn parse_guess(input: &str, min: u32, max: u32) -> Option<u32> {
    let input = input.trim();
    if !input.chars().all(|c| c.is_ascii_digit()) || input.is_empty() {
        return None;
    }
    match input.parse() {
        Ok(num) if num >= min && num <= max => Some(num),
        _ => None,
    }
}

// 提取猜测判断函数
fn check_guess(guess: u32, secret: u32) -> &'static str {
    if guess == secret {
        "equal"
    } else if guess < secret {
        "less"
    } else {
        "greater"
    }
}

fn main() {
    // 生成一个 1 到 100 之间的随机数，作为答案
    let secret = rand::rng().random_range(1..=100);
    let mut attempts = 0; // 猜测次数统计
    let start_time = Instant::now(); // 记录开始时间
    println!("请输入一个 1 到 100 之间的数字：");
    loop {
        // 提示用户输入
        print!("你的猜测: ");
        io::stdout().flush().unwrap(); // 刷新输出缓冲区，确保提示立即显示
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap(); // 读取用户输入
        let guess = guess.trim(); // 去除首尾空白字符
        // 只允许输入数字
        let guess: u32 = match parse_guess(guess, 1, 100) {
            Some(num) => num,
            None => {
                println!("只允许输入 1 到 100 之间的数字！");
                continue;
            }
        };
        attempts += 1; // 每次有效猜测次数加一
        // 判断用户输入与答案的关系
        match check_guess(guess, secret) {
            "equal" => {
                let duration = start_time.elapsed(); // 计算用时
                println!("恭喜你，猜对了！一共猜了 {} 次。", attempts);
                println!("总共用时：{:.2} 秒。", duration.as_secs_f64());
                break; // 猜对则退出循环
            }
            "less" => println!("太小了！"),
            "greater" => println!("太大了！"),
            _ => unreachable!(),
        }
    }
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_guess_valid() {
        assert_eq!(parse_guess("42", 1, 100), Some(42));
        assert_eq!(parse_guess("  1 ", 1, 100), Some(1));
        assert_eq!(parse_guess("100", 1, 100), Some(100));
    }

    #[test]
    fn test_parse_guess_invalid() {
        assert_eq!(parse_guess("abc", 1, 100), None);
        assert_eq!(parse_guess("", 1, 100), None);
        assert_eq!(parse_guess("0", 1, 100), None);
        assert_eq!(parse_guess("101", 1, 100), None);
    }

    #[test]
    fn test_check_guess() {
        assert_eq!(check_guess(50, 50), "equal");
        assert_eq!(check_guess(30, 50), "less");
        assert_eq!(check_guess(70, 50), "greater");
    }
} 