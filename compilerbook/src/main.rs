use std::env;

enum TokenKind{
    TokenReserved,
    TokenNumber,
}

struct Token{
    kind: TokenKind,
    str: String,
    val: Option<i32>,

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("引数の数が正しくありません。")
    }
    let formula: String = args.get(1).unwrap().clone();
    let tokenized_formula: Vec<Token> = tokenize(formula);
    let formula_length: usize = tokenized_formula.len();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", tokenized_formula.get(0).unwrap().str);
    for i in 1..(formula_length-1){
        let token: String = tokenized_formula.get(i).unwrap().str.clone();
        let next_token: String = tokenized_formula.get(i+1).unwrap().str.clone();
        if token == "+"{
            println!("  add rax, {}", next_token);
        } else if token == "-"{
            println!("  sub rax, {}", next_token);
        }
    }
    println!("  ret");
    
}

fn tokenize(input: String) -> Vec<Token> {
    let input = input.chars().collect::<Vec<char>>();
    let mut result: Vec<Token> = Vec::<Token>::new();
    let mut i: usize = 0;
    let mut num: String = get_empty_string();
    while i < input.len() {
        let c: char = input[i];
        if c.is_whitespace(){
            i += 1;
            num = get_empty_string();
            continue;
        }
        if c == '+' || c == '-' {
            i += 1;
            result.push(Token{kind: TokenKind::TokenNumber, str: num.clone(), val: Some(num.clone().parse().unwrap())});
            num = get_empty_string();
            result.push(Token{kind: TokenKind::TokenReserved, str: c.to_string(), val: None});
        }

        if c.is_digit(10){
            i += 1;
            num.push(c);
        }
    }
    if num != ""{
        result.push(Token{kind: TokenKind::TokenNumber, str: num.clone(), val: Some(num.clone().parse().unwrap())});
    }

    return result;
}

fn get_empty_string() -> String{
    return "".to_string();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}