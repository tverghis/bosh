use std::time::Duration;

const UNIVERSE_ROWS: usize = 18;
const UNIVERSE_COLS: usize = 18;

const GENERATION_TICK_RATE: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Universe {
    cells: [[Cell; UNIVERSE_ROWS]; UNIVERSE_COLS],
}

impl Universe {
    fn new_empty() -> Self {
        Self {
            cells: [[Cell::new(CellState::Dead); UNIVERSE_ROWS]; UNIVERSE_COLS],
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    fn tick(&self) -> Self {
        let mut new_universe = Universe::new_empty();

        fn alive_neighbors_count(
            cells: &[[Cell; UNIVERSE_ROWS]; UNIVERSE_COLS],
            this_x: usize,
            this_y: usize,
        ) -> usize {
            let mut sum = 0;

            for row in get_neighbor_row_range(this_x) {
                for col in get_neighbor_col_range(this_y) {
                    if (this_x, this_y) == (row, col) {
                        continue;
                    }

                    if cells[row][col].state == CellState::Alive {
                        sum += 1;
                    }
                }
            }

            sum
        }

        for row in 0..UNIVERSE_ROWS {
            for col in 0..UNIVERSE_COLS {
                let current_cell = self.cells[row][col];
                let alive_neighbors = alive_neighbors_count(&self.cells, row, col);

                let new_state = current_cell.state.transition(alive_neighbors);
                new_universe.set_cell(row, col, Cell::new(new_state));
            }
        }

        new_universe
    }
}

impl std::fmt::Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..UNIVERSE_ROWS {
            for y in 0..UNIVERSE_COLS {
                write!(f, "{:?}", self.cells[x][y])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    state: CellState,
}

impl Cell {
    fn new(state: CellState) -> Self {
        Self { state }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            CellState::Alive => write!(f, "[x]"),
            CellState::Dead => write!(f, "[ ]"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

impl CellState {
    fn transition(&self, alive_neighbors: usize) -> Self {
        match self {
            CellState::Alive => match alive_neighbors {
                2 | 3 => CellState::Alive,
                _ => CellState::Dead,
            },
            CellState::Dead => match alive_neighbors {
                3 => CellState::Alive,
                _ => CellState::Dead,
            },
        }
    }
}

fn get_neighbor_row_range(this_x: usize) -> std::ops::RangeInclusive<usize> {
    this_x.saturating_sub(1)..=(this_x + 1).clamp(this_x, UNIVERSE_ROWS - 1)
}

fn get_neighbor_col_range(this_y: usize) -> std::ops::RangeInclusive<usize> {
    this_y.saturating_sub(1)..=(this_y + 1).clamp(this_y, UNIVERSE_COLS - 1)
}

fn main() {
    let mut universe = Universe::new_empty();

    universe.set_cell(0, 0, Cell::new(CellState::Alive));
    universe.set_cell(0, 2, Cell::new(CellState::Alive));
    universe.set_cell(1, 1, Cell::new(CellState::Alive));
    universe.set_cell(1, 2, Cell::new(CellState::Alive));
    universe.set_cell(2, 1, Cell::new(CellState::Alive));

    loop {
        print!("\x1B[2J\x1B[H"); // clear screen and set cursor position to top-left
        print!("{:?}", universe);

        universe = universe.tick();
        std::thread::sleep(GENERATION_TICK_RATE);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_neighbor_row_range_for_top_row() {
        assert_eq!(get_neighbor_row_range(0), 0..=1);
    }

    #[test]
    fn get_neighbor_row_range_for_bottom_row() {
        assert_eq!(get_neighbor_row_range(4), 3..=4);
    }

    #[test]
    fn get_neighbor_row_range_for_middle_row() {
        assert_eq!(get_neighbor_row_range(2), 1..=3);
    }

    #[test]
    fn get_neighbor_col_range_for_left_col() {
        assert_eq!(get_neighbor_col_range(0), 0..=1);
    }

    #[test]
    fn get_neighbor_col_range_for_right_col() {
        assert_eq!(get_neighbor_col_range(4), 3..=4);
    }

    #[test]
    fn get_neighbor_col_range_for_middle_col() {
        assert_eq!(get_neighbor_col_range(2), 1..=3);
    }
}
