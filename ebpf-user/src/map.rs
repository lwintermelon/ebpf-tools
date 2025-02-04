use super::{skeleton::MapRef, kind};

pub struct ArrayPerCpuRef<const V: usize>(MapRef);

impl<const V: usize> kind::AppItem for ArrayPerCpuRef<V> {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        ArrayPerCpuRef(MapRef::named(name))
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(&mut self.0)
    }
}

pub struct HashMapRef<const K: usize, const V: usize>(MapRef);

impl<const K: usize, const V: usize> kind::AppItem for HashMapRef<K, V> {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        HashMapRef(MapRef::named(name))
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(&mut self.0)
    }
}
