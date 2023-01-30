fn main() {
    let input = String::from("Hi my name is John ee Pee");
    println!("{}", encode(&input));
}

fn encode(text: &str) -> String {
    let mut result = String::new();
    let vowel_add = "hay";
    let consonant_add = "ay";
    let vowels = ["a", "o", "i", "e", "u", "y"];
    for word in text.split_whitespace() {   
        let (first, last) = word.split_at(1);
        let new_word = if vowels.contains(&first) {
             format!("{word}-{vowel_add}")    
        } else {
             format!("{last}-{first}{consonant_add}")  
        };
        result.push_str(&new_word);
        result.push(' '); 
    } 
    result
}
