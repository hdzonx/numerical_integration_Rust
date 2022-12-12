
pub mod matrix {

    pub fn arra() {
        let arr = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14, 15, 16, 17, 18],
            [19, 20, 21, 22, 23, 24, 25, 26, 27],
            [28, 29, 30, 31, 32, 33, 34, 35, 36],
            [37, 38, 39, 40, 41, 42, 43, 44, 45],
            [46, 47, 48, 49, 50, 51, 52, 53, 54],
            [55, 56, 57, 58, 59, 60, 61, 62, 63],
            [64, 65, 66, 67, 68, 69, 70, 71, 72],
            [73, 74, 75, 76, 77, 78, 79, 80, 81],
        ];
        let row = arr[3];
        println!("row: {:?}", row);

        //get a 4x4 slice starting at 2,2
        let (x, y, z) = (2, 2, 4);
        let slice = arr
            .iter()
            .skip(x)
            .take(z)
            .map(|s| s.iter().skip(y).take(z).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        println!("slice: {:?}", slice);

        //get a 5x5 slice starting at 1,1 and flatten into 1d vector
        let (x, y, z) = (1, 1, 5);
        let slice = arr
            .iter()
            .skip(x)
            .take(z)
            .map(|s| s.iter().skip(y).take(z).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        println!("flat slice: {:?}", slice);
    }

    pub fn matrix_2d() {
        use ndarray::Array2;
        //using the crate ndarray - creat a bidimensional array
        let mut temperature = Array2::<f64>::zeros((3, 4));
        temperature[[2, 2]] += 0.5;
        println!("teperanture = {}", temperature);
    }
}
