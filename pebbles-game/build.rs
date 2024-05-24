// build.rs
// Where ProgramMetadata is your metadata structure

use pebbles_game_io::ProgramMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ProgramMetadata>();
}
