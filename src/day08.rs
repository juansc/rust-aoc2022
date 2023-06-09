use crate::solver::Solver;
use crate::utils::grid::{Grid, GridCoord};

pub struct Day8Solver {}

struct Forest {
    num_visible: usize,
    trees: Grid<u8>,
    tree_is_visible: Grid<bool>,
    max_tree_visibility: Grid<Option<MaxTreeView>>,
}

#[derive(Copy, Clone)]
struct MaxTreeView {
    left_to_right: MaxTreeViewInDirection,
    right_to_left: MaxTreeViewInDirection,
    top_to_bottom: MaxTreeViewInDirection,
    bottom_to_top: MaxTreeViewInDirection,

}

#[derive(Copy, Clone, Default)]
struct MaxTreeViewInDirection {
    distance: usize,
}

enum ForestDirection {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

impl Forest {
    fn new(lines: Vec<String>) -> Self {
        let grid_height = lines.len();
        let grid_width = lines[0].len();
        let mut trees = Grid::new(grid_width, grid_height);
        for (row, line) in lines.iter().enumerate() {
            for (col, height) in line.chars().enumerate() {
                let tree_height: &mut u8 = trees.cell_mut((row as isize, col as isize).into()).unwrap();
                *tree_height = height.to_digit(10).unwrap() as u8;
            }
        }
        Self {
            num_visible: 0,
            trees,
            tree_is_visible: Grid::new(grid_height, grid_height),
            max_tree_visibility: Grid::new(grid_height, grid_height),
        }
    }

    /* Function used for debugging
    fn print_visible(&self) {
        for row in 0..self.trees.height() {
            let mut str: String = "".to_string();
            for col in 0..self.trees.width() {
                if *self.tree_is_visible.cell((row as isize, col as isize).into()).unwrap() {
                    str += "X"
                } else {
                    str += "0"
                }
            }
            println!("{}", str)
        }
    }
     */

    fn update_max_tree_visibility(&mut self) {
        for idx in self.trees.grid_coordinates() {
            let max_tree_view = MaxTreeView {
                left_to_right: self.calculate_tree_visibility(idx, ForestDirection::LeftToRight),
                right_to_left: self.calculate_tree_visibility(idx, ForestDirection::RightToLeft),
                top_to_bottom: self.calculate_tree_visibility(idx, ForestDirection::TopToBottom),
                bottom_to_top: self.calculate_tree_visibility(idx, ForestDirection::BottomToTop),
            };
            let view = self.max_tree_visibility.cell_mut(idx).unwrap();
            *view = Some(max_tree_view);
        }
    }

    fn calculate_tree_visibility(&self, coord: GridCoord, direction: ForestDirection) -> MaxTreeViewInDirection {
        let movement: GridCoord = match direction {
            ForestDirection::TopToBottom => { (0, 1).into() }
            ForestDirection::BottomToTop => { (0, -1).into() }
            ForestDirection::LeftToRight => { (1, 0).into() }
            ForestDirection::RightToLeft => { (-1, 0).into() }
        };
        let mut new_coord = coord + movement;
        let current_tree_height = *self.trees.cell(coord).unwrap();
        let mut dist = 0;
        while self.trees.in_bounds(new_coord) {
            dist += 1;
            let next_tree_height = *self.trees.cell(new_coord).unwrap();
            if next_tree_height >= current_tree_height {
                return MaxTreeViewInDirection { distance: dist };
            }
            new_coord = new_coord + movement;
        }
        MaxTreeViewInDirection { distance: dist }
    }

    fn tree_coordinates(&self, direction: ForestDirection, col_or_row: usize) -> Vec<GridCoord> {
        match direction {
            ForestDirection::TopToBottom | ForestDirection::BottomToTop => {
                let col = col_or_row;
                let mut indexes = vec![];
                for row in 0..self.trees.height() {
                    indexes.push((row as isize, col as isize).into())
                }
                if let ForestDirection::BottomToTop = direction {
                    indexes.reverse()
                }
                indexes
            }
            ForestDirection::LeftToRight | ForestDirection::RightToLeft => {
                let row = col_or_row;
                let mut indexes = vec![];
                for col in 0..self.trees.width() {
                    indexes.push((row as isize, col as isize).into())
                }
                if let ForestDirection::RightToLeft = direction {
                    indexes.reverse()
                }
                indexes
            }
        }
    }

    fn update_tree_visibility(&mut self) {
        for row in 0..self.trees.height() {
            self.update_line(row, ForestDirection::LeftToRight);
            self.update_line(row, ForestDirection::RightToLeft);
        }
        for col in 0..self.trees.width() {
            self.update_line(col, ForestDirection::TopToBottom);
            self.update_line(col, ForestDirection::BottomToTop);
        }
    }

    fn update_line(&mut self, col_or_row: usize, direction: ForestDirection) {
        let mut last_tallest_height = -1;
        for idx in self.tree_coordinates(direction, col_or_row) {
            let tree_height = *self.trees.cell(idx).unwrap();
            let tree_visibility = self.tree_is_visible.cell_mut(idx).unwrap();
            // This tree is visible.
            if tree_height as i16 > last_tallest_height {
                // If the tree wasn't marked as visible before, increase the amount
                if !*tree_visibility {
                    self.num_visible += 1;
                }
                // Mark as visible.
                *tree_visibility = true;
                last_tallest_height = tree_height as i16;
            }
        }
    }
}

impl Solver for Day8Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut forest = Forest::new(lines);
        forest.update_tree_visibility();
        let num_visible = forest.tree_is_visible.grid_coordinates().iter().filter(|&c| *forest.tree_is_visible.cell(*c).unwrap()).count();
        format!("{}", num_visible)
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut forest = Forest::new(lines);
        forest.update_max_tree_visibility();
        let mut max_visibility = 0;
        for idx in forest.max_tree_visibility.grid_coordinates() {
            // Grid requires that we always unwrap, since the entry may not exist (because it's out
            // of bounds). We unwrap one more time because tree visibility may not have been
            // calculated, so it is itself an option.
            let vis = forest.max_tree_visibility.cell(idx).unwrap().unwrap();
            let score = vis.top_to_bottom.distance *
                vis.bottom_to_top.distance *
                vis.left_to_right.distance *
                vis.right_to_left.distance;
            if score > max_visibility {
                max_visibility = score;
            }
        }
        format!("{}", max_visibility)
    }
}

#[cfg(test)]
mod test {
    use crate::day08::Day8Solver;
    use crate::lines_from_file;
    use crate::solver::Solver;

    #[test]
    fn test_part_1() {
        let solver = Day8Solver {};
        let lines = lines_from_file("./inputs/unit_test/day08.txt");
        assert_eq!(solver.solve_part_1(lines), "21")
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day8Solver {};
        let lines = lines_from_file("./inputs/day08.txt");
        assert_eq!(solver.solve_part_1(lines), "1662")
    }

    #[test]
    fn test_part_2() {
        let solver = Day8Solver {};
        let lines = lines_from_file("./inputs/unit_test/day08.txt");
        assert_eq!(solver.solve_part_2(lines), "8")
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day8Solver {};
        let lines = lines_from_file("./inputs/day08.txt");
        assert_eq!(solver.solve_part_2(lines), "537600")
    }
}



