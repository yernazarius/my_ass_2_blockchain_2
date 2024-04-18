pub fn selection_sort<T, F>(arr: &mut [T], comparator: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if comparator(&arr[j], &arr[min_index]) {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}