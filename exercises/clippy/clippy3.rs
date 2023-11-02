// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap_or(());
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    vec![1, 2, 3, 4, 5].resize(0, 5);
    // println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;

    // Clippy suggestion 1 - using swap()
    // std::mem::swap(&mut value_a, &mut value_b);
    // Clippy suggestion 2 - using replace()
    value_b = std::mem::replace(&mut value_a, value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}
