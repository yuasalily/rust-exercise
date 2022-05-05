
pub fn borrows_type_main(){
    let furious_str = "furious";
    let furious_string = String::from("furious");
    let puddle_str = "puddle";
    let puddle_string = String::from("puddle");
    assert_eq!(three_vowels(furious_str), true);
    assert_eq!(three_vowels(&furious_string), true);
    assert_eq!(three_vowels(puddle_str), false);
    assert_eq!(three_vowels(&puddle_string), false);
    println!("borrows_type end");

}

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