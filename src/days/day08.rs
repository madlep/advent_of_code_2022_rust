use std::cmp;

pub fn part1(data: String) -> String {
    let mut trees = parse_trees(&data);
    build_trees_visibility(&mut trees);

    trees.iter().filter(|t| t.is_visible()).count().to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented");
}

fn parse_trees(data: &str) -> Trees {
    let mut trees = Trees::new();

    for (y, line) in data.lines().enumerate() {
        let mut tree_row: Vec<Tree> = vec![];
        for (x, c) in line.chars().enumerate() {
            let h = match c {
                c @ '0'..='9' => c as i8 - '0' as i8,
                e => panic!("bad height found: {}", e),
            };
            let tree = Tree::new(h, Coord { x, y });
            tree_row.push(tree);
        }
        trees.append_row(tree_row);
    }

    trees
}

fn build_trees_visibility(trees: &mut Trees) -> () {
    // calc vis by walking from dir setting req vis on each tree to be max of previous in that row

    // walk from north,
    {
        let mut north_vis = vec![-1; trees.width];
        for y in 0..trees.height {
            for x in 0..trees.width {
                let tree = &mut trees.get_mut(Coord { x, y });
                tree.vis.n = north_vis[x];
                north_vis[x] = cmp::max(north_vis[x], tree.h);
            }
        }
    }

    // walk from east,
    {
        let mut east_vis = vec![-1; trees.height];
        for x in (0..trees.width).rev() {
            for y in 0..trees.height {
                let tree = &mut trees.get_mut(Coord { x, y });
                tree.vis.e = east_vis[y];
                east_vis[y] = cmp::max(east_vis[y], tree.h);
            }
        }
    }

    // walk from west,
    {
        let mut west_vis = vec![-1; trees.height];
        for x in 0..trees.width {
            for y in 0..trees.height {
                let tree = &mut trees.get_mut(Coord { x, y });
                tree.vis.w = west_vis[y];
                west_vis[y] = cmp::max(west_vis[y], tree.h);
            }
        }
    }

    // walk from south,
    {
        let mut south_vis = vec![-1; trees.width];
        for y in (0..trees.height).rev() {
            for x in 0..trees.width {
                let tree = &mut trees.get_mut(Coord { x, y });
                tree.vis.s = south_vis[x];
                south_vis[x] = cmp::max(south_vis[x], tree.h);
            }
        }
    }
}

struct Trees {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl Trees {
    fn new() -> Self {
        Trees {
            trees: vec![],
            width: 0,
            height: 0,
        }
    }

    fn append_row(&mut self, mut row: Vec<Tree>) -> () {
        if self.width == 0 {
            // first row, set width and expect subsequent to be equal
            self.width = row.len();
        } else {
            if self.width != row.len() {
                panic!("Expected row size: {} got: {}", self.width, row.len());
            }
        }
        self.height += 1;
        self.trees.append(&mut row);
    }

    fn get_mut(&mut self, coord: Coord) -> &mut Tree {
        let i = coord.y * self.width + coord.x;
        &mut self.trees[i]
    }

    fn iter(&self) -> impl Iterator<Item = &Tree> {
        self.trees.iter()
    }
}

impl IntoIterator for Trees {
    type Item = Tree;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.trees.into_iter()
    }
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
struct Coord {
    x: usize,
    y: usize,
}
