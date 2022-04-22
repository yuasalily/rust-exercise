use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("引数の数が正しくありません。")
    }
    let num: i32 = args[1].parse().unwrap();
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", num);
    println!("  ret");
    
}
