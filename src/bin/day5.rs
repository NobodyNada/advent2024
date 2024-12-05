pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let mut input = c.input.lines().map(Result::unwrap);

    let rules = (&mut input)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once("|").context("| expected")?;
            Ok((before.parse::<u32>()?, after.parse::<u32>()?))
        });

    let mut rules_map = HashMap::new();
    for rule in rules {
        let rule = rule?;
        rules_map.entry(rule.1).or_insert(Vec::new()).push(rule.0);
    }

    let updates = input.map(|line| {
        line.split(',')
            .map(|x| x.parse::<u32>().expect("number expected"))
            .collect::<Vec<u32>>()
    });

    if !c.part2 {
        let valid = updates.filter(|update| {
            for (i, page) in update.iter().enumerate() {
                // For everything that needs to come before this page, make sure it doesn't appear
                // after
                for ordered_before in rules_map.get(page).unwrap_or(&Vec::new()) {
                    if update[i + 1..].contains(ordered_before) {
                        return false;
                    }
                }
            }
            true
        });

        println!(
            "{}",
            valid.map(|update| update[update.len() / 2]).sum::<u32>()
        );
    } else {
        let invalid = updates.filter_map(|mut update| {
            let mut i = 0;
            let mut valid = true;
            while i < update.len() {
                // Find the last index of a page that is ordered before this page.
                let last_index = rules_map
                    .get(&update[i])
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter_map(|ordered_before| {
                        update
                            .iter()
                            .enumerate()
                            .skip(i)
                            .find(|(_, page)| *page == ordered_before)
                            .map(|(index, _)| index)
                    })
                    .max();

                // Move the current page to come right after that last index.
                if let Some(last_index) = last_index {
                    valid = false;
                    update[i..=last_index].rotate_left(1);
                } else {
                    i += 1;
                }
            }
            if valid { None } else { Some(update) }
        });

        println!(
            "{}",
            invalid.map(|update| update[update.len() / 2]).sum::<u32>()
        );
    }

    Ok(())
}
