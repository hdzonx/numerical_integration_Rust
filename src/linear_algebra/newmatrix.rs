//mod newmatrix {}
//use ndarray::Array2;
pub struct NewMatrix {
    pub n_rows: usize,
    pub n_columns: usize,
}

impl NewMatrix {
    pub fn matrix_2d(&self) -> Vec<Vec<f64>> {
        //let vx = vec![0;3];
        //Matrix of n rows and n colums
        let matrix: Vec<Vec<f64>> = vec![vec![0.0; self.n_rows]; self.n_columns];
        //let arr =[[1,1],[1,1]];
        //matrix of n rows and n columns from ndArray crate
        //let mut matrix_zeros = Array2::<f64>::zeros((self.n_rows, self.n_columns));
        //matrix_zeros[[2, 2]] += self.value;
        //println!("teperanture = {}", matrix_zeros);
        println!("v1 = {:?}", matrix);
        return matrix;
    }
    pub fn set_value(&self,matrix: Vec<Vec<f64>>, row_i: usize, column_j: usize, value: f64)-> Vec<Vec<f64>> {
        //let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; self.n_rows]; self.n_columns];
        //vec.remove(*row_i);
        // vec.insert(*row_i, value);
        let mut new_matrix = matrix.clone();
        new_matrix[row_i][column_j] = value;
        // for i in 0..self.n_rows {
        //     for j in 0..self.n_columns {
        //         if i == row_i && j == column_j {
        //             new_matrix[i][j] = value;
        //         }
        //     }
        // }
        println!("matrix = {:?}", new_matrix);
        new_matrix
    }
}
