use board::Board;

mod board;
mod adjacency_list;
mod dijkstras;
#[cfg(test)]
mod tests_common;

const GRID_SIZE: usize = 2500;

#[derive(Clone)]
enum SquareStatus {
    On,
    Off,
    // Start,
    // End,
    // Path,
    // Visited,
}

pub fn run() {
    yew::Renderer::<Board>::new().render();
}

