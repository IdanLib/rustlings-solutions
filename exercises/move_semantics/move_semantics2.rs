// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

fn main() {
    // let vec0 = Vec::new(); //1. immutable borrow solution
    let mut vec0 = Vec::new(); //2. mutable borrow solution

    // Do not move the following line!
    // let mut vec1 = fill_vec(&vec0); //1. immutable borrow solution

    fill_vec_mutable(&mut vec0);  //2. mutable borrow solution

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88); //1. immutable borrow solution

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); //1. immutable borrow solution
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec.to_vec(); //copying the data from the (immutable) borrow

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_mutable(vec: &mut Vec<i32>) -> () {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}