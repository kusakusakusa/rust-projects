fn main() {
    let given_string = "apple";
    let mut output = String::new();

    match match &given_string[0..1] {
        "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => Character::Vowel,
        _ => Character::Consonant,
    } {
        Character::Consonant => {
            output.push_str(&given_string[1..]);
            output.push_str(&given_string[0..1]);
            output.push_str("ay");
        },
        Character::Vowel => {
            output.push_str(&given_string);
            output.push_str("hay");
        }
    }
    println!("Answer: {output}");
}

enum Character {
    Consonant,
    Vowel
}