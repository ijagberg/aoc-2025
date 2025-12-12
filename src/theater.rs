use simple_grid::{Grid, GridIndex};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::read_dir,
};

fn join_red_tiles(grid: &mut Grid<Tile>, from: GridIndex, to: GridIndex) {
    if to.column() == from.column() {
        // move vertically
        let column = to.column();
        if to.row() > from.row() {
            // move down
            for row in from.row() + 1..to.row() {
                grid[(column, row)] = Tile::Green;
            }
        } else if to.row() < from.row() {
            // move up
            for row in to.row() + 1..from.row() {
                grid[(column, row)] = Tile::Green;
            }
        } else {
            panic!("corners {} and {} are equal!", from, to);
        }
    } else if to.row() == from.row() {
        // move horizontally
        let row = to.row();
        if to.column() > from.column() {
            // move right
            for column in from.column() + 1..to.column() {
                grid[(column, row)] = Tile::Green;
            }
        } else if to.column() < from.column() {
            // move left
            for column in to.column() + 1..from.column() {
                grid[(column, row)] = Tile::Green;
            }
        } else {
            panic!("corners {} and {} are equal!", from, to);
        }
    } else {
        panic!("corners {} and {} are not positioned correctly!", from, to);
    }
}

pub fn largest_red_green_rectangle(red_tiles: &[GridIndex]) -> usize {
    let (compressed_x, compressed_y) = compress_coords(red_tiles);
    let right = compressed_x.values().max().unwrap();
    let bottom = compressed_y.values().max().unwrap();

    let width = right + 5;
    let height = bottom + 5;
    let mut grid = Grid::new(width, height, vec![Tile::Other; width * height]);

    fill_grid(&mut grid, red_tiles, (&compressed_x, &compressed_y));

    let mut areas = Vec::with_capacity(red_tiles.len().pow(2));
    for i in 0..red_tiles.len() {
        let corner_a = red_tiles[i];
        for &corner_b in red_tiles.iter().skip(1) {
            let area = area(corner_a, corner_b);
            areas.push((corner_a, corner_b, area));
        }
    }
    areas.sort_by_key(|(_, _, area)| *area);

    for (corner_a, corner_b, area) in areas.iter().rev() {
        if is_red_green(&grid, *corner_a, *corner_b, (&compressed_x, &compressed_y)) {
            return *area;
        }
    }

    unreachable!()
}

fn fill_grid(
    grid: &mut Grid<Tile>,
    red_tiles: &[GridIndex],
    (compressed_x, compressed_y): (&HashMap<usize, usize>, &HashMap<usize, usize>),
) {
    // join all the red tiles
    let mut from = red_tiles[0];
    for &to in red_tiles.iter().skip(1) {
        let comp_from = GridIndex::new(compressed_x[&from.column()], compressed_y[&from.row()]);
        let comp_to = GridIndex::new(compressed_x[&to.column()], compressed_y[&to.row()]);
        grid[comp_from] = Tile::Red;
        join_red_tiles(grid, comp_from, comp_to);
        from = to;
    }
    let to = red_tiles[0];
    let comp_from = GridIndex::new(compressed_x[&from.column()], compressed_y[&from.row()]);
    let comp_to = GridIndex::new(compressed_x[&to.column()], compressed_y[&to.row()]);
    grid[comp_from] = Tile::Red;
    join_red_tiles(grid, comp_from, comp_to);

    println!("joined: ");
    println!("{}", grid.to_pretty_string());

    // flood "fill" the outside
    let mut queue = VecDeque::new();
    queue.push_back(GridIndex::new(0, 0));
    let mut visited = HashSet::new();
    while let Some(i) = queue.pop_front() {
        if visited.contains(&i) || grid[i] == Tile::Red || grid[i] == Tile::Green {
            continue;
        }
        visited.insert(i);
        grid[i] = Tile::Void;
        for n in grid.neighbor_indices_of(i) {
            queue.push_back(n);
        }
    }

    println!("empty outside:");
    println!("{}", grid.to_pretty_string());
}

fn is_red_green(
    grid: &Grid<Tile>,
    corner_a: GridIndex,
    corner_b: GridIndex,
    (compressed_x, compressed_y): (&HashMap<usize, usize>, &HashMap<usize, usize>),
) -> bool {
    let corner_a = GridIndex::new(
        compressed_x[&corner_a.column()],
        compressed_y[&corner_a.row()],
    );
    let corner_b = GridIndex::new(
        compressed_x[&corner_b.column()],
        compressed_y[&corner_b.row()],
    );

    // go around the edge and check for any reds that are not on the corners
    let (left, right) = if corner_a.column() < corner_b.column() {
        (corner_a.column(), corner_b.column())
    } else {
        (corner_b.column(), corner_a.column())
    };
    let (top, bottom) = if corner_a.row() < corner_b.row() {
        (corner_a.row(), corner_b.row())
    } else {
        (corner_b.row(), corner_a.row())
    };

    for col in left..=right {
        for row in top..=bottom {
            if grid[(col, row)] == Tile::Void {
                return false;
            }
        }
    }
    //
    // for col in left + 1..right {
    //     // top row
    //     if grid[(col, top)] == Tile::Red {
    //         return false;
    //     }
    //     // bottom row
    //     if grid[(col, bottom)] == Tile::Red {
    //         return false;
    //     }
    // }
    // for row in top + 1..bottom {
    //     // left column
    //     if grid[(left, row)] == Tile::Red {
    //         return false;
    //     }
    //     // right column
    //     if grid[(right, row)] == Tile::Red {
    //         return false;
    //     }
    // }

    true
}

fn compress_coords(red_tiles: &[GridIndex]) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut x: Vec<_> = red_tiles.iter().map(|i| i.column()).collect();
    x.sort();
    x.dedup();
    let mut y: Vec<_> = red_tiles.iter().map(|i| i.row()).collect();
    y.sort();
    y.dedup();
    let mut compressed_x = 1;
    let mut compressed_x_map = HashMap::new();
    let mut compressed_y = 1;
    let mut compressed_y_map = HashMap::new();
    for x_coord in x {
        compressed_x_map.insert(x_coord, compressed_x);
        compressed_x += 2;
    }
    for y_coord in y {
        compressed_y_map.insert(y_coord, compressed_y);
        compressed_y += 2;
    }

    (compressed_x_map, compressed_y_map)
}

pub fn largest_rectangle(red_tiles: &[GridIndex]) -> usize {
    let mut largest_rectangle = 0;
    for a in 0..red_tiles.len() {
        let corner_a = red_tiles[a];
        for &corner_b in red_tiles.iter().skip(1) {
            let area = area(corner_a, corner_b);
            if area > largest_rectangle {
                largest_rectangle = area;
            }
        }
    }

    largest_rectangle
}

fn area(corner_a: GridIndex, corner_b: GridIndex) -> usize {
    if corner_a == corner_b {
        1
    } else {
        (corner_a.column().abs_diff(corner_b.column()) + 1)
            * (corner_a.row().abs_diff(corner_b.row()) + 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Red,
    Green,
    Other,
    Void,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Red => '#',
            Tile::Green => 'X',
            Tile::Other => '.',
            Tile::Void => ' ',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
