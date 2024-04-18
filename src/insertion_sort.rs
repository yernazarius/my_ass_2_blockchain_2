pub fn insertion_sort<T, F>(arr: &mut [T], comparator: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && !comparator(&arr[j - 1], &arr[j]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}