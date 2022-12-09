use std::cmp;

pub fn part1(data: String) -> String {
    let mut trees: Vec<Vec<Tree>> = vec![];

    for (y, line) in data.lines().enumerate() {
        let mut tree_row: Vec<Tree> = vec![];
        for (x, c) in line.chars().enumerate() {
            let h = match c {
                c @ '0'..='9' => c as i8 - '0' as i8,
                e => panic!("bad height found: {}", e),
            };
            let tree = Tree::new(h, Coord(x, y));
            tree_row.push(tree);
        }
        trees.push(tree_row);
    }
    let height = trees.len();
    let width = trees[0].len();

    // calc vis by walking from dir setting req vis on each tree to be max of previous in that row

    // walk from north,
    {
        let mut north_vis = vec![-1; width];
        for y in 0..height {
            for x in 0..width {
                let tree = &mut trees[y][x];
                tree.vis.n = north_vis[x];
                north_vis[x] = cmp::max(north_vis[x], tree.h);
            }
        }
    }

    // walk from east,
    {
        let mut east_vis = vec![-1; height];
        for x in (0..width).rev() {
            for y in 0..height {
                let tree = &mut trees[y][x];
                tree.vis.e = east_vis[y];
                east_vis[y] = cmp::max(east_vis[y], tree.h);
            }
        }
    }

    // walk from west,
    {
        let mut west_vis = vec![-1; height];
        for x in 0..width {
            for y in 0..height {
                let tree = &mut trees[y][x];
                tree.vis.w = west_vis[y];
                west_vis[y] = cmp::max(west_vis[y], tree.h);
            }
        }
    }

    // walk from south,
    {
        let mut south_vis = vec![-1; width];
        for y in (0..height).rev() {
            for x in 0..width {
                let tree = &mut trees[y][x];
                tree.vis.s = south_vis[x];
                south_vis[x] = cmp::max(south_vis[x], tree.h);
            }
        }
    }

    let mut visible_count = 0_u32;
    for tree_line in trees {
        for tree in tree_line {
            if tree.is_visible() {
                visible_count += 1;
            }
        }
    }
    visible_count.to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented");
}

#[derive(Debug)]
#[allow(dead_code)]
struct Tree {
    h: Height,
    coord: Coord,
    vis: Visibilities,
}

impl Tree {
    fn new(h: Height, coord: Coord) -> Self {
        Self {
            h,
            coord,
            vis: Visibilities {
                n: 0,
                e: 0,
                w: 0,
                s: 0,
            },
        }
    }

    fn is_visible(&self) -> bool {
        self.h > self.vis.n || self.h > self.vis.e || self.h > self.vis.w || self.h > self.vis.s
    }
}

type Height = i8;
type Vis = i8;

#[derive(Debug)]
struct Visibilities {
    n: Vis,
    e: Vis,
    w: Vis,
    s: Vis,
}
#[derive(Debug)]
struct Coord(usize, usize);
