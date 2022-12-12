mod gauss_rule;
mod linear_algebra;
fn main() {
    let points_of_integration: u8 = 8;

    let mut vector_of_integration_point: Vec<f64> = Vec::new();

    //Call functions for gauss rule numerical integration
    let new_gauss_coord = gauss_rule::gauss_rule_natural_coord(points_of_integration);
    let new_gauss_weight=gauss_rule::weight_for_gauss(points_of_integration);

    println!("gauss coordinate for {} gauss points = {:?}",points_of_integration, new_gauss_coord);
    println!("gauss weight for {} gauss points = {:?}",points_of_integration, new_gauss_weight);

    for vector in new_gauss_coord {
        vector_of_integration_point = vector;
    }
    println!(" vector of integration points = {:?}", vector_of_integration_point);
    
    //linear_algebra::matrix::matrix::matrix_2d();

    use linear_algebra::newmatrix;
    //let row:u32 = 2;
    //let m_1 = newmatrix::NewMatrix{};
    let mut m1 =newmatrix::NewMatrix{n_columns:2, n_rows:3, value:5.0};
    m1.matrix_2d();

}
