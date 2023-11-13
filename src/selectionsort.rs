use super::*;

struct SelectionSorter;

impl Sorter for SelectionSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let mut key = i;
            for j in i + 1..slice.len() {
                if slice[j] < slice[key] {
                    key = j;
                }
            }

            slice.swap(key, i);
        }
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    SelectionSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
