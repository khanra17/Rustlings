use crate::first_floor::toys_room::i_love_toys;
use crate::ground_floor::storage_room::i_found_telescope;
use crate::second_floor::astronomy_tower::i_see_the_stars;

mod ground_floor;
mod first_floor;
mod second_floor;

pub fn day_x() {
    i_love_toys();
    i_found_telescope();
    i_see_the_stars();
}

