#[derive(Debug)]
pub struct Matrix {
    data: Vec<u32>,
    rows: usize,
    cols: usize,
}

pub struct MatrixIterator<'a> {
    matrix: &'a Matrix,
    row: usize,
    col: usize,
}

impl<'a> MatrixIterator<'a> {
    pub fn new(matrix: &'a Matrix) -> MatrixIterator {
        MatrixIterator {
            matrix: matrix,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for MatrixIterator<'a> {
    type Item = (u32, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.matrix.cols {
            self.col = 0;
            self.row += 1;
        }
        if self.row == self.matrix.rows {
            None
        } else {
            let row = self.row;
            let col = self.col;
            let val = self.matrix.get(row, col);

            self.col += 1;
            Some((val, row, col))
        }
    }
}

impl Matrix {
    pub fn new(data: Vec<u32>, rows: usize, cols: usize) -> Matrix {
        assert_eq!(data.len(), rows * cols);
        Matrix { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> u32 {
        let index: usize = col + self.cols * row;
        self.data[index]
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                str += &self.get(row, col).to_string();
            }
            str += "\n";
        }
        str
    }

    pub fn iter(&self) -> MatrixIterator {
        MatrixIterator::new(self)
    }

    pub fn is_visible(&self, row: usize, col: usize) -> bool {
        self.is_visible_from_bottom(row, col)
            || self.is_visible_from_left(row, col)
            || self.is_visible_from_right(row, col)
            || self.is_visible_from_top(row, col)
    }

    pub fn is_visible_from_left(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);
        for test_col in 0..col {
            if self.get(row, test_col) >= val {
                return false;
            }
        }
        true
    }
    pub fn is_visible_from_right(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);
        for test_col in col + 1..self.cols {
            if self.get(row, test_col) >= val {
                return false;
            }
        }
        true
    }
    pub fn is_visible_from_top(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);
        for test_row in 0..row {
            if self.get(test_row, col) >= val {
                return false;
            }
        }
        true
    }
    pub fn is_visible_from_bottom(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);
        for test_row in row + 1..self.rows {
            if self.get(test_row, col) >= val {
                return false;
            }
        }
        true
    }
    pub fn is_border(&self, row: usize, col: usize) -> bool {
        col == 0 || col == self.cols - 1 || row == 0 || row == self.rows - 1
    }
    pub fn scenic_score(&self, row: usize, col: usize) -> u32 {
        if self.is_border(row, col) {
            return 0;
        }
        self.scenic_score_left(row, col)
            * self.scenic_score_right(row, col)
            * self.scenic_score_bottom(row, col)
            * self.scenic_score_top(row, col)
    }
    pub fn scenic_score_left(&self, row: usize, col: usize) -> u32 {
        if col == 0 {
            return 0;
        }
        let val = self.get(row, col);
        let mut score = 0;
        for test_col in (0..col).rev() {
            score += 1;
            if self.get(row, test_col) >= val {
                return score;
            }
        }
        score
    }
    pub fn scenic_score_right(&self, row: usize, col: usize) -> u32 {
        if col == self.cols - 1 {
            return 0;
        }
        let val = self.get(row, col);
        let mut score = 0;
        for test_col in col + 1..self.cols {
            score += 1;
            if self.get(row, test_col) >= val {
                return score;
            }
        }
        score
    }
    pub fn scenic_score_top(&self, row: usize, col: usize) -> u32 {
        if row == 0 {
            return 0;
        }
        let val = self.get(row, col);
        let mut score = 0;
        for test_row in (0..row).rev() {
            score += 1;
            if self.get(test_row, col) >= val {
                return score;
            }
        }
        score
    }
    pub fn scenic_score_bottom(&self, row: usize, col: usize) -> u32 {
        if row == self.rows - 1 {
            return 0;
        }
        let val = self.get(row, col);
        let mut score = 0;
        for test_row in row + 1..self.rows {
            score += 1;
            if self.get(test_row, col) >= val {
                return score;
            }
        }
        score
    }
}

pub fn parse(str: &str) -> Matrix {
    let cols = str.lines().next().unwrap().len();
    let rows = str.lines().count();
    let mut data: Vec<u32> = Vec::new();
    data.reserve(cols * rows);
    for line in str.lines() {
        for char in line.chars() {
            if char.is_numeric() {
                data.push(char.to_digit(10).unwrap());
            }
        }
    }
    Matrix::new(data, rows, cols)
}
