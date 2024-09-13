mod board;

use board::Board;

pub fn run() {
    yew::Renderer::<Board>::new().render();
}

