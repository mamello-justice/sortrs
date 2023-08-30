mod sortrs;

use sortrs::{Display, Sort};

fn main() {
    let values = vec![3, 1, 5, 2, 4];
    println!("original\n{}\n", values.print());

    let mut v1 = values.to_owned();
    v1.bubble_sort();
    println!("bubble_sort\n{}\n", v1.print());

    let mut v2 = values.to_owned();
    v2.insertion_sort();
    println!("insertion_sort\n{}\n", v2.print());

    let mut v3 = values.to_owned();
    v3.merge_sort();
    println!("merge_sort\n{}\n", v3.print());

    let mut v4 = values.to_owned();
    v4.selection_sort();
    println!("selection_sort\n{}\n", v4.print());
}
