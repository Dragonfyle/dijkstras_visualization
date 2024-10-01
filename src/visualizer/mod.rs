use crate::board::SquareNode;
use crate::{utils, SquareStatus};
use gloo_timers::future::TimeoutFuture;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub fn visualize(
    squares: Rc<RefCell<Vec<SquareNode>>>,
    square_status_map: Rc<RefCell<Vec<SquareStatus>>>,
    traversed_nodes: Vec<usize>,
    path: Vec<usize>,
    end_of_visualization_callback: impl Fn() + 'static,
) {
    spawn_local(async move {
        let squares_borrow = squares.borrow();
        for chunk in traversed_nodes.chunks(10) {
            for &node_id in chunk {
                if let Some(node_ref) = squares_borrow
                    .get(node_id)
                    .unwrap()
                    .node_ref
                    .cast::<HtmlElement>()
                {
                    let square_status_map_entry = &mut square_status_map.borrow_mut()[node_id];
                    utils::set_square_color(&node_ref, SquareStatus::Visited);
                    utils::set_square_status(square_status_map_entry, SquareStatus::Visited);
                }
            }
            TimeoutFuture::new(5).await;
        }

        for &node_id in path.iter() {
            if let Some(node_ref) = squares_borrow
                .get(node_id)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                let square_status_map_entry = &mut square_status_map.borrow_mut()[node_id];
                utils::set_square_color(&node_ref, SquareStatus::Path);
                utils::set_square_status(square_status_map_entry, SquareStatus::Path);
            }
        }
        end_of_visualization_callback();
    });
}
