use std::collections::BTreeMap;

pub trait VecExt<V> {
    fn group_by<K: Ord, F>(self, f: F) -> BTreeMap<K, Vec<V>>
    where
        F: Fn(&V) -> K;
}

impl<V> VecExt<V> for Vec<V> {
    fn group_by<K: Ord, F>(self, f: F) -> BTreeMap<K, Vec<V>>
    where
        F: Fn(&V) -> K,
    {
        let mut grouped: BTreeMap<K, Vec<V>> = BTreeMap::new();
        for item in self {
            grouped.entry(f(&item)).or_default().push(item);
        }
        grouped
    }
}
