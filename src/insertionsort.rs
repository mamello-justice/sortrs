use super::*;

struct InsertionSorter;

impl Sorter for InsertionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy,
    {
        for j in 1..slice.len() {
            let key = slice[j].to_owned();

            let mut i = j;
            while i >= 1 && slice[i - 1] > key {
                slice[i] = slice[i - 1];
                i -= 1;
            }
            slice[i] = key;
        }
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    InsertionSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
