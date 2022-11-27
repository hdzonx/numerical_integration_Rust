use std::collections::HashMap;
//Gauss rule for numerical integration
pub fn gauss_rule(points_of_integration: u8) -> HashMap<u8, Vec<f64>> {
    //Abscissas for the gauss rule
    let mut xi_i: Vec<f64> = Vec::new();
    let mut eta_i: Vec<f64> = Vec::new();
    let mut zeta_i: Vec<f64> = Vec::new();
    //natural coordinate for numerical integration

    let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();

    if points_of_integration == 1 {
        natural_coordinate.insert(1, vec![0.0]);
    } else {
        println!("deu pau")
    }
    let v = natural_coordinate.get(&points_of_integration);
    println!("v = {:?}", v);
    natural_coordinate
    //match points_of_integration {
    // 1 => natural_coordinate.insert(1, vec![0.0]),//k:points of integration, v:coordniates
    // 2 => natural_coordinate.insert(2, vec![0.0]),
    // 3 => natural_coordinate.insert(3, vec![0.0]),
    // 4 => natural_coordinate.insert(4, vec![0.0]),
    //  5 => natural_coordinate.insert(5, vec![0.0]),
    //  6 => natural_coordinate.insert(6, vec![0.0]),
    //  7 => natural_coordinate.insert(7, vec![0.0]),
    //  8 => natural_coordinate.insert(8, vec![0.0]),
    // 0_u8 | 9_u8..=u8::MAX => todo!(),

    //}
    //println!("NV: {:?}", &natural_coordinate);
} //

fn coordinate(points_of_integration: u8) -> (){
    let mut natural_coordinate: HashMap<u8, Vec<f64>> = HashMap::new();
    if points_of_integration == 1 {
        natural_coordinate.insert(1, vec![0.0]);
    } else {
        println!("deu pau")
    }

    let v = natural_coordinate.get(&points_of_integration);
    //v
}