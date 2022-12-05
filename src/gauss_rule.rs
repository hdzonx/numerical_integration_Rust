use std::collections::HashMap;
//Gauss rule for numerical integration
// pub fn gauss_rule(points_of_integration: u8) -> HashMap<u8, Vec<f64>> {
//     //Abscissas for the gauss rule
//     let mut xi_i: Vec<f64> = Vec::new();
//     let mut eta_i: Vec<f64> = Vec::new();
//     let mut zeta_i: Vec<f64> = Vec::new();
//     //natural coordinate for numerical integration

//     let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();
//     let mut weight_for_gauss_points: HashMap<u8, Vec<f64>> = HashMap::new();

//     //Fill the natural coordinates and weights for numerical integration in HashMaps
//     if points_of_integration == 1 {
//         natural_coordinate.insert(1, vec![0.0]);
//         weight_for_gauss_points.insert(1, vec![2.0]);
//     } else if points_of_integration == 2 {
//         natural_coordinate.insert(2, vec![-0.5773502692, 0.5773502692]);
//         weight_for_gauss_points.insert(2, vec![1.0, 1.0]);
//     } else if points_of_integration == 3 {
//         natural_coordinate.insert(3, vec![-0.774596697, 0.0, 0.774596697]);
//         weight_for_gauss_points.insert(3, vec![0.5555555556, 0.8888888889, 0.5555555556]);
//     } else if points_of_integration == 4 {
//         natural_coordinate.insert(
//             4,
//             vec![-0.8611363116, -0.3399810436, 0.3399810436, 0.8611363116],
//         );
//         weight_for_gauss_points.insert(
//             4,
//             vec![0.3478548451, 0.6521451549, 0.6521451549, 0.3478548451],
//         );
//     }
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
//     else {
//         println!("deu pau")
//     }
//     let v = natural_coordinate.get(&points_of_integration);
//     println!("v = {:?}", v);
//     natural_coordinate
// }

//Natural coordinate for numerical integration using Gauss Rule
pub fn gauss_rule_natural_coord(points_of_integration: u8) -> Option<Vec<f64>> {
    let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut vector_of_coords: Vec<f64> = Vec::new();

    match points_of_integration {
        1 => natural_coordinate.insert(1, vec![0.0]),
        2 => natural_coordinate.insert(2, vec![-0.5773502692, 0.5773502692]),
        3 => natural_coordinate.insert(3, vec![-0.774596697, 0.0, 0.774596697]),
        4 => natural_coordinate.insert(
            4,
            vec![-0.8611363116, -0.3399810436, 0.3399810436, 0.8611363116],
        ),
        5 => natural_coordinate.insert(
            5,
            vec![
                -0.9061798459,
                -0.5384693101,
                0.0,
                0.5384693101,
                0.9061798459,
            ],
        ),
        6 => natural_coordinate.insert(
            6,
            vec![
                -0.9324695142,
                -0.6612093865,
                -0.2386191861,
                0.2386191861,
                0.6612093865,
                0.9324695142,
            ],
        ),
        7 => natural_coordinate.insert(
            7,
            vec![
                -0.9491079123,
                -0.7415311856,
                -0.4058451514,
                0.0,
                0.4058451514,
                0.7415311856,
                0.9491079123,
            ],
        ),
        8 => natural_coordinate.insert(
            8,
            vec![
                -0.9602898565,
                -0.7966664774,
                -0.5255324099,
                -0.1834346425,
                0.1834346425,
                0.5255324099,
                0.7966664774,
                0.9602898565,
            ],
        ),
        0_u8 | 9_u8..=u8::MAX => unimplemented!(),
    };
    for vector in natural_coordinate.get(&points_of_integration) {
        vector_of_coords = vector.to_vec();
    }
    Some(vector_of_coords)
}

//Weight for numerical integration using Gauss Rule
pub fn weight_for_gauss(points_of_integration: u8) -> Option<Vec<f64>> {
    let mut weight: HashMap<u8, Vec<f64>> = HashMap::new();
    let mut vector_of_weights: Vec<f64> = Vec::new();
    match points_of_integration {
        1 => weight.insert(1, vec![2.0]),
        2 => weight.insert(2, vec![1.0, 1.0]),
        3 => weight.insert(3, vec![0.5555555556, 0.8888888889, 0.5555555556]),
        4 => weight.insert(
            4,
            vec![0.3478548451, 0.6521451549, 0.6521451549, 0.3478548451],
        ),
        5 => weight.insert(
            5,
            vec![
                0.2369268851,
                0.4786286705,
                0.5688888889,
                0.4786286705,
                0.2369268851,
            ],
        ),
        6 => weight.insert(
            6,
            vec![
                0.1713244924,
                0.3607615730,
                0.4679139346,
                0.3607615730,
                0.1713244924,
            ],
        ),
        7 => weight.insert(
            7,
            vec![
                0.1294849662,
                0.2797053915,
                0.3818300505,
                0.4179591837,
                0.3818300505,
                0.2797053915,
                0.1294849662,
            ],
        ),
        8 => weight.insert(
            8,
            vec![
                0.1012285363,
                0.2223810345,
                0.3137066459,
                0.3626837834,
                0.3626837834,
                0.3137066459,
                0.2223810345,
                0.1012285363,
            ],
        ),
        0_u8 | 9_u8..=u8::MAX => unimplemented!(),
    };

    for vector in weight.get(&points_of_integration) {
        vector_of_weights = vector.to_vec();
    }
    Some(vector_of_weights)
}
