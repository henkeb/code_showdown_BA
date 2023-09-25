use std::collections::HashSet;

use crate::Movie;

pub fn lös(input: &[Movie]) -> usize {
    let mut knows_tom: HashSet<&str> = HashSet::new();

    knows_tom.insert("Tom Jones");

    let mut closure = || {
        let mut temp_set: HashSet<&str> = HashSet::new();
        input
            .iter()
            .map(|movie| &movie.cast)
            .map(|cast| cast.trim().split(", ").collect::<Vec<&str>>())
            .for_each(|cast| {
                for name in &cast {
                    if knows_tom.contains(name) {
                        for name in cast {
                            temp_set.insert(name);
                        }
                        break;
                    }
                }
            });
        knows_tom.extend(temp_set);
    };

    for _ in 0..4 {
        closure();
    }

    knows_tom.len() - 1
}
