// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

///Pour le 1er exercice de ce fichier, on nous demande simplement de multiplier "element" par 2 dans un vecteur mutable
/// Il nous suffit donc de faire un "*=" par 2 sur la référence de "element" dans la loop de v.iter_mut

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

///Pour le 2eme exercice, on nous demande de faire la meme chose, cette fois ci sur un vecteur non mutable.
/// Pour ce faire, un iterateur est créé sur "v" avec "iter()", map() itère sur "iter()" et appelle la fonction présente
/// à l'intèrieur d'elle sur chaque élément de l'itérateur.
///".collect()" permet enfin de transformer l'itérateur en collection (en Vec dans ce cas là)
///La solution est donc tout simplement de faire element*2 !

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        element*2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
