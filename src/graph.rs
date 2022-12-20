use num::Bounded;
use priority_queue::DoublePriorityQueue;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
pub struct Graph<T, W> {
    vertices: HashSet<T>,
    edges: HashMap<T, HashMap<T, W>>,
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Copy + Debug,
    W: Bounded + Add<Output = W> + PartialOrd + Ord + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn push_vertex(&mut self, vertex: T) -> () {
        self.vertices.insert(vertex);
    }

    pub fn push_edge(&mut self, from: T, to: T, weight: W) -> () {
        match self.edges.get_mut(&from) {
            Some(edge_ends) => {
                edge_ends.insert(to, weight);
            }
            None => {
                let mut edge_ends = HashMap::new();
                edge_ends.insert(to, weight);
                self.edges.insert(from, edge_ends);
            }
        }
    }

    pub fn shortest_paths_from(&self, from: &T) -> HashMap<T, W> {
        let mut dist: HashMap<T, W> = HashMap::new();
        let mut queue: DoublePriorityQueue<T, W> = DoublePriorityQueue::new();

        for v in self.vertices.iter() {
            let v_dist: W = if v == from {
                Bounded::min_value()
            } else {
                Bounded::max_value()
            };
            dist.insert(*v, v_dist.clone());
            queue.push(*v, v_dist.clone());
        }

        while let Some((u, _priority)) = queue.pop_min() {
            if let Some(edges) = self.edges.get(&u) {
                let dist_u = dist.get(&u).unwrap().clone();
                for (v, weight) in edges.iter() {
                    let dist_v = dist.get(&v).unwrap();
                    let alt = dist_u.clone() + weight.clone();
                    if alt < *dist_v {
                        dist.insert(v.clone(), alt);
                        queue.change_priority(v, alt);
                    }
                }
            }
        }

        dist
    }
}
