use std::iter::Iterator;

/// A trait that extends [`Iterator`] with `chunks` method.
pub trait IterChunks: Sized + Iterator {
    /// Create an iterator-liked struct that yields elements by chunk every n
    /// elements, or fewer if the underlying iterator ends sooner.
    ///
    /// [`Chunks`] is not a real Iterator, but a LendingIterator, which is
    /// currently not in std and blocked by GAT. We have to iterate with
    /// while loop now.
    ///
    /// ```
    /// use iter_chunks::IterChunks;
    ///
    /// let arr = [1, 1, 2, 2, 3];
    /// let expected = [vec![1, 1], vec![2, 2], vec![3]];
    /// let mut chunks = arr.into_iter().chunks(2);
    /// let mut i = 0;
    /// while let Some(chunk) = chunks.next() {
    ///     assert_eq!(chunk.collect::<Vec<_>>(), expected[i]);
    ///     i += 1;
    /// }
    /// ```
    fn chunks(self, n: usize) -> Chunks<Self>;
}

impl<I> IterChunks for I
where
    I: Iterator,
{
    fn chunks(self, n: usize) -> Chunks<Self> {
        assert_ne!(n, 0);
        Chunks {
            inner: self,
            n,
            end_flag: false,
        }
    }
}

/// An iterator-like struct that yields chunks.
///
/// This `struct` is created by [`chunks`] method on [`IterChunks`]. See its
/// documentation for more.
///
/// [`chunks`]: IterChunks::chunks
pub struct Chunks<I: Iterator> {
    inner: I,
    n: usize,
    end_flag: bool,
}

impl<I: Iterator> Chunks<I> {
    /// Similar to [`Iterator::next`], but not implements [`Iterator`] due to
    /// lifetime.
    ///
    /// The underlying iterator implementations may choose to resume iteration
    /// after finished, so calling `Chunks::next` may also return `Some(Chunk)`
    /// after returning `None`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Chunk<'_, I>> {
        if self.end_flag {
            // The inner iterator may be resumable.
            self.end_flag = false;
            None
        } else {
            match self.inner.next() {
                Some(v) => {
                    let n = self.n;
                    Some(Chunk {
                        first: Some(v),
                        parent: self,
                        n: n - 1,
                    })
                }
                None => None,
            }
        }
    }

    /// Similar to [`Iterator::for_each`].
    ///
    /// ```
    /// use iter_chunks::IterChunks;
    ///
    /// let arr = [1, 4, 2, 3, 5];
    /// arr.into_iter().chunks(2).for_each(|chunk| {
    ///     assert_eq!(chunk.sum::<i32>(), 5);
    /// });
    /// ```
    pub fn for_each(&mut self, mut f: impl FnMut(Chunk<'_, I>)) {
        while let Some(item) = self.next() {
            f(item)
        }
    }
}

/// An iterator over a chunk of data.
///
/// Unlike [`Chunks`], `Chuuk` implements `Iterator` and can be used in for
/// loop.
///
/// This `struct` is created by [`Chunks::next`].
pub struct Chunk<'a, I: Iterator> {
    first: Option<I::Item>,
    parent: &'a mut Chunks<I>,
    n: usize,
}

impl<'a, I> Iterator for Chunk<'a, I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.first.take() {
            Some(v) => Some(v),
            None if self.n > 0 => {
                self.n -= 1;
                match self.parent.inner.next() {
                    Some(v) => Some(v),
                    None => {
                        // The current chunk iterator should output None and end forever.
                        self.n = 0;

                        // The parent chunks iterator should output None once.
                        self.parent.end_flag = true;

                        None
                    }
                }
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.parent.inner.size_hint();
        // SAFETY: `checked_add` is unnecessary here since n is always less than
        // `usize::MAX`.
        let has_first = self.first.is_some() as usize;
        let n = self.n;
        let lower = lower.min(n) + has_first;
        let upper = upper.map(|v| v.min(n) + has_first);
        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::IterChunks;

    #[test]
    fn test_impls() {
        let chunks = [0i32].into_iter().chunks(1);

        // A helper function that asserts a type impl Send.
        fn assert_send<T: Send>(_: &T) {}
        // A helper function that asserts a type impl Sync.
        fn assert_sync<T: Sync>(_: &T) {}

        assert_sync(&chunks);
        assert_send(&chunks);
    }

    #[test]
    fn test_chunks() {
        let arr = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3];
        let mut i = 0;
        let mut chunks = arr.into_iter().chunks(3);

        while let Some(chunk) = chunks.next() {
            for v in chunk {
                assert_eq!(v, i);
            }
            i += 1;
        }
        assert_eq!(i, 4);
    }

    #[test]
    fn test_chunk_resumable() {
        let inner_gen = |rem| {
            let mut i = 0;
            std::iter::from_fn(move || {
                i += 1;
                if i % rem == 0 {
                    None
                } else {
                    Some(i)
                }
            })
        };

        let inner = inner_gen(3);
        let mut chunks = inner.chunks(4);
        while let Some(chunk) = chunks.next() {
            assert_eq!(chunk.collect::<Vec<_>>(), vec![1, 2]);
        }
        while let Some(chunk) = chunks.next() {
            assert_eq!(chunk.collect::<Vec<_>>(), vec![4, 5]);
        }
        while let Some(chunk) = chunks.next() {
            assert_eq!(chunk.collect::<Vec<_>>(), vec![7, 8]);
        }

        let inner = inner_gen(6);
        let mut chunks = inner.chunks(4);

        assert_eq!(chunks.next().unwrap().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(chunks.next().unwrap().collect::<Vec<_>>(), vec![5]);
        assert!(chunks.next().is_none());

        assert_eq!(
            chunks.next().unwrap().collect::<Vec<_>>(),
            vec![7, 8, 9, 10]
        );
        assert_eq!(chunks.next().unwrap().collect::<Vec<_>>(), vec![11]);
        assert!(chunks.next().is_none());
    }

    #[test]
    fn test_chunks_count() {
        let arr: [bool; 0] = [];
        let mut i = 0;
        let mut chunks = arr.into_iter().chunks(3);

        while let Some(chunk) = chunks.next() {
            for _ in chunk {}
            i += 1;
        }
        assert_eq!(i, 0);

        let arr: [bool; 3] = [false; 3];
        let mut i = 0;
        let mut chunks = arr.into_iter().chunks(3);

        while let Some(chunk) = chunks.next() {
            for _ in chunk {}
            i += 1;
        }
        assert_eq!(i, 1);
    }

    #[test]
    fn test_size_hint() {
        let iter = [1, 2, 3, 4]
            .into_iter()
            .chain([5, 6, 7].into_iter().filter(|_| true));
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 4);
        assert_eq!(upper, Some(7));
        let mut chunks = iter.chunks(3);

        let mut chunk1 = chunks.next().unwrap();

        assert_eq!(chunk1.size_hint(), (3, Some(3)));
        chunk1.next().unwrap();
        assert_eq!(chunk1.size_hint(), (2, Some(2)));

        for _ in chunk1 {}

        let chunk2 = chunks.next().unwrap();
        assert_eq!(chunk2.size_hint(), (1, Some(3)));
        for _ in chunk2 {}

        let mut chunk3 = chunks.next().unwrap();
        assert_eq!(chunk3.size_hint(), (1, Some(1)));
        chunk3.next().unwrap();
        assert_eq!(chunk3.size_hint(), (0, Some(0)));
    }
}
