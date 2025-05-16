use std::cmp::Ordering;

pub fn find_magic_index(arr: &[i32]) -> Option<usize> {
    let mut low = 0isize;
    let mut high = arr.len() as isize - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let v = arr[mid as usize];
        match v.cmp(&(mid as i32)) {
            Ordering::Equal => return Some(mid as usize),
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }
    None
}
