use std::borrow::Borrow;
use std::hash::Hash;

pub trait DispatchFunction<P: ?Sized, R>: Fn(&P) -> R + 'static + Sync + Send
where
    P: Send + Sync,
{
}

impl<T, P: ?Sized, R> DispatchFunction<P, R> for T
where
    T: Fn(&P) -> R + 'static + Sync + Send,
    P: Send + Sync,
{
}

pub trait Dispatcher<K, P: ?Sized, R>
where
    K: Eq + Hash,
{
    fn into_vec(self) -> Vec<(K, Box<dyn DispatchFunction<P, R>>)>;

    fn remove<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Eq + Hash;

    fn insert(&mut self, key: K, item: Box<dyn DispatchFunction<P, R>>);

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash;

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Box<dyn DispatchFunction<P, R>>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash;

    fn len(&self) -> usize;

    fn call<Q: ?Sized>(&self, key: &Q, params: &P) -> Option<R>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        match self.get(key) {
            Some(func) => Some(func(params)),
            None => None,
        }
    }
}
