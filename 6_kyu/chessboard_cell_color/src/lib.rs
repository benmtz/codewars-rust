#[cfg(test)]
mod tests {

    // Task

    // Given two cells on the standard chess board, determine whether they have the same color or not.
    // Example

    // For cell1 = "A1" and cell2 = "C3", the output should be true.
    // For cell1 = "A1" and cell2 = "H3", the output should be false.

    // Input/Output

    // [input] string cell1
    // [input] string cell2
    // [output] a boolean value

    // true if both cells have the same color, false otherwise.


    fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
   
        let coords1 = get_cell_coords(cell1).unwrap_or((0,0));
        let coords2 = get_cell_coords(cell2).unwrap_or((0,0));

        (coords1.0 + coords1.1) % 2 == (coords2.0 + coords2.1) % 2
    }

    fn get_cell_coords(cell: &str) -> Option<(u32, u32)> {
        let x: u32;
        let y: u32;
        
        x = cell.chars().nth(0)?.to_digit(18)? - 10;
        y = cell.chars().nth(1)?.to_digit(10)? - 1;

        Some((x, y))
    }

    #[test]
    fn get_cell_coords_test() {
        assert_eq!(get_cell_coords("A1").unwrap(), (0,0));
        assert_eq!(get_cell_coords("C4").unwrap(), (2,3));
    }

    // https://www.codewars.com/kata/5894134c8afa3618c9000146/solutions/rust

    


    #[test]
    fn basic_tests() {
        assert_eq!(chessboard_cell_color("A1", "C3"), true);
        assert_eq!(chessboard_cell_color("A1", "H3"), false);
        assert_eq!(chessboard_cell_color("A1", "A2"), false);
        assert_eq!(chessboard_cell_color("A1", "C1"), true);
        assert_eq!(chessboard_cell_color("A1", "A1"), true);
    }
}
