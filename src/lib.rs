pub trait GetMax {
    fn get_max() -> Self;
}

impl GetMax for i32 {
    fn get_max() -> i32 {
        std::i32::MAX
    }
}

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy + Default + GetMax;
}

mod bubblesort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod selectionsort;
mod stdsort;
