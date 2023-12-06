use std::cmp::min;
use std::ops::Range;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use robotics_lib::world::tile::Content::Garbage;
use robotics_lib::world::tile::{Content, Tile};

#[derive(Clone)]
pub(crate) struct GarbageSettings {
    pub(crate) total_garbage_quantity: usize,
    pub(crate) garbage_pile_size: Range<usize>,
    pub(crate) garbage_per_tile_quantity: Range<usize>,
    pub(crate) spawn_in_near_tiles_probability: f64,
    pub(crate) probability_step_by: f64,
}

impl GarbageSettings {
    /// Initialize the struct with optimal parameters given the world size
    pub(crate) fn default(size: usize) -> Self {
        GarbageSettings {
            total_garbage_quantity: size,
            garbage_pile_size: 1..size / 20,
            garbage_per_tile_quantity: 1..size / 100,
            spawn_in_near_tiles_probability: 1.0,
            probability_step_by: 0.2,
        }
    }
}

pub(crate) fn spawn_garbage(world: &mut Vec<Vec<Tile>>, settings: &GarbageSettings) {
    let mut i = 0;
    let mut rng = thread_rng();
    let max_amount = min(
        settings.garbage_per_tile_quantity.clone().max().unwrap(),
        Garbage(0).properties().max(),
    );

    while i < settings.total_garbage_quantity {
        spawn_garbage_build_up(world, settings, &mut i, &mut rng, max_amount);
    }
}

#[inline(always)]
pub(crate) fn spawn_garbage_build_up(
    world: &mut Vec<Vec<Tile>>,
    settings: &GarbageSettings,
    placed: &mut usize,
    rng: &mut ThreadRng,
    max_garbage_per_tile: usize,
) {
    // Get size of garbage pile
    let pile_range = rng.gen_range(settings.garbage_pile_size.clone());

    // Note that the matrix size will be rounded to greater odd number
    let probability_matrix = generate_prob_matrix(pile_range, settings.probability_step_by);

    // get random x and y coordinates, the base indexes where matrix garbage will starts
    let map_range = 0..world.len();

    let base_y = rng.gen_range(map_range.clone());
    let base_x = rng.gen_range(map_range.clone());

    //(x,y) will be the (0,0) of the probability matrix (not the center cause im lazy)
    for (row_index, row) in probability_matrix.iter().enumerate() {
        for col_index in 0..row.len() {
            // get the random value for the spawn
            let value: f64 = thread_rng().gen_range(0.1..=settings.spawn_in_near_tiles_probability);

            // assign if the probability is satisfied
            if value > (1. - probability_matrix[row_index][col_index]) {
                // get random amount of garbage fot the tile content
                let random_amount = rng.gen_range(1..max_garbage_per_tile);
                if set_content(
                    world,
                    base_y + col_index,
                    base_x + row_index,
                    random_amount,
                    probability_matrix.len(),
                ) {
                    *placed += random_amount;
                }
            }
        }
    }
}

#[inline(always)]
fn set_content(world: &mut [Vec<Tile>], y: usize, x: usize, amount: usize, mat_size: usize) -> bool {
    if y == 0 || y >= world.len() - mat_size || x == 0 || x >= world.len() - mat_size {
        return false;
    }

    if world[y][x].tile_type.properties().can_hold(&Garbage(0)) && !(world[y][x].content != Content::None) {
        world[y][x].content = Garbage(amount);
        true
    } else {
        false
    }
}

// probability matrix
#[inline(always)]
fn generate_prob_matrix(mut size: usize, probability_step: f64) -> Vec<Vec<f64>> {
    // some edgy checks
    if size == 0 {
        return vec![vec![]];
    } else if size / 2 == 1 {
        size += 1; //we want the size to be odd
    }

    // initialize the matrix and calculate the total number of rings
    let mut matrix = vec![vec![0.0; size]; size];
    let total_rings = size / 2; // total number of ring

    // iterate over the ring
    for ring in 0..total_rings {
        // assign the probability for the ring
        let prob = 1. - probability_step * ((total_rings - ring) as f64);

        // iterate over the first row of the ring
        for col_index in ring..(size - ring) {
            matrix[ring][col_index] = prob;
            matrix[size - 1 - ring][col_index] = prob;
        }

        // iterate over the first column of the ring
        for row_index in ring..(size - ring) {
            matrix[row_index][ring] = prob;
            matrix[row_index][size - 1 - ring] = prob;
        }
    }
    matrix
}
