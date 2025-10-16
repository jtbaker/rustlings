fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    input.iter().map(|element| { return element + 1 }).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Here, we also want to multiply each element in the `input` slice
    // by 2, but with iterator mapping instead of manually pushing into an empty
    // vector.
    // See the example in the function `vec_map_example` above.
    input
        .iter()
        .map(|element| {
            // ???
            element * 2
        })
        .collect()
}


pub fn square<T>(i: &[T]) -> Vec<T> 
where T: std::ops::Mul<Output = T> + Copy,
{
    i.iter().map(|v|  *v * *v).collect()
    // i.into_iter().map(|v| v * v).collect()
}

// pub fn square<T>(i: Vec<T>) -> Vec<T>
// where
//     T: std::ops::Mul<Output = T> + Copy,
// {
//     i.iter().map(|v| *v * *v).collect()
// }


fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![1.3, 4.8, 9.2];

    

    let int_squared = square(&v1);

    let float_squared = square(&v2);

    // let squared = square(m);

    println!("v1: {:?} and int_squared: {:?}", v1, int_squared);

    println!("float_squared: {:?}", float_squared);
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
