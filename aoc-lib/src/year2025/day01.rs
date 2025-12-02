// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2025, 1)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 1 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let mut current = 50;
    let mut password_count = 0;

    for line in _input.lines() {
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        if direction == 'L' {
            current = (current - amount).rem_euclid(100)
        }

        if direction == 'R' {
            current = (current + amount) % 100
        }

        if current == 0 {
            password_count += 1
        }
    }

    Ok(password_count)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(
    _input: &str,
) -> Result<impl std::fmt::Display, std::fmt::Error> {
    let mut current = 50;
    let mut password_count: u64 = 0;

    for line in _input.lines() {
        if line.is_empty() {
            continue;
        }
        let direction = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        if direction == 'L' {
            let dist_to_zero = if current == 0 { 100 } else { current };

            if amount > dist_to_zero {
                password_count +=
                    1 + ((amount - dist_to_zero - 1) as u64 / 100);
            }
            current = (current - amount).rem_euclid(100);
        }

        if direction == 'R' {
            let turns = (current + amount - 1) / 100;
            if turns > 0 {
                password_count += turns as u64;
            }
            current = (current + amount) % 100;
        }

        if current == 0 {
            password_count += 1;
        }
    }

    Ok(password_count)
}
