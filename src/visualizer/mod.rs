use crate::{utils, NodeStatus};
use crate::board::Nodes;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

pub fn visualize(
    nodes: Nodes,
    traversed_nodes: Vec<usize>,
    path: Vec<usize>,
    end_of_visualization_callback: impl Fn() + 'static,
) {
    spawn_local(async move {
        let mut nodes_borrow = nodes.borrow_mut();
        for chunk in traversed_nodes.chunks(10) {
            for &node_id in chunk {
                if let Some(node_ref) = nodes_borrow
                    .get(node_id)
                    .unwrap()
                    .node_ref
                    .cast::<HtmlElement>()
                {
                    let node_status_map_entry = &mut nodes_borrow[node_id].node_status;
                    utils::set_square_color(&node_ref, NodeStatus::Visited);
                    utils::set_node_status(node_status_map_entry, NodeStatus::Visited);
                }
            }
            TimeoutFuture::new(5).await;
        }

        for &node_id in path.iter() {
            if let Some(node_ref) = nodes_borrow
                .get(node_id)
                .unwrap()
                .node_ref
                .cast::<HtmlElement>()
            {
                let node_status_map_entry = &mut nodes_borrow[node_id].node_status;
                utils::set_square_color(&node_ref, NodeStatus::Path);
                utils::set_node_status(node_status_map_entry, NodeStatus::Path);
            }
        }
        end_of_visualization_callback();
    });
}
