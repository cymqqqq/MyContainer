use std::cmp::Eq;
use std::fmt;
use std::iter::StepBy;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;
use std::slice::IterMut;

pub struct Matrix<T> {
    val: Vec<T>,
    col: usize,
    row: usize,
}
impl<T: Clone> Matrix<T> {
    pub fn new(row: usize, col: usize) -> Self 
    where
        T: Default,
    {
        if row < 1 || col < 1 {
            panic!("matrix size of row and col must be larger than zero");
        }
        Self {
            val: vec![T::default(); row * col],
            col,
            row,
        }
    }
    pub fn init(row: usize, col: usize, val: T) -> Self {
        if row < 1 || col < 1 {
            panic!("the dimension of matrix not valid");
        }
        Self {
            val: vec![val; row * col],
            col,
            row,
        }
    }
    pub fn from_vec(vec: Vec<T>, col: usize) -> Self {
        let row = vec.len();
        if row == 0 {
            if col == 0 {
                Self {
                    val: vec![],
                    row:0,
                    col: 0,
                }
            } else {
                panic!("vector len is zero, col is {:?}", col);
            }
        } else if row % col != 0 {
            panic!("vector len must be a multiple of col");
        } else {
            Self {
                val: vec,
                row: row / col,
                col,
            }
        }
            
    }
    #[inline]
    pub unsafe fn unchecked(&self, row: usize, col: usize) -> &T {
        self.val.get_unchecked(row * self.col * col)
    }
    #[inline]
    pub unsafe fn unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        let cols = self.col;
        self.val.get_unchecked_mut(row * cols + col)
    }
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.row && col < self.col {
            unsafe { Some(self.unchecked(row, col)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.row && col < self.col {
            unsafe { Some(self.unchecked_mut(row, col)) }
        } else {
            None
        }
    }
    pub fn iter(&self) -> Iter<T> {
        self.val.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.val.iter_mut()
    }
    pub fn iter_col(&self, col: usize) -> StepBy<Iter<T>> {
        if col < self.col {
            return self.val[col..].iter().step_by(self.col);
        } else {
            panic!("out of bound, col must be less than {:?}, but is {:?}",
            self.col, col)
        }
    }
    pub fn iter_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        let cols = self.col;
        if col < cols {
            return self.val[col..].iter_mut().step_by(cols);
        } else {
            panic!("out of bound, col must be less than {:?}, but is {:?}",
            self.col, col)
        }
    }
    pub fn iter_row(&self, row: usize) -> Iter<T> {
        if row < self.row {
            let start = row * self.col;
            self.val[start..(start + self.col)].iter()
        } else {
            panic!("out of bound, row must be less than {:?}, but is {:?}",
            self.row, row)
        }
    }
    pub fn iter_row_mut(&mut self, row: usize) -> IterMut<T> {
        if row < self.row {
            let cols = self.col;
            let start = row * cols;
            self.val[start..(start + cols)].iter_mut()
        } else {
            panic!("out of bound, row must be less than {:?}, but is {:?}",
            self.row, row)
        }
    }
    pub fn push_row(&mut self, row: Vec<T>) {
        let inp_row_len = row.len();
        if self.row > 0 && inp_row_len != self.col {
            panic!(" the dim of pushed row not valid, len must be {:?}, but is {:?}",
            self.col, inp_row_len)
        }
        self.val.extend(row);
        self.row += 1;
        self.col = inp_row_len;
    }
    pub fn push_col(&mut self, col: Vec<T>) {
        let inp_col_len = col.len();
        if self.col > 0 && inp_col_len != self.row {
            panic!("the dim of pushed col not valid, len must be {:?},
            but is {:?}",
            self.row, inp_col_len)
        }
        self.val.reserve(col.len());
        for (idx, i) in col.iter().enumerate() {
            let vec_idx = (idx + 1) * self.col + idx;
            self.val.insert(vec_idx, i.to_owned());
        }
        self.col += 1;
        self.row = inp_col_len;
    }
    pub fn pop_row(&mut self) -> Option<Vec<T>> {
        if self.row > 0 {
            let row = self.val.split_off((self.row - 1) * self.col);
            self.row -= 1;
            if self.row == 0 { self.col = 0; }
            Some(row)
        } else { None }
    }
    pub fn pop_col(&mut self) -> Option<Vec<T>> {
        if self.col > 0 {
            let mut col = Vec::with_capacity(self.row);
            for i in 0..self.row {
                let idx = i * self.col + self.col - 1 - i;
                col.push(self.val.remove(idx));
            }
            self.col -= 1;
            if self.col == 0 { self.row = 0; }
            Some(col)
        } else { None }
    }
    pub fn insert_row(&mut self, idx: usize, row: Vec<T>) {
        if row.len() != self.col {
            panic!("len of inserted row must be {:?},
            but is {:?}",self.col, row.len());
        }
        if idx > self.row {
            panic!("out of bound, idx is {:?}, but must be {:?}", idx,self.col);
        }
        self.row += 1;
        let val_idx = idx * self.col;
        self.val.splice(val_idx..val_idx, row.iter().cloned());
    }
    pub fn insert_col(&mut self, idx: usize, col: Vec<T>) {
        if col.len() != self.row {
            panic!("len of inserted row must be {:?},
            but is {:?}",self.row, col.len());
        }
        if idx > self.col {
            panic!("out of bound, idx is {:?}, but must be {:?}", idx,self.row);

        }
        for (row_idx, col_val) in col.iter().enumerate() {
            let val_idx = row_idx * self.col + idx + row_idx;
            self.val.insert(val_idx, col_val.clone());
        }
        self.col += 1;
    }
    pub fn flatten(&self) -> &Vec<T> {
        &self.val
    }
    pub fn into_vec(self) -> Vec<T> {
        self.val
    }
    pub fn transpose(&self) -> Self {
        let mut val = Vec::with_capacity(self.val.len());
        for c in 0..self.col {
            for r in 0..self.row {
                val.push(self[r][c].clone());
            }
        }
        Self {
            val,
            col: self.row,
            row: self.col,
        }
    }
}
impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            val: self.val.clone(),
        }
    }
}
impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        if idx < self.row {
            let start_idx = idx * self.col;
            &self.val[start_idx..start_idx + self.col]
        } else {
            panic!("idx {:?} out of bound, Matrix has {:?} row",
            self.row, idx);
        }
    }
}
impl<T: Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.val[(idx + self.col)..]
    }
}
impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    #[warn(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[");
        if self.col > 0 {
            for (idx, _) in self.val.iter().enumerate().step_by(self.col) {
                write!(f, "{:?}", &self.val[idx..(idx + self.col)]);
            }
        }
        write!(f, "]")
    }
}
impl<T: Eq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col &&
        self.val == other.val
    }
}
impl<T: Eq> Eq for Matrix<T> {}
