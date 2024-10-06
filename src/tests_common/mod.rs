use yew::{html, NodeRef};

use crate::{board::GridNode, NodeStatus};

pub fn get_mock_grid() -> Vec<usize> {
    vec![
        2, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 0, 0, 3,
    ]
}

pub fn get_mock_nodes(mock_grid: Vec<usize>) -> Vec<GridNode> {
    mock_grid
        .iter()
        .map(|node| match node {
            0 => GridNode {node_status: NodeStatus::On, node: html! {<div></div>}, node_ref: NodeRef::default()},
            1 => GridNode {node_status: NodeStatus::Off, node: html! {<div></div>}, node_ref: NodeRef::default()},
            2 => GridNode {node_status: NodeStatus::Start, node: html! {<div></div>}, node_ref: NodeRef::default()},
            3 => GridNode {node_status: NodeStatus::End, node: html! {<div></div>}, node_ref: NodeRef::default()},
            _ => GridNode {node_status: NodeStatus::On, node: html! {<div></div>}, node_ref: NodeRef::default()},
        })
        .collect()
}

pub fn get_mock_grid_with_unreachable_end() -> Vec<usize> {
    vec![
        0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 0, 0, 0,
    ]
}
