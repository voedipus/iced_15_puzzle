use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, row, text},
};

const GRID_SIZE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    TilePressed(usize, usize),
    Shuffle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    value: Option<u8>, // None represents the empty tile
}

impl Tile {
    fn empty() -> Self {
        Self { value: None }
    }

    fn new(value: u8) -> Self {
        Self { value: Some(value) }
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    tiles: [[Tile; GRID_SIZE]; GRID_SIZE],
    moves: u32,
}

impl Default for Puzzle {
    fn default() -> Self {
        let mut tiles = [[Tile::empty(); GRID_SIZE]; GRID_SIZE];
        let mut value = 1u8;

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if i == GRID_SIZE - 1 && j == GRID_SIZE - 1 {
                    // Last tile is empty
                    tiles[i][j] = Tile::empty();
                } else {
                    tiles[i][j] = Tile::new(value);
                    value += 1;
                }
            }
        }

        Self { tiles, moves: 0 }
    }
}

impl Puzzle {
    fn find_empty(&self) -> Option<(usize, usize)> {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.tiles[i][j].is_empty() {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn is_adjacent_to_empty(&self, row: usize, col: usize) -> bool {
        if let Some((empty_row, empty_col)) = self.find_empty() {
            // Check if the tile is adjacent to the empty tile
            (row == empty_row && (col as isize - empty_col as isize).abs() == 1)
                || (col == empty_col && (row as isize - empty_row as isize).abs() == 1)
        } else {
            false
        }
    }

    fn move_tile(&mut self, row: usize, col: usize) -> bool {
        if self.is_adjacent_to_empty(row, col) {
            if let Some((empty_row, empty_col)) = self.find_empty() {
                // Swap the tile with the empty tile
                let temp = self.tiles[row][col];
                self.tiles[row][col] = self.tiles[empty_row][empty_col];
                self.tiles[empty_row][empty_col] = temp;
                self.moves += 1;
                return true;
            }
        }
        false
    }

    fn shuffle(&mut self) {
        // Reset to solved state first
        *self = Self::default();

        // Make a series of random valid moves
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            if let Some((empty_row, empty_col)) = self.find_empty() {
                // Find all valid moves
                let mut valid_moves = Vec::new();

                if empty_row > 0 {
                    valid_moves.push((empty_row - 1, empty_col));
                }
                if empty_row < GRID_SIZE - 1 {
                    valid_moves.push((empty_row + 1, empty_col));
                }
                if empty_col > 0 {
                    valid_moves.push((empty_row, empty_col - 1));
                }
                if empty_col < GRID_SIZE - 1 {
                    valid_moves.push((empty_row, empty_col + 1));
                }

                // Make a random valid move
                if let Some((row, col)) = valid_moves.get(rng.gen_range(0..valid_moves.len())) {
                    self.move_tile(*row, *col);
                }
            }
        }

        // Reset move counter after shuffling
        self.moves = 0;
    }

    fn is_solved(&self) -> bool {
        let mut expected_value = 1u8;

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if i == GRID_SIZE - 1 && j == GRID_SIZE - 1 {
                    // Last tile should be empty
                    if !self.tiles[i][j].is_empty() {
                        return false;
                    }
                } else {
                    if self.tiles[i][j].value != Some(expected_value) {
                        return false;
                    }
                    expected_value += 1;
                }
            }
        }

        true
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TilePressed(row, col) => {
                self.move_tile(row, col);
            }
            Message::Shuffle => {
                self.shuffle();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut grid_rows = Vec::new();

        for i in 0..GRID_SIZE {
            let mut row_tiles = Vec::new();

            for j in 0..GRID_SIZE {
                let tile = &self.tiles[i][j];

                let tile_button = if tile.is_empty() {
                    button("").width(Length::Fill).height(Length::Fill)
                } else {
                    button(
                        text(tile.value.unwrap().to_string())
                            .size(24)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .center(),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::TilePressed(i, j))
                };

                row_tiles.push(tile_button.into());
            }

            grid_rows.push(row(row_tiles).spacing(5).into());
        }

        let status_text = if self.is_solved() {
            text("Puzzle Solved! 🎉").size(24)
        } else {
            text(format!("Moves: {}", self.moves)).size(20)
        };

        container(
            column![
                text("15 Puzzle").size(32),
                status_text,
                column(grid_rows).spacing(5),
                button("Shuffle").on_press(Message::Shuffle)
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(Puzzle::update, Puzzle::view)
}
