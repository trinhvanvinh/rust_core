// === clippy 1 ===
/*use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}*/

// === clippy2 ===
// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     /*for x in option {
//         res += x;
//     }*/
//     if let Some(x) = option{
//         res += x;
//     }
//     println!("{}", res);
// }

//=== clippy3===
/*use std::mem::swap;
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //if my_option.is_none() {
        //my_option.unwrap();
    //}

    let my_arr = &[-1, -2, -3,-4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let my_empty_vec: Vec<()> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    swap(&mut value_a,&mut value_b);
    println!("value a: {}; value b: {}",  value_a,     value_b);

}*/

// === conversions ===
// == as_ref_mut==

fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn num_sq<T: AsMut<usize>>(arg: &mut T) -> usize {
    arg.as_mut().pow(2)
}

fn main() {
    let s = "verdex ";
    //let s = String::from("verdex ");
    println!("count {:?}, {:?}", byte_counter(s), char_counter(s));

    let mut num = Box::new(4);
    println!("num_square {:?}", num_sq(&mut num));
}
