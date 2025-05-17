use ::std::io::stdin;

fn main() {
    println!("请输入你的名字");
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    
    println!("你好,{}欢迎入门Rust Lang 程序!", input);
    
}
