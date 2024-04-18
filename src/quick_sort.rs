pub fn quick_sort<T, F>(arr: &mut [T], comparator: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return;
    }

    let pivot = &arr[arr.len() / 2];

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();

    for item in arr.iter() {
        if comparator(item, pivot) {
            less.push(*item);
        } else if comparator(pivot, item) {
            greater.push(*item);
        } else {
            equal.push(*item);
        }
    }

    quick_sort(&mut less, comparator);
    quick_sort(&mut greater, comparator);

    arr[..less.len()].copy_from_slice(&less);
    arr[less.len()..less.len() + equal.len()].copy_from_slice(&equal);
    arr[less.len() + equal.len()..].copy_from_slice(&greater);
}
