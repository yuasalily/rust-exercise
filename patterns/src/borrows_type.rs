pub fn borrows_type_main(){
    println!("borrows_type start");

    // &strは当然渡せる
    let furious_str = "furious";
    let puddle_str = "puddle";
    assert_eq!(three_vowels(furious_str), true);
    assert_eq!(three_vowels(puddle_str), false);

    // &Stringは型変換して渡される
    let furious_string = String::from("furious");
    let puddle_string = String::from("puddle");
    assert_eq!(three_vowels(&furious_string), true);
    assert_eq!(three_vowels(&puddle_string), false);
    

    // 当然&[i32]は渡せる
    let array = [1,3,5,4,2];
    assert_eq!(max::<i32>(&array), 5);

    // Vec<u32>は型変換して渡される
    let v1: Vec<u32> = vec![1,2,3,5,9,3];
    assert_eq!(max(&v1), 9);

    println!("borrows_type end");
}

// 関数の引数は借用型を使った方が受け入れが広くなる
// 例えば&String -> &strは暗黙的に型変換が行われるので&strに&Stringは渡せる
// &str -> &Stringは型変換が行われないので&Stringに&strは渡せない。
fn three_vowels(word: &str) -> bool{
    let mut vowel_count = 0;
    for c in word.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            },
            _ => {vowel_count = 0;}
        }
    }
    return false;
}

fn max<T: PartialOrd + Copy>(array: &[T]) -> T{
    let mut result: T = array[0];
    for &item in array.iter() {
        result = if item > result {item} else {result};
    }
    return result;
}