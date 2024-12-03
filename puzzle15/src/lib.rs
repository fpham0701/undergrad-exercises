mod circuits;

/// Holds information about which tile is in which position.
/// Should be fairly compact and easy to copy.
#[derive(Debug, Clone)]
pub struct GameState {
    grid: [[Option<u8>; 4]; 4], // option so there can be nothing
    // TODO
}

/// Creates the default position of tiles, starting with 1 in the top left corner.
impl Default for GameState {
    fn default() -> Self {
        let mut grid = [[None; 4]; 4]; 
        let mut value = 1;

        for i in 0..4 {
            for j in 0..4 {
                if value <= 15 {
                    grid[i][j] = Some(value);
                    value += 1; // Update value
                }
            }
        }
        GameState { grid }
    }
}

/// Generates a human-readable representation of the game state.
impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            print!("|  ");
            for j in 0..4 {
                match self.grid[i][j] {
                    Some(val) => print!("{} ", val),
                    None => print!("  "),
                }
            }
        }
        Ok(())
    }
}

/// Checks whether two game states are the same,.
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

/// Feel free to ignore this. (but do not remove)
impl Eq for GameState {}

impl GameState {
    /// Updates a position with a new tile.
    pub fn set(&mut self, x: u8, y: u8, tile: Option<u8>) {
        self.grid[x][y] = tile
    }

    /// Returns the tile at position x,y.
    pub fn get(&self, x: u8, y: u8) -> Option<u8> {
        self.grid[x][y] 
    }

    /// Returns false if there is a duplicate tile in this game state.
    pub fn all_tiles_unique(&self) -> bool {
        let mut tiles = Vec::new();
        
        for i in 0.len(self.grid) {
            for j in 0.len(self.grid[i]) {
                if tiles.contains(&self.grid[i][j]) {
                    false
                } 
                else {
                    tiles.push(self.grid[i][j]);
                }
            }
        }
        true
    }

    /// Swaps the tile from (x1,y1) with the tile from (x2,y2)
    pub fn swap(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let temp = self.grid[x1][y1];
        self.set(x1, y1, self.grid[x2][y2]);
        self.set(x2, y2, temp);
    }

    /// Updates the state to reflect the move that was performed. Returns false if the move was
    /// not possible.
    pub fn perform_move(&mut self, m: Move) -> bool {
        todo!()
    }

    /// Performs a series of moves. Returns the number of moves that were successful.
    pub fn perform_moves(&mut self, moves: &[Move]) -> usize {
        todo!()
    }

    /// Tries to parse a game state from the provided string.
    /// Returns None if parsing is not possible, or if the parsed game state would contain
    /// duplicate or invalid tiles.
    /// Ignores whitespace.
    pub fn from_str(s: &str) -> Option<Self> {
        todo!()
    }
}

/// Finds the minimal number of moves needed to get from one state to the other.
/// Might run forever if there is no path, so use with caution!
pub fn find_shortest_path(from: GameState, to: GameState) -> Vec<Move> {
    todo!()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum Move {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_state() {
        let state = GameState::default();
        assert_eq!(state.get(0, 0), Some(1));
        assert_eq!(state.get(1, 0), Some(2));
        assert_eq!(state.get(2, 0), Some(3));
        assert_eq!(state.get(3, 0), Some(4));
        assert_eq!(state.get(0, 1), Some(5));
        assert_eq!(state.get(1, 1), Some(6));
        assert_eq!(state.get(2, 1), Some(7));
        assert_eq!(state.get(3, 1), Some(8));
        assert_eq!(state.get(0, 2), Some(9));
        assert_eq!(state.get(1, 2), Some(10));
        assert_eq!(state.get(2, 2), Some(11));
        assert_eq!(state.get(3, 2), Some(12));
        assert_eq!(state.get(0, 3), Some(13));
        assert_eq!(state.get(1, 3), Some(14));
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
    }

    #[test]
    fn test_set_game_state() {
        let mut state = GameState::default();
        state.set(0, 2, Some(3));
        assert_eq!(state.get(0, 2), Some(3));
        // TODO: add more tests
    }

    const DEFAULT_STATE_STR: &'static str = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 | 15 |    |
";

    #[test]
    fn test_display_game_state() {
        let state = GameState::default();
        assert_eq!(DEFAULT_STATE_STR, format!("{state}"));

        // TODO: add more tests
    }

    #[test]
    fn test_validate_game_state() {
        let mut state = GameState::default();
        assert!(state.all_tiles_unique());
        state.set(3, 0, Some(1));
        assert!(!state.all_tiles_unique());
        state.set(0, 0, Some(4));
        assert!(state.all_tiles_unique());

        // TODO: add more tests
    }

    #[test]
    fn test_swap() {
        let mut state = GameState::default();
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
        state.swap(2, 3, 3, 3);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(2, 3), None);
        assert_eq!(state.get(3, 3), Some(15));

        //
        state.swap(0, 0, 2, 2);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 0), Some(11));

        // TODO: add more tests
    }

    #[test]
    fn test_perform_move() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::RightToLeft));
        assert!(!state.perform_move(Move::BottomToTop));
        assert!(state.perform_move(Move::TopToBottom));
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(3, 3), Some(12));
        assert_eq!(state.get(3, 2), None);
        assert!(state.perform_move(Move::LeftToRight));
        assert_eq!(state.get(3, 2), Some(11));
        assert_eq!(state.get(2, 2), None);

        // TODO: add more tests
    }

    #[test]
    fn test_game_state_equality() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::BottomToTop));
        assert_eq!(state, GameState::default());
        assert!(state.perform_move(Move::TopToBottom));
        let mut state_2 = GameState::default();
        state_2.set(3, 3, Some(12));
        state_2.set(3, 2, None);
        assert_eq!(state, state_2);

        // TODO: add more tests
    }

    #[test]
    fn test_perform_moves() {
        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::RightToLeft, Move::BottomToTop, Move::TopToBottom]),
            1
        );

        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::TopToBottom, Move::TopToBottom, Move::TopToBottom]),
            3
        );
        let expected = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        assert_eq!(expected, format!("{state}"));

        // TODO: add more tests
    }

    #[test]
    fn test_parse_state() {
        assert_eq!(
            GameState::from_str(DEFAULT_STATE_STR).unwrap(),
            GameState::default()
        );

        let wrong0 = "\
|  1 | 22 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong1 = "\
|  1 |  2 ,  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong2 = "\
|  1 |  2 |  3 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong3 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
| 13 | 14 | 15 | 12 |
";
        let wrong4 = "\
|  1 |  2 |  3 |    | 1 |
|  5 |  6 |  7 |  4 | 1 |
|  9 | 10 | 11 |  8 | 1 |
| 13 | 14 | 15 | 12 | 1 |
";
        let wrong5 = "\
|  1 |  2 |  3 |    |
|  5 |  2 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        assert!(GameState::from_str(wrong0).is_none());
        assert!(GameState::from_str(wrong1).is_none());
        assert!(GameState::from_str(wrong2).is_none());
        assert!(GameState::from_str(wrong3).is_none());
        assert!(GameState::from_str(wrong4).is_none());
        assert!(GameState::from_str(wrong5).is_none());

        // TODO: add more tests
    }

    #[test]
    fn test_find_shortest_path() {
        let expected_moves = [Move::TopToBottom, Move::TopToBottom, Move::TopToBottom];
        let mut state = GameState::default();
        assert_eq!(state.perform_moves(&expected_moves), 3);

        let actual_moves = find_shortest_path(GameState::default(), state);
        assert_eq!(actual_moves.len(), 3);
        assert_eq!(actual_moves, expected_moves);

        // TODO: add more tests
    }
}
