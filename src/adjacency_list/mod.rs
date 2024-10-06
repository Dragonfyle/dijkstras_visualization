use crate::{board::GridNode, NodeStatus};

#[derive(Debug, Hash)]
pub struct Edge {
    pub to: usize,
    pub weight: usize,
}

pub struct AdjacencyEntry {
    pub edges: Vec<Edge>,
}

fn is_node_on(node_id: usize, nodes: &Vec<GridNode>) -> bool {
    if let NodeStatus::Off = nodes[node_id].node_status {
        return false;
    }

    true
}

fn add_edge(edges: &mut Vec<Edge>, to: usize, weight: usize) {
    edges.push(Edge { to, weight });
}

fn get_edges_to_neighbors(node_id: usize, nodes: &Vec<GridNode>) -> Vec<Edge> {
    let mut edges_to_neighbors = Vec::new();
    let side_length = (nodes.len() as f64).sqrt() as usize;
    let row = node_id / side_length;
    let col = node_id % side_length;

    //the value 1 is the edge weight. here all edges have the same weight for demo purposes
    if col > 0 && is_node_on(node_id - 1, nodes) {
        add_edge(&mut edges_to_neighbors, node_id - 1, 1);
    }

    if col < side_length - 1 && is_node_on(node_id + 1, nodes) {
        add_edge(&mut edges_to_neighbors, node_id + 1, 1);
    }

    if row > 0 && is_node_on(node_id - side_length, nodes) {
        add_edge(&mut edges_to_neighbors, node_id - side_length, 1);
    }

    if row < side_length - 1 && is_node_on(node_id + side_length, nodes) {
        add_edge(&mut edges_to_neighbors, node_id + side_length, 1);
    }

    edges_to_neighbors
}

pub fn create_adjacency_list(nodes: &Vec<GridNode>) -> Vec<AdjacencyEntry> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, _node)| AdjacencyEntry {
            edges: get_edges_to_neighbors(i, nodes),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_common;

    #[test]
    fn create_adjacency_list_returns_correct_list() {
        let mock_grid = tests_common::get_mock_grid();
        let mock_node_statuses = tests_common::get_mock_nodes(mock_grid);

        let adjacency_list = create_adjacency_list(&mock_node_statuses);

        let number_of_expected_edges_list = vec![
            1, 3, 2, 2, 1, 1, 2, 3, 3, 2, 3, 2, 4, 3, 1, 1, 2, 3, 4, 3, 2, 3, 3, 3, 1, 1, 2, 3, 2,
            3, 2, 3, 2, 3, 2, 1, 3, 2, 4, 2, 3, 1, 3, 2, 1, 2, 3, 4, 2, 3, 2, 3, 2, 2, 2, 2, 3, 3,
            4, 2, 3, 3, 3, 4, 3, 2, 4, 4, 3, 3, 3, 3, 4, 4, 2, 3, 3, 4, 4, 3, 2, 3, 3, 3, 2, 1, 2,
            4, 4, 3, 1, 2, 2, 2, 1, 0, 2, 2, 3, 2,
        ];

        assert_eq!(adjacency_list.len(), 100);

        for (i, &expected_edges) in number_of_expected_edges_list.iter().enumerate() {
            assert!(
                adjacency_list[i].edges.len() == expected_edges,
                "Mismatch at index {}: expected {} edges, but got {}",
                i,
                expected_edges,
                adjacency_list[i].edges.len()
            );
        }
    }

    #[test]
    fn correctly_identifies_off_nodes() {
        let mock_grid = tests_common::get_mock_grid();
        let mock_node_statuses = tests_common::get_mock_nodes(mock_grid);

        let first_row_expected_values = vec![
            true, false, true, true, false, false, true, true, true, true,
        ];
        let last_row_expected_values = vec![
            true, false, true, true, false, false, false, true, true, true,
        ];

        for i in 0..10 {
            assert_eq!(
                is_node_on(i, &mock_node_statuses),
                first_row_expected_values[i]
            );
        }

        for i in 0..10 {
            assert_eq!(
                is_node_on(i + 90, &mock_node_statuses),
                last_row_expected_values[i]
            );
        }
    }
}
