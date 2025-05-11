pub fn bubble_sort(arr: &mut [i32]) {
    for _ in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j + 1] < arr[j] {
                let temp = arr[j+1];
                arr[j + 1] = arr[j];
                arr[j] = temp;
            }
        }
    }
}
