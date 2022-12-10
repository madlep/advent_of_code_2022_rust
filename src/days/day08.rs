use std::cmp;
use std::str::FromStr;

pub fn part1(data: String) -> String {
    let mut trees: Trees = data.parse().unwrap();
    trees.build_trees_visibility();

    trees.iter().filter(|t| t.is_visible()).count().to_string()
}

pub fn part2(data: String) -> String {
    let mut trees: Trees = data.parse().unwrap();
    trees.build_trees_scenic_score();

    trees
        .iter()
        .map(|t| t.scenic_score)
        .max()
        .unwrap()
        .to_string()
}

struct Trees {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

struct InconsistentRowSizeError(String);

impl Trees {
    fn new() -> Self {
        Trees {
            trees: vec![],
            width: 0,
            height: 0,
        }
    }

    fn append_row(&mut self, mut row: Vec<Tree>) -> Result<(), InconsistentRowSizeError> {
        if self.width == 0 {
            // first row, set width and expect subsequent to be equal
            self.width = row.len();
        } else {
            if self.width != row.len() {
                return Err(InconsistentRowSizeError(format!(
                    "Expected row size: {} got: {}",
                    self.width,
                    row.len()
                )));
            }
        }
        self.height += 1;
        self.trees.append(&mut row);
        Ok(())
    }

    fn get(&self, coord: Coord) -> &Tree {
        let i = coord.y * self.width + coord.x;
        &self.trees[i]
    }

    fn get_mut(&mut self, coord: Coord) -> &mut Tree {
        let i = coord.y * self.width + coord.x;
        &mut self.trees[i]
    }

    fn iter(&self) -> impl Iterator<Item = &Tree> {
        self.trees.iter()
    }

    fn build_trees_visibility(&mut self) -> () {
        // calc vis by walking from dir setting req vis on each tree to be max of previous in that row

        // walk from north,
        {
            let mut north_vis = vec![-1; self.width];
            for y in 0..self.height {
                for x in 0..self.width {
                    let tree = &mut self.get_mut(Coord { x, y });
                    tree.vis.n = north_vis[x];
                    north_vis[x] = cmp::max(north_vis[x], tree.h);
                }
            }
        }

        // walk from east,
        {
            let mut east_vis = vec![-1; self.height];
            for x in (0..self.width).rev() {
                for y in 0..self.height {
                    let tree = &mut self.get_mut(Coord { x, y });
                    tree.vis.e = east_vis[y];
                    east_vis[y] = cmp::max(east_vis[y], tree.h);
                }
            }
        }

        // walk from west,
        {
            let mut west_vis = vec![-1; self.height];
            for x in 0..self.width {
                for y in 0..self.height {
                    let tree = &mut self.get_mut(Coord { x, y });
                    tree.vis.w = west_vis[y];
                    west_vis[y] = cmp::max(west_vis[y], tree.h);
                }
            }
        }

        // walk from south,
        {
            let mut south_vis = vec![-1; self.width];
            for y in (0..self.height).rev() {
                for x in 0..self.width {
                    let tree = &mut self.get_mut(Coord { x, y });
                    tree.vis.s = south_vis[x];
                    south_vis[x] = cmp::max(south_vis[x], tree.h);
                }
            }
        }
    }

    fn build_trees_scenic_score(&mut self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                let tree = self.get(Coord { x, y });

                let mut north_score = 0;
                for other_y in (0..y).rev() {
                    let other_tree = self.get(Coord {
                        x: tree.coord.x,
                        y: other_y,
                    });
                    north_score += 1;
                    if other_tree.h >= tree.h {
                        break;
                    }
                }

                let mut east_score = 0;
                for other_x in x + 1..self.width {
                    let other_tree = self.get(Coord {
                        x: other_x,
                        y: tree.coord.y,
                    });
                    east_score += 1;
                    if other_tree.h >= tree.h {
                        break;
                    }
                }

                let mut west_score = 0;
                for other_x in (0..x).rev() {
                    let other_tree = self.get(Coord {
                        x: other_x,
                        y: tree.coord.y,
                    });
                    west_score += 1;
                    if other_tree.h >= tree.h {
                        break;
                    }
                }

                let mut south_score = 0;
                for other_y in y + 1..self.height {
                    let other_tree = self.get(Coord {
                        x: tree.coord.x,
                        y: other_y,
                    });
                    south_score += 1;
                    if other_tree.h >= tree.h {
                        break;
                    }
                }

                self.get_mut(Coord { x, y }).scenic_score =
                    (north_score * east_score * west_score * south_score)
                        .try_into()
                        .unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct ParseTreesError(String);

impl FromStr for Trees {
    type Err = ParseTreesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = Trees::new();

        for (y, line) in s.lines().enumerate() {
            let mut tree_row: Vec<Tree> = vec![];
            for (x, c) in line.chars().enumerate() {
                let h = match c {
                    c @ '0'..='9' => c as i8 - '0' as i8,
                    e => {
                        return Err(ParseTreesError(
                            format!("bad height found: {e}").to_string(),
                        ))
                    }
                };
                let tree = Tree::new(h, Coord { x, y });
                tree_row.push(tree);
            }
            trees
                .append_row(tree_row)
                .or_else(|InconsistentRowSizeError(msg)| Err(ParseTreesError(msg)))?;
        }

        Ok(trees)
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
    scenic_score: u32,
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
            scenic_score: 0,
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
