use crate::board::GridNode;
use crate::utils;
use crate::{adjacency_list, adjacency_list::AdjacencyEntry};
use priority_queue::{self, DoublePriorityQueue};

pub struct DijkstrasResult {
    distance: usize,
    previous: Vec<Option<usize>>,
    visited_ordered: Vec<usize>,
}

pub struct TraversedNodesResult {
    pub path: Vec<usize>,
    pub visited_ordered: Vec<usize>,
}

fn dijkstras(
    adjacency_list: Vec<AdjacencyEntry>,
    start_square_id: usize,
    end_square_id: usize,
) -> DijkstrasResult {
    let mut heap: DoublePriorityQueue<usize, usize> = priority_queue::DoublePriorityQueue::new();
    heap.push(start_square_id, 0);

    let mut distances = vec![usize::MAX; adjacency_list.len()];
    distances[start_square_id] = 0;

    let mut visited = vec![false; adjacency_list.len()];
    let mut previous: Vec<Option<usize>> = vec![None; adjacency_list.len()];
    let mut visited_ordered = Vec::new();

    while let Some((node_id, min_distance)) = heap.pop_min() {
        visited[node_id] = true;
        visited_ordered.push(node_id);
        if min_distance > distances[node_id] {
            continue;
        } else {
            distances[node_id] = min_distance;
        }

        if node_id == end_square_id {
            return DijkstrasResult {
                distance: distances[node_id],
                previous,
                visited_ordered,
            };
        }

        adjacency_list[node_id].edges.iter().for_each(|edge| {
            if visited[edge.to] {
                return;
            }

            let new_distance = min_distance + edge.weight;
            if new_distance < distances[edge.to] {
                distances[edge.to] = new_distance;
                previous[edge.to] = Some(node_id);
                heap.push(edge.to, new_distance);
            }
        });
    }

    DijkstrasResult {
        distance: usize::MAX,
        previous,
        visited_ordered,
    }
}

pub fn get_traversed_nodes(
    nodes: &Vec<GridNode>,
    start_square_id: usize,
    end_square_id: usize,
) -> TraversedNodesResult {
    let num_nodes = nodes.len();
    assert!(
        end_square_id < num_nodes,
        "End square ID is out of bounds"
    );
    assert!(
        start_square_id < num_nodes,
        "Start square ID is out of bounds"
    );

    let adjacency_list = adjacency_list::create_adjacency_list(&nodes);
    let mut result = dijkstras(adjacency_list, start_square_id, end_square_id);

    utils::drop_first_and_last(&mut result.visited_ordered);

    if result.distance == usize::MAX {
        return TraversedNodesResult {
            path: Vec::new(),
            visited_ordered: result.visited_ordered,
        };
    }

    let mut path = Vec::new();
    let mut current_square_id = end_square_id;

    while current_square_id != start_square_id {
        if let Some(previous_square_id) = result.previous[current_square_id] {
            path.push(previous_square_id);
            current_square_id = previous_square_id;
        } else {
            break;
        }
    }

    path.pop();
    path.reverse();

    TraversedNodesResult {
        path,
        visited_ordered: result.visited_ordered,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency_list;
    use crate::tests_common;

    #[test]
    fn dijkstras_returns_correct_result() {
        let mock_grid = tests_common::get_mock_grid();
        let mock_square_statuses = tests_common::get_mock_nodes(mock_grid);
        let adjacency_list = adjacency_list::create_adjacency_list(&mock_square_statuses);
        let expected_distance = 22;
        let start_node_id = 0;
        let end_node_id = 8;

        let result = dijkstras(adjacency_list, start_node_id, end_node_id);

        assert_eq!(result.distance, expected_distance);
    }

    #[test]
    fn get_path_returns_correct_path() {
        let mock_grid = tests_common::get_mock_grid();
        let mock_square_statuses = tests_common::get_mock_nodes(mock_grid);
        let num_expected_nodes = 21;
        let expected_path = vec![
            10, 20, 30, 40, 50, 60, 61, 62, 63, 64, 65, 66, 56, 46, 47, 48, 49, 39, 29, 19, 9,
        ];
        let expected_visited_ordered = [
            10, 20, 11, 12, 30, 22, 40, 2, 13, 3, 41, 50, 32, 23, 33, 60, 70, 61, 43, 62, 53, 71,
            80, 90, 72, 63, 64, 82, 73, 92, 83, 74, 65, 66, 93, 76, 56, 67, 46, 68, 77, 57, 86, 69,
            87, 36, 78, 47, 35, 48, 88, 79, 59, 37, 97, 89, 49, 98, 39, 99, 29, 19, 28, 9, 18, 17,
        ];
        let start_node_id = 0;
        let end_node_id = 8;

        let result = get_traversed_nodes(&mock_square_statuses, start_node_id, end_node_id);
        let path = result.path;
        let visited_ordered = result.visited_ordered;

        assert_eq!(num_expected_nodes, path.len());
        assert_eq!(expected_path, path);
        expected_visited_ordered
            .iter()
            .enumerate()
            .for_each(|(index, &node_id)| {
                assert_eq!(node_id, visited_ordered[index]);
            });
    }

    #[test]
    fn get_path_returns_empty_path_when_end_unreachable() {
        let mock_grid = tests_common::get_mock_grid_with_unreachable_end();
        let mock_square_statuses = tests_common::get_mock_nodes(mock_grid);

        let start_node_id = 0;
        let end_node_id = 8;

        let result = get_traversed_nodes(&mock_square_statuses, start_node_id, end_node_id);
        let path = result.path;

        assert!(path.is_empty());
    }
}
