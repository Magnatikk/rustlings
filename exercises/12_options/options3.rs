// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

///Pour cet exercice, il fallait utiliser le mot clé "ref"
/// Car par défaut, match consomme tout ce qu'il peut, ce qui peut créer des problèmes
/// Avec "ref", la valeur est seulement borrowed, et non bougée, ce qui fait quelle reste disponible apres le match
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
