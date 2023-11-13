use super::*;

struct BubbleSorter;

impl Sorter for BubbleSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            for j in (i + 1..slice.len()).rev() {
                if slice[j] < slice[j - 1] {
                    slice.swap(j, j - 1);
                }
            }
        }
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    BubbleSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
