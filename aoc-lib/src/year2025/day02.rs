// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2025, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let ids: Vec<&str> = _input.trim().split(',').collect();

    let mut total = 0;

    for id in ids {
        let split: Vec<String> = id.split("-").map(|s| s.to_string()).collect();

        let id1: u64 = split[0].parse().unwrap();
        let id2: u64 = split[1].parse().unwrap();

        for id in id1..id2 + 1 {
            let id_to_string = id.to_string();

            if id_to_string.len() % 2 == 0 {
                let mid = id_to_string.len() / 2;
                let first_part = &id_to_string[..mid];
                let second_part = &id_to_string[mid..];

                if first_part == second_part {
                    total += id
                }
            }
        }
    }

    Ok(total)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let ids: Vec<&str> = _input.trim().split(',').collect();

    let mut total = 0;

    for id_range in ids {
        let split: Vec<&str> = id_range.split("-").collect();

        let id1: u64 = split[0].parse().unwrap();
        let id2: u64 = split[1].parse().unwrap();

        for id in id1..=id2 {
            let id_to_string = id.to_string();

            let doubled = format!("{}{}", id_to_string, id_to_string);

            let searched_area = &doubled[1..doubled.len() - 1];

            if searched_area.contains(&id_to_string) {
                total += id;
            }
        }
    }

    Ok(total)
}
