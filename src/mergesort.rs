use super::*;

struct MergeSorter;

fn merge<T>(slice: &mut [T], l: usize, m: usize, u: usize)
where
    T: Ord + Copy + Default + super::GetMax,
{
    let mut left: Vec<T> = vec![T::default(); m - l];
    left.clone_from_slice(&slice[l..m]);
    left.push(T::get_max());

    let mut right = vec![T::default(); u - m];
    right.clone_from_slice(&slice[m..u]);
    right.push(T::get_max());

    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in l..u {
        if left[i] < right[j] {
            slice[k] = left[i];
            i += 1;
        } else {
            slice[k] = right[j];
            j += 1;
        }
    }
}

fn merge_sort<T>(slice: &mut [T], l: usize, u: usize)
where
    T: Ord + Copy + Default + super::GetMax,
{
    if u - l < 2 {
        return;
    }
    let m = (l + u) / 2;
    merge_sort(slice, l, m);
    merge_sort(slice, m, u);
    merge(slice, l, m, u);
}

impl Sorter for MergeSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy + Default + super::GetMax,
    {
        merge_sort(slice, 0, slice.len());
    }
}

#[test]
fn it_sorts() {
    let mut values = vec![3, 1, 5, 2, 4];
    MergeSorter::sort(&mut values);
    assert_eq!(values, &[1, 2, 3, 4, 5]);
}
