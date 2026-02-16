#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn contains(haystack: &[i32], needle: &[i32]) -> bool {
    for i in 0..haystack.len() {
        if i + needle.len() > haystack.len() {
            return false;
        }
        let slice = &haystack[i..i + needle.len()];
        if slice == needle {
            return true;
        }
    }
    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if contains(second_list, first_list) {
        return Comparison::Sublist;
    }
    if contains(first_list, second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
