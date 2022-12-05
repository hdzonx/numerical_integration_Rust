mod gauss_rule;
fn main() {
    let points_of_integration: u8 = 9;

    let mut vector_of_integration_point: Vec<f64> = Vec::new();

    //Call functions for gauss rule numerical integration
    let new_gauss_coord = gauss_rule::gauss_rule_natural_coord(points_of_integration);
    let new_gauss_weight=gauss_rule::weight_for_gauss(points_of_integration);

    println!("gauss coordinate for {} gauss points = {:?}",points_of_integration, new_gauss_coord);
    println!("gauss weight for {} gauss points = {:?}",points_of_integration, new_gauss_weight);

    for vector in new_gauss_coord {
        vector_of_integration_point = vector;
    }
    println!(" vector of integration points = {:?}", vector_of_integration_point)
}
