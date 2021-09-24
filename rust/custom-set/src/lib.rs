#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    inner: Vec<T>,
}

impl<T: Clone + Copy + Ord + Sized + std::cmp::PartialEq> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut inner = _input.to_vec();
        inner.sort_unstable();
        inner.dedup();

        Self { inner }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.inner.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if let Err(ix) = self.inner.binary_search(&_element) {
            self.inner.insert(ix, _element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.inner.iter().all(|x| _other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.intersection(_other).is_empty()
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        Self::new(
            &self
                .inner
                .iter()
                .filter(|x| _other.contains(x))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    pub fn difference(&self, _other: &Self) -> Self {
        Self::new(
            &self
                .inner
                .iter()
                .filter(|x| !_other.contains(x))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut x = self.inner.clone();
        x.extend(_other.inner.clone());
        Self::new(&x)
    }
}
