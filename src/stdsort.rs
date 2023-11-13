use super::*;

struct StdSorter;

impl Sorter for StdSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    StdSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
