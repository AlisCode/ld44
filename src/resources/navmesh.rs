use crate::components::{TilePos, Walkable};
use std::collections::HashMap;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct NavCoords {
    pub x: i32,
    pub y: i32,
}

impl NavCoords {
    pub fn distance(&self, other: &Self) -> i32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx.abs() + dy.abs()
    }
}

impl From<(i32, i32)> for NavCoords {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Default)]
pub struct NavMesh {
    map: HashMap<u64, NavCoords>,
    next: u64,
}

impl NavMesh {
    pub fn setup(&mut self, walk: &mut Walkable, pos: &TilePos) {
        walk.id_nav_mesh = Some(self.next);
        self.map.insert(self.next, (pos.x, pos.y).into());
        self.next += 1;
    }

    pub fn update(&mut self, id: u64, pos: &TilePos) {
        let mut c = self.map.get_mut(&id).unwrap();
        c.x = pos.x;
        c.y = pos.y;
    }

    fn get_neighbors(&self, nav_coords: &NavCoords) -> Vec<(NavCoords, i32)> {
        self.map
            .values()
            .cloned()
            .filter_map(|p| {
                let dx = (nav_coords.x - p.x).abs();
                let dy = (nav_coords.y - p.y).abs();
                if dx + dy == 1 {
                    return Some((p, 1));
                }
                None
            })
            .collect()
    }

    pub fn next_hop<NC: Into<NavCoords>>(&self, start: NC, end: NC) -> Option<NavCoords> {
        let start: NavCoords = start.into();
        let end: NavCoords = end.into();

        let path: Option<(Vec<NavCoords>, i32)> = pathfinding::directed::astar::astar(
            &start,
            |n| self.get_neighbors(n).into_iter(),
            |n| n.distance(&end),
            |n| n == &end,
        );

        path.map(|p| p.0[1])
    }
}
