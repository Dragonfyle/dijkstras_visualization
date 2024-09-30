use crate::SquareStatus;

#[derive(Debug, Hash)]
pub struct Edge {
    pub to: usize,
    pub weight: usize,
}

pub struct AdjacencyEntry {
    pub edges: Vec<Edge>,
}

fn is_square_on(square_id: usize, square_status_map: &Vec<SquareStatus>) -> bool {
    if let SquareStatus::On = square_status_map[square_id] {
        return true;
    }

    false
}

fn add_edge(edges: &mut Vec<Edge>, to: usize, weight: usize) {
    edges.push(Edge { to, weight });
}

fn get_edges_to_neighbors(square_id: usize, square_status_map: &Vec<SquareStatus>) -> Vec<Edge> {
    let mut edges_to_neighbors = Vec::new();
    let side_length = (square_status_map.len() as f64).sqrt() as usize;
    let row = square_id / side_length;
    let col = square_id % side_length;

    //the value 1 is the edge weight. here all edges have the same weight for demo purposes
    if col > 0 && is_square_on(square_id - 1, square_status_map) {
        add_edge(&mut edges_to_neighbors, square_id - 1, 1);
    }

    if col < side_length - 1 && is_square_on(square_id + 1, square_status_map) {
        add_edge(&mut edges_to_neighbors, square_id + 1, 1);
    }

    if row > 0 && is_square_on(square_id - side_length, square_status_map) {
        add_edge(&mut edges_to_neighbors, square_id - side_length, 1);
    }

    if row < side_length - 1 && is_square_on(square_id + side_length, square_status_map) {
        add_edge(&mut edges_to_neighbors, square_id + side_length, 1);
    }

    edges_to_neighbors
}
    
pub fn create_adjacency_list(square_status_map: &Vec<SquareStatus>) -> Vec<AdjacencyEntry> {

    square_status_map.iter().enumerate().map(|(i, _square)| {
        AdjacencyEntry {
            edges: get_edges_to_neighbors(i, square_status_map)
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_common;

    #[test]
    fn create_adjacency_list_returns_correct_list() {
        let mock_grid = tests_common::get_mock_grid();
        let mock_square_statuses = tests_common::get_mock_squares(mock_grid);
        
        let adjacency_list = create_adjacency_list(&mock_square_statuses);

        let number_of_expected_edges_list = vec![
            1, 3, 2, 2, 1, 1, 2, 3, 3, 2,
            3, 2, 4, 3, 1, 1, 2, 3, 4, 3,
            2, 3, 3, 3, 1, 1, 2, 3, 2, 3,
            2, 3, 2, 3, 2, 1, 3, 2, 4, 2,
            3, 1, 3, 2, 1, 2, 3, 4, 2, 3,
            2, 3, 2, 2, 2, 2, 3, 3, 4, 2,
            3, 3, 3, 4, 3, 2, 4, 4, 3, 3,
            3, 3, 4, 4, 2, 3, 3, 4, 4, 3,
            2, 3, 3, 3, 2, 1, 2, 4, 4, 3,
            1, 2, 2, 2, 1, 0, 2, 2, 3, 2
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
}