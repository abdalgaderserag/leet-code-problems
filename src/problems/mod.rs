pub mod check_if_binary_string_has_at_most_one_segment_of_ones;
pub mod latest_time_by_replacing_hidden_digits;
pub mod power_of_three;
pub mod remove_duplicates_from_sorted_array;
pub mod single_number;
pub mod two_numbers;

pub fn get_solver(index: usize) -> Option<fn()> {
    match index {
        1 => Some(two_numbers::solve),
        26 => Some(remove_duplicates_from_sorted_array::solve),
        136 => Some(single_number::solve),
        326 => Some(power_of_three::solve),
        1736 => Some(latest_time_by_replacing_hidden_digits::solve),
        1784 => Some(check_if_binary_string_has_at_most_one_segment_of_ones::solve),
        _ => None,
    }
}
