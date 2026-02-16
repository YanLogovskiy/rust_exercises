use std::cmp::{min};

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut cloned: Vec<Vec<u8>> = garden.iter().map(|row| row.as_bytes().to_vec()).collect();
    for (i, row) in garden.iter().enumerate() {
        for (j, cell) in row.as_bytes().iter().enumerate() {
            if *cell == b'*' {
                // Directions: adjacent cells
                for ni in i.saturating_sub(1)..=min(cloned.len()-1, i+1) as usize {
                    for nj in j.saturating_sub(1)..=min(cloned[ni].len()-1, j+1) as usize {
                        if cloned[ni][nj] == b'*' {
                            continue;
                        }
                        if !cloned[ni][nj].is_ascii_digit() {
                            cloned[ni][nj] = b'1';
                            println!("cloned[{}][{}] = {}", ni, nj, cloned[ni][nj] as char);
                        } else {
                            cloned[ni][nj] += 1;
                            println!("cloned[{}][{}] = {}", ni, nj, cloned[ni][nj] as char);
                        }
                    }
                }
            }
        }
    }
    cloned.iter().map(|row| row.iter().map(|&b| b as char).collect::<String>()).collect()
}
