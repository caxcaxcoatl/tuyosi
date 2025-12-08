
pub struct GameMatrix {
    // solution : [u8; 81]
}

// Struct Game
//
// array uint8  [81]
// - solution
// - user
// - revealed
// - current (revealed + user)
//
// All below return bottom left cell
// - get_pos_line (1, 1)
// - get_pos_col (1,1) 
// - get_index_from_square_pos (1,1)
//
// All below return top right cell:
//
// - get_pos_line (9,9)
// - get_pos_col (9,9)
// - get_index_from_square_pos (9,9)
//
// Top left:
//
// - get_pos_line (9,1)
// - get_pos_col (1,9)
// - get_index_from_square_pos (7,7)
//
// Opposite functions:
//
// get_cell_line (0) -> 1, 1 (first cell in first line)
// get_cell_line (11) -> 2, 2 
// get_square_line (11) -> 4, 2
//
// Use the opposite functions to test all cells for all three get_pos
// Also test extremes, select and incorrect options (such as any zero entries)

impl GameMatrix{
    
    /// Returns the index in the array of the given position in the given square.
    /// Both parameter utilize a standard keyboard numpad for location.  That is:
    ///
    /// - 1 is bottom left
    /// - 5 is the center item
    /// - 7 is top left
    /// - 9 is top right
    ///
    /// ```
    /// use tuyosi::GameMatrix;
    /// assert_eq! (GameMatrix::get_index_from_square_pos(1,1), 0);
    /// assert_eq! (GameMatrix::get_index_from_square_pos(9,9), 80);
    /// assert_eq! (GameMatrix::get_index_from_square_pos(4,1), 27);
    /// ```
    pub fn get_index_from_square_pos (square: u8, pos: u8) -> usize {
        let (square_col_shift, square_line_shift)   = GameMatrix::get_square_shift (square);
        let (pos_col_shift, pos_line_shift)   = GameMatrix::get_square_shift (pos);
        square_col_shift * 3 + square_line_shift * 3 * 9 + pos_col_shift + pos_line_shift * 9
    }

    /// Given an array position, identify which square it belongs to
    ///
    /// ```
    /// use tuyosi::GameMatrix;
    ///
    /// assert_eq! (GameMatrix::get_square_from_index(0), 1);
    /// assert_eq! (GameMatrix::get_square_from_index(80), 9);
    /// assert_eq! (GameMatrix::get_square_from_index(50), 5);
    /// assert_eq! (GameMatrix::get_square_from_index(51), 6);
    /// assert_eq! (GameMatrix::get_square_from_index(59), 8);
    /// assert_eq! (GameMatrix::get_square_from_index(60), 9);
    /// ```
    pub fn get_square_from_index (index: usize) -> u8{
        let index = index as u8;
        dbg! ("----- get_square_from_index ----");
        dbg! (index);
        let square_line_zero = index - index % 9;
        dbg! (square_line_zero);
        let square_col = ( index - square_line_zero ) / 3;
        dbg! (square_col);
        let square_line = index / 9 / 3;
        dbg! (square_line);
        dbg! (square_line * 3 + square_col + 1)
    }

    /// This is the oposite of get_index_from_square_pos: given an array index, return 
    /// the square and position within the square
    ///
    /// ```
    /// use tuyosi::GameMatrix;
    ///
    /// assert_eq! (GameMatrix::get_square_pos_from_index(0), (1,1));
    /// assert_eq! (GameMatrix::get_square_pos_from_index(54), (7,1));
    /// assert_eq! (GameMatrix::get_square_pos_from_index(80), (9,9));
    /// assert_eq! (GameMatrix::get_square_pos_from_index(1), (1,2));
    /// assert_eq! (GameMatrix::get_square_pos_from_index(3), (2,1));
    /// ```
    ///
    pub fn get_square_pos_from_index (index: usize) -> (u8, u8) {
        let square = GameMatrix::get_square_from_index (index);
        dbg! (square);
        dbg! ("---- get_square_pos_from_index ----");
        dbg! (index);
        let index = index as u8;
        let (col_shift, line_shift) = GameMatrix::get_square_shift (square);
        let (col_shift, line_shift) = (col_shift as u8, line_shift as u8);
        dbg! (col_shift, line_shift);
        let square_zero = col_shift * 3 + line_shift * 9 * 3;
        dbg! (square_zero);
        let square_line_zero = square_zero - ( square_zero % 9 );
        dbg! (square_line_zero);
        let square_line = (index - square_line_zero) / 9;
        dbg!  (square_line);
        let square_col = (index - square_line_zero - 3 * col_shift ) % 3;
        dbg! (square_col);
        dbg! (square, square_line * 3 + 1 + square_col)
    }

    /// Given a square position, returns the colum and line shifts to be applied
    /// on a matrix to place it.
    ///
    /// The square position is the placement of the number in a standard keyboard numpad.
    ///
    /// ```
    /// use tuyosi::GameMatrix;
    /// assert_eq! (GameMatrix::get_square_shift (1), (0,0));
    /// assert_eq! (GameMatrix::get_square_shift (2), (1,0));
    /// assert_eq! (GameMatrix::get_square_shift (3), (2,0));
    /// assert_eq! (GameMatrix::get_square_shift (4), (0,1));
    /// assert_eq! (GameMatrix::get_square_shift (5), (1,1));
    /// assert_eq! (GameMatrix::get_square_shift (6), (2,1));
    /// assert_eq! (GameMatrix::get_square_shift (7), (0,2));
    /// assert_eq! (GameMatrix::get_square_shift (8), (1,2));
    /// assert_eq! (GameMatrix::get_square_shift (9), (2,2));
    /// ```
    pub fn get_square_shift (pos: u8) -> (usize, usize) {
        let pos = pos as usize;
        ( (pos-1) % 3, (pos-1) / 3)
    }

    /// Returns the position in the array of the given item on the given line
    /// Line numbers start at 1, at the bottom.  So, the top line is #9
    /// Item numbers are left to right, also starting in 1
    pub fn get_pos_line (line: u8, item: u8) -> usize {
        ((line-1) * 9 + item - 1) as usize
    }

    /// This is the opposite of get_pos_line: given an array position (starting at 0),
    /// return its line and item
    pub fn get_cell_line (cell: usize) -> (u8, u8) {
        let (div, module) = ( ( cell / 9) as u8, (cell % 9) as u8);
        (div+1, module+1)
    }

    /// Returns the position in the array of the tiven item on the given column.
    /// Column numbers start at 1 and go left to right.
    /// 1,1 -> bottom left
    /// 1,9 -> top left
    /// 9,9 -> top right
    pub fn get_pos_col (col: u8, item: u8) -> usize {
        ( (item-1) *9 + (col-1) ) as usize
    }

    /// This is the opposite of get_pos_col: given an array position (starting at )),
    /// return the column and item
    pub fn get_cell_col (cell:  usize) -> (u8, u8) {
        let (div, module) = ( ( cell / 9) as u8, (cell % 9) as u8);
        ( module+1, div+1)
    }

}

#[cfg(test)]
mod test {
    use super::*;

#[test]
    fn test_get_pos_line () {
        assert_eq! (GameMatrix::get_pos_line (1, 1), 0);
        assert_eq! (GameMatrix::get_pos_line (2, 1), 9);
        assert_eq! (GameMatrix::get_pos_line (9, 9), 80);

        assert_eq! (GameMatrix:: get_cell_line (0), (1,1));
        assert_eq! (GameMatrix:: get_cell_line (80), (9,9));

        for line in 1..10 {
            for item in 1..10 {
                assert_eq! (GameMatrix::get_cell_line(GameMatrix::get_pos_line(line, item)), (line, item))
            }
        }
    }

#[test]
    fn test_get_pos_col () {
        assert_eq! (GameMatrix::get_pos_col(1,1), 0);
        assert_eq! (GameMatrix::get_pos_col(2,1), 1);
        assert_eq! (GameMatrix::get_pos_col(9,9), 80);

        assert_eq! (GameMatrix::get_cell_col(0), (1,1));
        assert_eq! (GameMatrix::get_cell_col(80), (9,9));
        assert_eq! (GameMatrix::get_cell_col(1), (2,1));
        for line in 1..10 {
            for item in 1..10 {
                assert_eq! (GameMatrix::get_cell_col(GameMatrix::get_pos_col(line, item)), (line, item))
            }
        }
    }

#[test]
    fn test_get_index_from_square_pos () {
        assert_eq! (GameMatrix::get_index_from_square_pos(1,1), 0);
        assert_eq! (GameMatrix::get_index_from_square_pos(2,1), 3);
        assert_eq! (GameMatrix::get_index_from_square_pos(9,9), 80);

        for line in (1..10) {
            for item in (1..10) {
                assert_eq! (GameMatrix::get_square_pos_from_index(GameMatrix::get_index_from_square_pos(line, item)), (line, item))
            }
        }
    }

}
