use super::*;

struct QuickSorter;

fn partition<T: Ord + Copy>(v: &mut [T], l: usize, u: usize) -> usize {
    let pivot = v[u];

    let mut i = i32::try_from(l).unwrap() - 1;

    for j in l..u {
        if v[j] <= pivot {
            i += 1;
            v.swap(usize::try_from(i).unwrap(), j);
        }
    }

    i += 1;
    v.swap(usize::try_from(i).unwrap(), u);
    return usize::try_from(i).unwrap();
}

fn quicksort<T: Ord + Copy>(v: &mut [T], l: usize, u: usize) {
    if l >= u {
        return;
    }

    let m = partition(v, l, u);

    quicksort(v, l, m - 1);
    quicksort(v, m + 1, u);
}

impl Sorter for QuickSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy,
    {
        quicksort(slice, 0, slice.len() - 1);
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    QuickSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
