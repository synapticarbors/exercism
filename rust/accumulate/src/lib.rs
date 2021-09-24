pub fn map<F, T, W>(input: Vec<T>, function: F) -> Vec<W>
where
    F: FnMut(T) -> W,
{
    input.into_iter().map(function).collect()
}
