use std::vec;

fn main() {
    println!("Hello, world!");

    // // Variables 
    // let x: i16 = -12;
    // println!("this is a signed variable, we have i8, i16, i32, i64, i128 {}", x);

    // let arr  = [12, 10, 22];
    // println!("this is an array {}", arr[0]);

    // let tuple: (u8, bool, i8)  = (9, true, -4);
    // println!("{:?}", tuple);

    // let vec = vec![32, 3];


    // // Vectors
    // println!("this is a vector, {:?}", vec);

    // let mut vect = Vec::new();
    // vect.push(20);
    // println!("{:?}", vect);

    // let mut vect2 = Vec::<i128>::with_capacity(2);


    // Slices
    let v:Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv:&[i32] = &v[2..4];
    println!("{:?}", sv);



}


