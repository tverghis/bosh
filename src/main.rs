use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Universe {
    cells: [[Cell; Universe::UNIVERSE_ROWS]; Universe::UNIVERSE_COLS],
}

impl Universe {
    const UNIVERSE_ROWS: usize = 5;
    const UNIVERSE_COLS: usize = 5;

    const GEN_SLEEP: Duration = Duration::from_millis(700);

    fn new_empty() -> Self {
        Self {
            cells: [[Cell::new(CellState::Dead); Universe::UNIVERSE_ROWS]; Universe::UNIVERSE_COLS],
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    fn tick(&mut self) {}
}

impl std::fmt::Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..Universe::UNIVERSE_ROWS {
            for y in 0..Universe::UNIVERSE_COLS {
                write!(f, "{:?}", self.cells[x][y])?;
            }
            write!(f, "\n")?;
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

fn main() {
    let mut universe = Universe::new_empty();

    universe.set_cell(2, 1, Cell::new(CellState::Alive));
    universe.set_cell(2, 2, Cell::new(CellState::Alive));
    universe.set_cell(2, 3, Cell::new(CellState::Alive));

    loop {
        print!("\x1B[2J\x1B[H"); // reset screen
        print!("{:?}", universe);
        universe.tick();
        std::thread::sleep(Universe::GEN_SLEEP);
    }
}
