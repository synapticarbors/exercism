pub fn find<S: AsRef<[T]>, T: Ord>(array: S, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }

    if key < array[0] || key > array[array.len() - 1] {
        return None;
    }

    let mut left = 0;
    let mut right = array.len().checked_sub(1)?;

    while left <= right {
        let m = (left + right) / 2;

        if array[m] < key {
            left = m + 1;
        } else if array[m] > key {
            right = m - 1;
        } else {
            return Some(m);
        }
    }

    None
}
