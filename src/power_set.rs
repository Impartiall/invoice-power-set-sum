// Incomplete power set; drops emtpy set
pub fn power_set<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    (1..2usize.pow(items.len() as u32))
        .map(|i| {
            items
                .iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}
