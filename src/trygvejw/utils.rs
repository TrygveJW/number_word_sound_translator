pub mod vector_utils {
    use std::borrow::Borrow;

    pub fn index_of_first_match<T, Fun>(predicate: Fun, vector: &Vec<T>) -> usize
    where
        Fun: Fn(&T) -> bool,
    {
        let mut ret: usize = 0;
        for (i, val) in vector.iter().enumerate() {
            if predicate(val) {
                ret = i;
                break;
            }
        }
        ret
    }
    pub fn index_of_last_match<T, Fun>(predicate: Fun, vector: &Vec<T>) -> usize
    where
        Fun: Fn(&T) -> bool,
    {
        let mut ret: usize = 0;
        for (i, val) in vector.iter().rev().enumerate() {
            if predicate(val) {
                ret = i;
                break;
            }
        }
        ret
    }
}
