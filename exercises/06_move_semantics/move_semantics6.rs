// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

///Le but de cet exercice est de bien comprendre les enjeux d'emprunts
/// Pour la fonction get_char(), il faut passer la référence (&) de data, afin de ne pas transférer la propriété
/// du String
/// Pour la 2eme fonction, "data" est passée tel quel afin de pouvoir consommer le String (prendre l'ownership)
/// Il fallait donc enlever le & dans string_uppercase(data) et faire les changements en conséquence.
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
