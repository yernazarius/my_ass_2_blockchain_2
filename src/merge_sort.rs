pub fn merge_sort<T, F>(arr: &mut [T], comparator: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left, comparator);
    merge_sort(right, comparator);

    let mut merged = Vec::with_capacity(len);
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if comparator(&left[i], &right[j]) {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    arr.copy_from_slice(&merged);
}
