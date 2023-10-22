pub trait IntoArray {
    type Item;
    fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N];
}