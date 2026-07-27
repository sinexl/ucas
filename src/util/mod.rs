pub mod macros;

pub fn is_sorted<T>(arr: &[T]) -> bool where T: PartialOrd {
    arr.windows(2).all(|w| w[0] <= w[1])
}