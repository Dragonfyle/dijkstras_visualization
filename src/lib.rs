use board::Board;

mod adjacency_list;
mod board;
mod dijkstras;
#[cfg(test)]
mod tests_common;
mod utils;
mod visualizer;
mod grid_actions;

const GRID_SIZE: usize = 2500;
const DEFAULT_COLOR: &str = "bg-gray-700";
const START_COLOR: &str = "bg-green-700";
const END_COLOR: &str = "bg-red-700";
const OFF_COLOR: &str = "bg-gray-900";
const VISITED_COLOR: &str = "bg-blue-800";
const PATH_COLOR: &str = "bg-yellow-600";

#[derive(Clone, Debug, PartialEq)]
enum NodeStatus {
    On,
    Off,
    Start,
    End,
    Path,
    Visited,
}

pub fn run() {
    yew::Renderer::<Board>::new().render();
}
