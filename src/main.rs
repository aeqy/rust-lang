use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;

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
        if !guess.chars().all(|c| c.is_ascii_digit()) || guess.is_empty() {
            println!("只允许输入数字！");
            continue;
        }
        // 将输入字符串解析为 u32，并检查范围
        let guess: u32 = match guess.parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                println!("请输入 1 到 100 之间的数字！");
                continue;
            }
        };
        attempts += 1; // 每次有效猜测次数加一
        // 判断用户输入与答案的关系
        if guess == secret {
            let duration = start_time.elapsed(); // 计算用时
            println!("恭喜你，猜对了！一共猜了 {} 次。", attempts);
            println!("总共用时：{:.2} 秒。", duration.as_secs_f64());
            break; // 猜对则退出循环
        } else if guess < secret {
            println!("太小了！");
        } else {
            println!("太大了！");
        }
    }
} 