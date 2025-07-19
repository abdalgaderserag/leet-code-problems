pub mod two_numbers;
pub mod remove_duplicates_from_sorted_array;
pub mod single_number;

pub fn get_solver(index: usize) -> Option<fn()> {
    match index {
        1 => Some(two_numbers::solve),
        26 => Some(remove_duplicates_from_sorted_array::solve),
        136 => Some(single_number::solve),
        _ => None,
    }
}
