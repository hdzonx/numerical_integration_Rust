use std::collections::HashMap;
//Gauss rule for numerical integration
pub fn gauss_rule(points_of_integration: u8) -> HashMap<u8, Vec<f64>> {
    //Abscissas for the gauss rule
    let mut xi_i: Vec<f64> = Vec::new();
    let mut eta_i: Vec<f64> = Vec::new();
    let mut zeta_i: Vec<f64> = Vec::new();
    //natural coordinate for numerical integration

    let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut weight_for_gauss_points: HashMap<u8, Vec<f64>> = HashMap::new();

    //Fill the natural coordinates and weights for numerical integration in HashMaps
    if points_of_integration == 1 {
        natural_coordinate.insert(1, vec![0.0]);
        weight_for_gauss_points.insert(1, vec![2.0]);
    } else if points_of_integration == 2 {
        natural_coordinate.insert(2, vec![-0.5773502692, 0.5773502692]);
        weight_for_gauss_points.insert(2, vec![1.0, 1.0]);
    } else if points_of_integration == 3 {
        natural_coordinate.insert(3, vec![-0.774596697, 0.0, 0.774596697]);
        weight_for_gauss_points.insert(3, vec![0.5555555556, 0.8888888889, 0.5555555556]);
    } else if points_of_integration == 4 {
        natural_coordinate.insert(
            4,
            vec![-0.8611363116, -0.3399810436, 0.3399810436, 0.8611363116],
        );
        weight_for_gauss_points.insert(
            4,
            vec![0.3478548451, 0.6521451549, 0.6521451549, 0.3478548451],
        );
    }
    // else if points_of_integration ==5{
    //     natural_coordinate.insert(5, vec![-0.774596697, 0.0, 0.774596697]);
    //     weight_for_gauss_points.insert(5, vec![0.5555555556, 0.8888888889, 0.5555555556]);
    // }
    // else if points_of_integration ==6{
    //     natural_coordinate.insert(6, vec![-0.774596697, 0.0, 0.774596697]);
    //     weight_for_gauss_points.insert(6, vec![0.5555555556, 0.8888888889, 0.5555555556]);
    // }
    // else if points_of_integration ==7{
    //     natural_coordinate.insert(7, vec![-0.774596697, 0.0, 0.774596697]);
    //     weight_for_gauss_points.insert(7, vec![0.5555555556, 0.8888888889, 0.5555555556]);
    // }
    else {
        println!("deu pau")
    }
    let v = natural_coordinate.get(&points_of_integration);
    println!("v = {:?}", v);
    natural_coordinate
}

pub fn gauss_rule_with_option(points_of_integration: u8) -> Option<Vec<f64>> {
    let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut weight_for_gauss_points: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut vector_of_coords: Vec<f64> = Vec::new();

    match points_of_integration {
        //None => println!("Houve erro com {}",points_of_integration),
        1 => natural_coordinate.insert(1, vec![0.0]),
        2 => natural_coordinate.insert(2, vec![-0.5773502692, 0.5773502692]),
        0_u8 | 3_u8..=u8::MAX => todo!(),
    };
    for vector in natural_coordinate.get(&points_of_integration) {
        vector_of_coords = vector.to_vec();
    }
    Some(vector_of_coords)
}
