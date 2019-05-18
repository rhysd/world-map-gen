use rand;

use self::rand::seq::SliceRandom;
use self::rand::Rng;
use crate::board::{Board, Pos};
use crate::land::LandKind;
use crate::slope::SlopeGen;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Connection<'a> {
    from: &'a Pos,
    to: &'a Pos,
}

pub struct LargeBoardGen<'a, R: Rng> {
    rng: &'a mut R,
    width: usize,
    height: usize,
    max_towns: usize,
    num_tops: usize,
    town_min_cost: usize,
    conn_max_cost: usize,
    down_rate: u8,
}

// This warning is raised only on wasm32 target since Pos contains usize fields.
// usize field have 64bits length on x86_64 but have 32bits on wasm32.
// So using `self` argument is efficient on wasm32, but not on x86_64.
#[allow(clippy::trivially_copy_pass_by_ref)]
impl<'a, R: Rng> LargeBoardGen<'a, R> {
    pub fn new<'b: 'a>(rng: &'b mut R, width: usize, height: usize) -> Self {
        let max_towns = rng.gen_range(10, 16);
        let num_tops = width * height / 2048 + rng.gen_range(0, 4);
        let average_len = (width + height) / 2;
        let town_min_cost = average_len.checked_div(max_towns).unwrap_or(average_len);
        let conn_max_cost = average_len / 2;
        let down_rate = 6; // Set smaller down rate for larger map

        LargeBoardGen {
            rng,
            height,
            width,
            max_towns,
            num_tops,
            town_min_cost,
            conn_max_cost,
            down_rate,
        }
    }

    #[inline]
    fn land_kind(altitude: u8) -> LandKind {
        match altitude {
            0..=40 => LandKind::DeepSea,
            41..=55 => LandKind::Sea,
            56..=70 => LandKind::Plain,
            71..=80 => LandKind::Forest,
            81..=90 => LandKind::Mountain,
            91..=99 => LandKind::Highland,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn towns(&mut self, altitudes: &[Vec<u8>]) -> HashSet<Pos> {
        #[inline]
        fn land_fitness(kind: LandKind) -> u8 {
            match kind {
                LandKind::DeepSea => 0,
                LandKind::Sea => 16,
                LandKind::Plain => 8,
                LandKind::Forest => 4,
                LandKind::Mountain => 2,
                LandKind::Highland => 1,
                _ => unreachable!(),
            }
        }

        // Initialize fitness
        let mut fitness = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                row.push(land_fitness(Self::land_kind(altitudes[y][x])))
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
                                sum += i32::from(fitness[*y][*x]);
                            }
                        }
                        fitness[y][x] = (sum / 9) as u8;
                    }
                }
            }
        }

        convo_3times(&mut fitness);
        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0
                    || x == 0
                    || y == self.height - 1
                    || x == self.width - 1
                    || Self::land_kind(altitudes[y][x]) != LandKind::Plain
                {
                    fitness[y][x] = 0;
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
    fn shortest_path<'b>(&self, conn: &Connection<'b>, altitudes: &[Vec<u8>]) -> Vec<Pos> {
        #[inline]
        fn land_cost(kind: LandKind) -> usize {
            match kind {
                LandKind::DeepSea => 64,
                LandKind::Sea => 32,
                LandKind::Plain => 1,
                LandKind::Forest => 4,
                LandKind::Mountain => 8,
                LandKind::Highland => 16,
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

            // Note: OK to create an Rc pointer in advance since at least one point iterated in below loop is valid.
            let prev = Rc::new(prev);
            let Pos { x, y } = pos;
            for pair in &[
                (Some(x), y.checked_sub(1)),
                (x.checked_sub(1), Some(y)),
                (x.checked_add(1), Some(y)),
                (Some(x), y.checked_add(1)),
            ] {
                let (x, y) = match pair {
                    (Some(x), Some(y)) if *x < self.width && *y < self.height => (*x, *y),
                    _ => continue,
                };

                if let Route::Cons(pos, ..) = *prev {
                    if pos.x == x && pos.y == y {
                        // Going back to previous position never happens
                        continue;
                    }
                }

                let cost = cost + land_cost(Self::land_kind(altitudes[y][x]));
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
                    prev: Route::Cons(pos, prev.clone()),
                });
            }
        }

        // Connection unreachable
        Vec::new()
    }

    // Get all cells of paths
    fn paths(&mut self, towns: &HashSet<Pos>, altitudes: &[Vec<u8>]) -> HashSet<Pos> {
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
                near_towns.sort_unstable_by_key(|(cost, _)| *cost);

                let mut dirs = HashSet::new();
                near_towns.into_iter().filter_map(move |(_, near)| {
                    // On wasm32, usize has 32bits length so converting from usize to f64 is lossless.
                    // For conversion between usize and f64 using `as`, clippy complains that using
                    // f64::from instead is safer. This is true on wasm32, but on x86_64, conversion
                    // between usize and f64 is not lossless so f64::from(usize) is not implemented.
                    #[allow(clippy::cast_lossless)]
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
                move |conn: &Connection<'_>| {
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
            .map(|conn| self.shortest_path(&conn, altitudes))
            .flatten()
            .collect()
    }

    pub fn gen(&mut self) -> Board<'static> {
        let mut slope = SlopeGen::new(
            self.rng,
            self.width,
            self.height,
            self.down_rate,
            self.num_tops,
        );
        slope.gen();
        let altitudes = slope.altitudes;
        let tops = slope.tops;

        let towns = self.towns(&altitudes);
        let paths = self.paths(&towns, &altitudes);

        Board::build(self.width, self.height, |w, h| {
            let alt = altitudes[h][w];
            let p = Pos { x: w, y: h };
            if tops.contains(&p) {
                LandKind::Top.preset(alt)
            } else if towns.contains(&p) {
                LandKind::Town.preset(alt)
            } else if paths.contains(&p) {
                LandKind::Path.preset(alt)
            } else {
                Self::land_kind(alt).preset(alt)
            }
        })
    }
}
