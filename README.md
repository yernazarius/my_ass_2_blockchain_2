Sorting Algos Lib in Rust:

Usage: <br>
Add dependency into your project<br>
		```
		[dependencies]
		sorting_algos = { git = "" }
  		```
<br>
open any Rust file and type<br><br>
	```use sorting_algos::quick_sort::quick_sort;```<br><br>
or use merging sort, selection sort or insertion sort



Example: <br>
    ```
    let mut arr = [10, 7, 8, 9, 1, 5]; 
    quick_sort(&mut arr, &|a, b| a < b);
    println!("Sorted array: {:?}", arr);
    ```
