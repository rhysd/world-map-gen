use game_map_gen::{draw, gen};

fn main() {
    if let Err(e) = draw::draw_term(&gen::gen_small(10, 10)) {
        eprintln!("Error: {}", e);
    }
}
