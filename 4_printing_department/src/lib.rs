pub fn part_1(input: &str) -> u32 {
    let cols: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let mut slots = 0;
    for (y, row) in cols.iter().enumerate() {
        for (x, occupied) in row.iter().enumerate() {
            if !occupied {
                continue;
            }

            let abv = y.checked_sub(1);
            let blw = if y < cols.len() - 1 {
                Some(y + 1)
            } else {
                None
            };
            let lft = x.checked_sub(1);
            let rgt = if x < row.len() - 1 { Some(x + 1) } else { None };
            let xct = Some(x);
            let yct = Some(y);
            let neighbour_indexes = [
                (lft, abv), // TL
                (xct, abv), // TC
                (rgt, abv), // TR
                (lft, yct), // CL
                (rgt, yct), // CR
                (lft, blw), // BL
                (xct, blw), // BC
                (rgt, blw), // BR
            ];
            let occupied_neighbours = neighbour_indexes
                .into_iter()
                .filter_map(|(x, y)| {
                    let x = x?;
                    let y = y?;
                    Some(cols[y][x])
                })
                .filter(|b| *b)
                .count();
            if occupied_neighbours < 4 {
                slots += 1;
            }
        }
    }

    slots
}

pub fn part_2(input: &str) -> u32 {
    let mut cols: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let mut total_taken = 0;

    loop {
        let mut slots = Vec::new();
        for (y, row) in cols.iter().enumerate() {
            for (x, occupied) in row.iter().enumerate() {
                if !occupied {
                    continue;
                }

                let abv = y.checked_sub(1);
                let blw = if y < cols.len() - 1 {
                    Some(y + 1)
                } else {
                    None
                };
                let lft = x.checked_sub(1);
                let rgt = if x < row.len() - 1 { Some(x + 1) } else { None };
                let xct = Some(x);
                let yct = Some(y);
                let neighbour_indexes = [
                    (lft, abv), // TL
                    (xct, abv), // TC
                    (rgt, abv), // TR
                    (lft, yct), // CL
                    (rgt, yct), // CR
                    (lft, blw), // BL
                    (xct, blw), // BC
                    (rgt, blw), // BR
                ];
                let occupied_neighbours = neighbour_indexes
                    .into_iter()
                    .filter_map(|(x, y)| {
                        let x = x?;
                        let y = y?;
                        Some(cols[y][x])
                    })
                    .filter(|b| *b)
                    .count();
                if occupied_neighbours < 4 {
                    slots.push((x, y));
                }
            }
        }

        if slots.is_empty() {
            break;
        }

        total_taken += slots.len() as u32;
        for (x, y) in slots {
            cols[y][x] = false;
        }
    }

    total_taken
}

pub fn part_1_kye(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect();

    let (n, m) = (grid.len(), grid[0].len());

    let mut accessible = 0;
    for y in 0..n {
        for x in 0..m {
            if grid[y][x] != '@' {
                continue;
            }

            let neighbours = count_neighbours(&grid, x, y, n, m);
            if neighbours < 4 {
                accessible += 1
            }
        }
    }
    accessible
}

pub fn part_2_kye(input: &str) -> u64 {
    let mut grid: Vec<Vec<_>> = input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect();

    let (n, m) = (grid.len(), grid[0].len());

    let mut accessible = 0;
    loop {
        let mut cur = 0;
        for y in 0..n {
            for x in 0..m {
                if grid[y][x] != '@' {
                    continue;
                }

                let neighbours = count_neighbours(&grid, x, y, n, m);
                if neighbours < 4 {
                    grid[y][x] = '.';
                    cur += 1
                }
            }
        }
        if cur > 0 {
            accessible += cur;
        } else {
            break;
        }
    }
    accessible
}

fn count_neighbours(grid: &[Vec<char>], x: usize, y: usize, n: usize, m: usize) -> u32 {
    const DIRS: [(i32, i32); 8] = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut neighbours = 0;
    for (d_x, d_y) in DIRS {
        let new_x = x as i32 + d_x;
        let new_y = y as i32 + d_y;

        if new_x < 0 || new_x >= m as i32 || new_y < 0 || new_y >= n as i32 {
            continue;
        }

        if grid[new_y as usize][new_x as usize] == '@' {
            neighbours += 1;
        }
    }
    neighbours
}
