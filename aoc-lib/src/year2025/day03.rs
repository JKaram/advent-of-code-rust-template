// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2025, 3)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 3 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let mut total = 0;

    for line in _input.lines() {
        let numbers: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let search_area = &numbers[..numbers.len() - 1];
        let max_val = search_area.iter().max().unwrap();

        let max_index = search_area.iter().position(|x| x == max_val).unwrap();

        let rest = &numbers[max_index + 1..];

        let next_highest = rest.iter().max().unwrap();

        let combine = (*max_val as u32 * 10) + *next_highest as u32;

        total += combine;
    }

    Ok(total)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let mut total_output_joltage: u64 = 0;
    const DIGITS_TO_FIND: usize = 12;

    for line in _input.lines() {
        if line.is_empty() {
            continue;
        }

        let numbers: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        if numbers.len() < DIGITS_TO_FIND {
            eprintln!("Skipping line, too short: {}", line);
            continue;
        }

        let mut current_start_index = 0;
        let mut current_bank_value: u64 = 0;

        for i in 0..DIGITS_TO_FIND {
            let digits_still_needed = DIGITS_TO_FIND - 1 - i;

            let end_bound = numbers.len() - digits_still_needed;

            let search_slice = &numbers[current_start_index..end_bound];

            let max_val =
                search_slice.iter().max().ok_or("Slice was empty").unwrap();

            let offset =
                search_slice.iter().position(|x| x == max_val).unwrap();

            // Append this digit to our running number
            current_bank_value = current_bank_value * 10 + (*max_val as u64);

            current_start_index += offset + 1;
        }

        total_output_joltage += current_bank_value;
    }

    Ok(total_output_joltage)
}
