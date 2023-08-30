pub trait GetMax {
    fn get_max() -> Self;
}

pub trait Display<T> {
    fn print(&self) -> String;
}

pub trait VecItem: Ord + Copy + Default + std::fmt::Display + GetMax {}

pub trait Sort<T>
where
    T: VecItem,
{
    fn bubble_sort(&mut self);
    fn insertion_sort(&mut self);
    fn merge_sort(&mut self);
    fn selection_sort(&mut self);
}

fn merge<T: VecItem>(v: &mut Vec<T>, l: usize, m: usize, u: usize) {
    let mut left: Vec<T> = vec![T::default(); m - l];
    left.clone_from_slice(&v[l..m]);
    left.push(T::get_max());

    let mut right = vec![T::default(); u - m];
    right.clone_from_slice(&v[m..u]);
    right.push(T::get_max());

    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in l..u {
        if left[i] < right[j] {
            v[k] = left[i];
            i += 1;
        } else {
            v[k] = right[j];
            j += 1;
        }
    }
}

fn merge_sort<T: VecItem>(v: &mut Vec<T>, l: usize, u: usize) {
    if u - l < 2 {
        return;
    }
    let m = (l + u) / 2;
    merge_sort(v, l, m);
    merge_sort(v, m, u);
    merge(v, l, m, u);
}

impl GetMax for i32 {
    fn get_max() -> i32 {
        std::i32::MAX
    }
}

impl<T> Display<T> for Vec<T>
where
    T: std::fmt::Display,
{
    fn print(&self) -> String {
        let mut val = "[ ".to_owned();
        for item in self.iter() {
            val = val + &(item.to_string()) + " ";
        }
        val.push_str("]");
        val.to_string()
    }
}
impl VecItem for i32 {}

impl<T> Sort<T> for Vec<T>
where
    T: VecItem,
{
    fn bubble_sort(&mut self) {
        for i in 0..self.len() {
            for j in (i + 1..self.len()).rev() {
                if self[j] < self[j - 1] {
                    self.swap(j, j - 1);
                }
            }
        }
    }

    fn insertion_sort(&mut self) {
        for j in 1..self.len() {
            let key = self[j].to_owned();

            let mut i = j;
            while i >= 1 && self[i - 1] > key {
                self[i] = self[i - 1];
                i -= 1;
            }
            self[i] = key;
        }
    }

    fn merge_sort(&mut self) {
        merge_sort(self, 0, self.len());
    }

    fn selection_sort(&mut self) {
        for i in 0..self.len() {
            let mut key = i;
            for j in i + 1..self.len() {
                if self[j] < self[key] {
                    key = j;
                }
            }

            self.swap(key, i);
        }
    }
}
