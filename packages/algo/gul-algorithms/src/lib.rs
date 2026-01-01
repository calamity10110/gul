pub mod sort {
    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot_idx = partition(arr);
        quick_sort(&mut arr[0..pivot_idx]);
        quick_sort(&mut arr[pivot_idx + 1..]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len - 1;
        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[pivot_index] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }
}

pub mod search {
    // Dijkstra could go here (stub)
    pub struct Graph;
}
