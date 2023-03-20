#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if assert_eq!(_first_list, _second_list) {
        return Comparison::Equal
    }

    
   // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
fn main() {
    sublist(&[1,2,3], &[1,2,3]);
}