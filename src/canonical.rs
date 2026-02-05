use crate::{Coord, Grid};

pub fn canonical_entry_point(cells: &Grid) {
    // Converts to vec for easier handling
    let cells: Vec<_> = cells.iter().collect();

    // FIRST STEP: FINDING BOUNDING BOX
    // VERY important, needs to be able to find the minimum rectangle that encompasses the alive cells.
    // This has to be in local space, so subtracting all values by min x and y.
    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;

    for (x, y) in cells.iter() {
        let x = *x;
        let y = *y;
        if x < min_x {
            min_x = x
        }
        if y < min_y {
            min_y = y
        }
    }

    let mut normalized: Vec<(u32, u32)> = Vec::new();

    for (x, y) in cells.iter() {
        let new_x = (x - min_x) as u32;
        let new_y = (y - min_y) as u32;
        normalized.push((new_x, new_y));
    }

    println!("{:?}", normalized);
}

// -10 + 10 = 0
// 0 + 0 = 0
// 10 - 10 = 0
