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
