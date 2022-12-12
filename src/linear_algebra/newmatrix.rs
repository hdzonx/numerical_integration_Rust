//mod newmatrix {}
use ndarray::Array2;
pub struct NewMatrix {
    pub n_rows: usize,
    pub n_columns: usize,
    pub value: f64,
}

impl NewMatrix {
    pub fn matrix_2d(&self) ->Vec<Vec<f64>>{
        //let vx = vec![0;3];
        //Matrix of n rows and n colums
        let v1:Vec<Vec<f64>> = vec![vec![self.value;self.n_rows];self.n_columns];
        //let arr =[[1,1],[1,1]];
        //matrix of n rows and n columns from ndArray crate
        //let mut matrix_zeros = Array2::<f64>::zeros((self.n_rows, self.n_columns));
        //matrix_zeros[[2, 2]] += self.value;
        //println!("teperanture = {}", matrix_zeros);
        println!("v1 = {:?}", v1);
        return v1;
    }
}
