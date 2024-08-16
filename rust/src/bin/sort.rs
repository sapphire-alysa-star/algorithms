
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    update_pointers(&nums1, &nums2, 0, 0);

    return 0.0;
}

fn update_pointers(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize) {
    let total_len = nums1.len() + nums2.len();
    let median_index = total_len / 2;

    let total_steps = i + j + 2;

    let distance = median_index - total_steps;
 
}

fn main() {
    println!("this is working");
}