mod gauss_rule;
fn main() {
    println!("Hello, world!");
    let s: u8 = 1;
    let g = gauss_rule::gauss_rule(s);
    println!("g = {:?}", g);
    println!("g = {:?}", g.get(&s));

    let mut vector_of_integration_point:Vec<f64> = Vec::new();

    for vector in g.get(&s) {
        vector_of_integration_point = vector.to_vec();
    }
    println!("vector of integration points = {:?}", vector_of_integration_point);
}
