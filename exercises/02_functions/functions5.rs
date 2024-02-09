// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

///Le but de cet exercice était de comprendre comment bien retourner une valeur d'une fonction
///  Plusieurs réponses étaient possibles, comme par exemple :
///     let good = num * num;
///     good
/// J'ai néanmoins pris le choix de simplement utiliser le mot clé "return" afin de renvoyer le carré du nombre
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return  num * num;
}
