extern crate rand;

use self::rand::seq::SliceRandom;
use self::rand::Rng;
use crate::board::{Board, Pos};
use crate::error::Result;
use crate::land;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Connection<'a> {
    from: &'a Pos,
    to: &'a Pos,
}

pub struct LargeBoardGen<'a, R: Rng + 'a> {
    rng: &'a mut R,
    width: usize,
    height: usize,
    altitudes: Vec<Vec<u8>>,
    max_towns: usize,
    num_tops: usize,
    town_min_cost: usize,
    conn_max_cost: usize,
    down_rate: u8,
}

impl<'a, R: Rng> LargeBoardGen<'a, R> {
    pub fn new<'b: 'a>(rng: &'b mut R, width: usize, height: usize) -> Self {
        let mut altitudes = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(0);
            }
            altitudes.push(row);
        }

        let max_towns = rng.gen_range(10, 16);
        let num_tops = width * height / 2048 + rng.gen_range(0, 4);
        let town_min_cost = width / max_towns;
        let conn_max_cost = width / 2;
        let down_rate = 4; // Set smaller down rate for larger map

        LargeBoardGen {
            rng,
            height,
            altitudes,
            width,
            max_towns,
            num_tops,
            town_min_cost,
            conn_max_cost,
            down_rate,
        }
    }

    #[inline]
    fn land_kind(altitude: u8) -> land::LandKind {
        match altitude {
            0...40 => land::LandKind::DeepSea,
            41...55 => land::LandKind::Sea,
            56...70 => land::LandKind::Ground,
            71...80 => land::LandKind::Forest,
            81...90 => land::LandKind::Mountain,
            91...99 => land::LandKind::Alpine,
            _ => unreachable!(),
        }
    }

    // Down a slope
    fn down(&mut self, altitude: u8, x: usize, y: usize) {
        let delta = self.rng.gen_range(0, self.down_rate);
        if altitude < delta {
            // Skip when altitude is min since default value of altitude is 0
            return;
        }
        let altitude = altitude - delta;
        if self.altitudes[y][x] >= altitude {
            // Skip when the altitude is already calculated as other mountain's slope
            return;
        }
        self.slope(altitude, x, y);
    }

    // Create a slope of mountain
    fn slope(&mut self, altitude: u8, x: usize, y: usize) {
        self.altitudes[y][x] = altitude;
        if x > 0 {
            self.down(altitude, x - 1, y);
        }
        if self.width - 1 > x {
            self.down(altitude, x + 1, y);
        }
        if y > 0 {
            self.down(altitude, x, y - 1);
        }
        if self.height - 1 > y {
            self.down(altitude, x, y + 1);
        }
    }

    fn tops(&mut self) -> HashSet<Pos> {
        let mut tops = HashSet::with_capacity(self.num_tops);
        while tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            tops.insert(Pos { x, y });
        }
        tops
    }

    fn towns(&mut self) -> HashSet<Pos> {
        fn land_fitness(kind: land::LandKind) -> u8 {
            match kind {
                land::LandKind::DeepSea => 0,
                land::LandKind::Sea => 16,
                land::LandKind::Ground => 8,
                land::LandKind::Forest => 4,
                land::LandKind::Mountain => 2,
                land::LandKind::Alpine => 1,
                _ => unreachable!(),
            }
        }

        // Initialize fitness
        let mut fitness = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                row.push(land_fitness(Self::land_kind(self.altitudes[y][x])))
            }
            fitness.push(row);
        }

        // Cells at edges of map, (0, y), (x, 0), (MAX, y), (x, MAX), never become towns

        fn convo_3times(fitness: &mut Vec<Vec<u8>>) {
            for _ in 0..3 {
                for y in 1..fitness.len() - 1 {
                    for x in 1..fitness[y].len() - 1 {
                        let mut sum = 0 as i32;
                        for y in &[y - 1, y, y + 1] {
                            for x in &[x - 1, x, x + 1] {
                                sum = sum + fitness[*y][*x] as i32;
                            }
                        }
                        fitness[y][x] = (sum / 9) as u8;
                    }
                }
            }
        }

        convo_3times(&mut fitness);
        for h in 0..self.height {
            for w in 0..self.width {
                if h == 0
                    || w == 0
                    || h == self.height - 1
                    || w == self.width - 1
                    || Self::land_kind(self.altitudes[h][w]) != land::LandKind::Ground
                {
                    fitness[h][w] = 0;
                }
            }
        }
        convo_3times(&mut fitness);

        let mut min_fitness = 0;
        for row in fitness.iter() {
            for f in row.iter() {
                if *f > min_fitness {
                    min_fitness = *f;
                }
            }
        }
        let min_fitness = min_fitness * 9 / 10; // * 0.9

        let mut candidates = Vec::new();
        for y in 1..fitness.len() - 1 {
            for x in 1..fitness[y].len() - 1 {
                if fitness[y][x] >= min_fitness {
                    candidates.push(Pos { x, y });
                }
            }
        }
        candidates.as_mut_slice().shuffle(&mut self.rng);

        let mut towns = HashSet::with_capacity(self.max_towns);
        for c in candidates.iter() {
            if towns.len() == self.max_towns {
                break;
            }
            if towns
                .iter()
                .all(|p: &Pos| p.move_cost(c) > self.town_min_cost)
            {
                towns.insert(*c);
            }
        }
        towns
    }

    // Get shortest path of the connection using Dijkstra's algorithm
    fn shortest_path<'b>(&self, conn: &Connection<'b>) -> Vec<Pos> {
        #[inline]
        fn land_cost(kind: land::LandKind) -> usize {
            match kind {
                land::LandKind::DeepSea => 64,
                land::LandKind::Sea => 32,
                land::LandKind::Ground => 1,
                land::LandKind::Forest => 2,
                land::LandKind::Mountain => 8,
                land::LandKind::Alpine => 16,
                _ => unreachable!(),
            }
        }

        #[derive(Clone)]
        enum Route {
            Cons(Pos, Rc<Route>),
            Nil,
        }

        struct Vert {
            cost: usize,
            pos: Pos,
            prev: Route,
        }

        // Note: Vert is ordered by cost for priority queue

        impl PartialEq for Vert {
            fn eq(&self, rhs: &Vert) -> bool {
                self.cost == rhs.cost
            }
        }
        impl Eq for Vert {}

        impl Ord for Vert {
            fn cmp(&self, rhs: &Vert) -> Ordering {
                rhs.cost.cmp(&self.cost)
            }
        }

        impl PartialOrd for Vert {
            fn partial_cmp(&self, rhs: &Vert) -> Option<Ordering> {
                Some(self.cmp(rhs))
            }
        }

        // Map node => cost
        let mut costs = HashMap::new();
        costs.insert(*conn.from, 0);

        let mut state = BinaryHeap::new();
        state.push(Vert {
            cost: 0,
            pos: *conn.from,
            prev: Route::Nil,
        });

        while let Some(Vert { cost, pos, prev }) = state.pop() {
            if &pos == conn.to {
                // Collect list as Vec<Pos>
                // Note: Start node and goal node are not included since they are town
                let mut verts = Vec::new();
                let mut route = &prev;
                while let Route::Cons(pos, ref prev) = route {
                    verts.push(*pos);
                    route = prev;
                }
                return verts;
            }

            let Pos { x, y } = pos;
            for pair in &[
                // (x.checked_sub(1), y.checked_sub(1)),
                (Some(x), y.checked_sub(1)),
                // (x.checked_add(1), y.checked_sub(1)),
                (x.checked_sub(1), Some(y)),
                (x.checked_add(1), Some(y)),
                // (x.checked_sub(1), y.checked_add(1)),
                (Some(x), y.checked_add(1)),
                // (x.checked_add(1), y.checked_add(1)),
            ] {
                let (x, y) = match pair {
                    (Some(x), Some(y)) => {
                        if *x >= self.width || *y >= self.height {
                            continue;
                        }
                        (*x, *y)
                    }
                    _ => continue,
                };

                if let Route::Cons(pos, ..) = prev {
                    if pos.x == x && pos.y == y {
                        // Going back to previous position never happens
                        continue;
                    }
                }

                let cost = cost + land_cost(Self::land_kind(self.altitudes[y][x]));
                let pos = Pos { x, y };

                if let Some(c) = costs.get(&pos) {
                    if cost >= *c {
                        continue;
                    }
                }

                costs.insert(pos, cost);
                state.push(Vert {
                    cost,
                    pos,
                    prev: Route::Cons(pos, Rc::new(prev.clone())),
                });
            }
        }

        // Connection unreachable
        Vec::new()
    }

    // Get all cells of paths
    fn paths(&mut self, towns: &HashSet<Pos>) -> HashSet<Pos> {
        towns
            .iter()
            .map(|town| {
                let mut near_towns = towns
                    .iter()
                    .filter_map(|t| {
                        if t == town {
                            return None;
                        }
                        let cost = t.move_cost(town);
                        if cost > self.conn_max_cost {
                            return None;
                        }
                        Some((cost, t))
                    })
                    .collect::<Vec<_>>();
                near_towns.sort_unstable_by_key(|(cost, _)| cost.clone());

                let mut dirs = HashSet::new();
                near_towns.into_iter().filter_map(move |(_, near)| {
                    let angle =
                        (town.y as f64 - near.y as f64).atan2(town.x as f64 - near.x as f64);
                    let dir = (angle / 45.0) as usize;
                    for dir in &[dir, (dir + 1) % 8, (dir + 7) % 8] {
                        if dirs.contains(dir) {
                            return None;
                        }
                    }

                    dirs.insert(dir);
                    Some(Connection {
                        from: &town,
                        to: &near,
                    })
                })
            })
            .flatten()
            .filter({
                // Dedup connections (from-to pairs)
                let mut saw = HashMap::new();
                move |conn: &Connection| {
                    if let Some(to) = saw.get(conn.from) {
                        if to == &conn.to {
                            return false;
                        }
                    }
                    if let Some(from) = saw.get(conn.to) {
                        if from == &conn.from {
                            return false;
                        }
                    }
                    saw.insert(conn.from, conn.to);
                    true
                }
            })
            .map(|conn| self.shortest_path(&conn))
            .flatten()
            .collect()
    }

    pub fn gen(&mut self) -> Result<Board<'static>> {
        let tops = self.tops();

        // Calculate altitude of cells
        for Pos { x, y } in tops.iter() {
            // Altitude is 0~99. Top is always at 99
            self.slope(99, *x, *y);
        }

        let towns = self.towns();
        let paths = self.paths(&towns);

        Ok(Board::build(self.width, self.height, |w, h| {
            let alt = self.altitudes[h][w];
            let p = Pos { x: w, y: h };
            let mut land = if tops.contains(&p) {
                land::TOP.clone()
            } else if towns.contains(&p) {
                land::TOWN.clone()
            } else if paths.contains(&p) {
                land::PATH.clone()
            } else {
                Self::land_kind(alt).constant()
            };
            land.altitude = alt;
            land
        }))
    }
}
