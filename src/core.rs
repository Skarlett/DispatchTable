pub trait DispatchFunction<P, R>: Fn(&P) -> R + 'static + Sync + Send
where
    P: Send + Sync {}

impl<T, P, R> DispatchFunction<P, R> for T
where 
  T: Fn(&P) -> R + 'static + Sync + Send, 
  P: Send + Sync {}

pub trait Dispatcher<K, P, R> 
where
    K: std::cmp::Eq
{
    fn into_vec(self) -> Vec<(K, Box<dyn DispatchFunction<P, R>> )>;
    fn remove(&mut self, key: &K);
    fn insert(&mut self, key: K, item: Box<dyn DispatchFunction<P, R>>);
    fn contains_key(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&Box<dyn DispatchFunction<P, R>>>;
    fn len(&self) -> usize;
    
    fn call(&self, key: &K, params: &P) -> Option<R> {
        match self.get(key) {
            Some(func) => Some(func(params)),
            None => None            
        }
    }
}