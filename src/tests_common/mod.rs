use crate::SquareStatus;

pub fn get_mock_grid() -> Vec<usize> {
    vec![
        0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 0, 0, 1, 0, 0, 0, 1, 0,
        0, 0, 1, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 0, 0, 0,
    ]
}

pub fn get_mock_squares(mock_grid: Vec<usize>) -> Vec<SquareStatus> {
    mock_grid.iter().map(|square| {
        match square {
            0 => SquareStatus::On,
            1 => SquareStatus::Off,
            _ => SquareStatus::On,
        }
    }).collect()
}

pub fn get_mock_grid_with_unreachable_end() -> Vec<usize> {
    vec![
        0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 0, 0, 1, 0, 0, 0, 1, 0,
        0, 0, 1, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 1, 1, 1, 0, 0, 0,
    ]
}