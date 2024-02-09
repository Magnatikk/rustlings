// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

///L'objectif de cet exercice est simplement de récuperer la valeur en deuxième position dans le tuple "numbers"
/// Pour cela, la solution la plus simple consiste à utiliser le ".1" pour aller récupérer la valeur en 2eme position

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
