use std::{
    collections::{BTreeSet, HashSet},
    iter::once,
};

use smartstring::alias::String;

pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().map(Result::unwrap);

    let direct_connections = input.map(|line| {
        let (l, r) = line.split_once('-').unwrap();
        (String::from(l), String::from(r))
    });

    let mut adjacency = HashMap::<String, HashSet<String>>::new();
    for (a, b) in direct_connections {
        adjacency.entry(a.clone()).or_default().insert(b.clone());
        adjacency.entry(b).or_default().insert(a);
    }

    let mut cliques: HashSet<BTreeSet<String>> = adjacency
        .keys()
        .cloned()
        .map(|x| BTreeSet::from_iter([x]))
        .collect();

    let mut clique_size = 1;
    while cliques.len() > 1 {
        cliques = cliques
            .into_iter()
            .flat_map(|clique| {
                // Look for neighbors to add to the clique
                adjacency
                    .iter()
                    .filter(|(_, neighbors)| clique.iter().all(|n| neighbors.contains(n)))
                    .map(|(k, _)| {
                        BTreeSet::from_iter(clique.iter().cloned().chain(once(k.clone())))
                    })
                    .collect_vec()
            })
            .collect();
        clique_size += 1;
        println!("{} cliques of size {}", cliques.len(), clique_size);
    }

    println!("{}", cliques.iter().next().unwrap().iter().join(","));

    Ok(())
}
