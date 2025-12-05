// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use crate::utils;
use anyhow::Result;

// Example template.

pub fn solve() -> Result<()> {
    // Load your input file.
    let input = utils::load_input(2025, 4)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 4 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let grid: Vec<&[u8]> = _input.lines().map(|l| l.as_bytes()).collect();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut accessible_count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] != b'@' {
                continue;
            }

            let mut neighbors = 0;
            for (dr, dc) in directions {
                let nr = r + dr;
                let nc = c + dc;

                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    if grid[nr as usize][nc as usize] == b'@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible_count += 1;
            }
        }
    }

    Ok(accessible_count)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let mut grid: Vec<Vec<u8>> =
        _input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut total_removed = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut to_remove = Vec::new();

        // 1. SCAN
        for r in 0..rows {
            for c in 0..cols {
                if grid[r as usize][c as usize] != b'@' {
                    continue;
                }

                let mut neighbors = 0;
                for (dr, dc) in directions {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                        if grid[nr as usize][nc as usize] == b'@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        // 2. CHECK STOP CONDITION
        if to_remove.is_empty() {
            break;
        }

        // 3. UPDATE
        total_removed += to_remove.len();
        for (r, c) in to_remove {
            grid[r as usize][c as usize] = b'.';
        }
    }

    Ok(total_removed)
}
