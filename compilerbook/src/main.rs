use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("引数の数が正しくありません。")
    }
    let formula: String = args.get(1).unwrap().clone();
    let splited_formula: Vec<String> = split_formula(formula);
    let formula_length: usize = splited_formula.len();
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", splited_formula.get(0).unwrap());
    for i in 1..(formula_length-1){
        let token: String = splited_formula.get(i).unwrap().clone();
        let next_token: String = splited_formula.get(i+1).unwrap().clone();
        if token == "+"{
            println!("  add rax, {}", next_token);
        } else if token == "-"{
            println!("  sub rax, {}", next_token);
        }
    }
    println!("  ret");
    
}

fn split_formula(formula: String) -> Vec<String>{
    let formula = formula.replace(" ", "");
    let mut result: Vec<String> = Vec::<String>::new();
    let mut num: String = get_empty_string();
    for c in formula.as_str().chars(){
        if c == '+'{
            result.push(num.clone());
            result.push("+".to_string());
            num = get_empty_string();
        } else if c == '-'{
            result.push(num.clone());
            result.push("-".to_string());
            num = get_empty_string();
        } else {
            num.push(c);
        }
    }
    result.push(num.clone());
    return result;
}

fn get_empty_string() -> String{
    return "".to_string();
}
