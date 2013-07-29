/// Traits of objects having a key.
pub trait HasKey<T>
{
  /// Key representative of this object.
  fn key(&self) -> T;
}

#[deriving(Clone, DeepClone, ToStr)]
pub enum MyList<T, K>
{
  Cons(T, ~MyList<T, K>),
  Nil
}

impl<T, K> MyList<T, K>
{
  /// List Length.
  pub fn len(&self) -> uint
  {
    match *self
    {
      Cons(_, ref child) => 1 + child.len(),
      Nil                => 0
    }
  }

  /// Iterate over a list.
  pub fn iter(&self, f: &fn(&T))
  {
    match *self
    {
      Cons(ref k, ref child) => {
        f(k);
        child.iter(f);
      },
      Nil                => { }
    }
  }
}

impl<T: HasKey<K>, K> MyList<T, K>
{
  ///  Search for an element that matches a given predicate
  /// 
  /// Apply function f to each element of v, starting from the first. When function f returns true
  /// then an option containing the element is returned. If f matches no elements then none is
  /// returned. 
  pub fn find_mut<'r>(&'r mut self,
                      p: &fn(K) -> bool)
                     -> Option<&'r mut T>
  {
    match *self
    {
      Cons(ref mut k, ref mut child) => {
        if p(k.key())
        { Some(k) }
        else
        { child.find_mut(p) }
      },
      Nil                    => None
    }
  }
}
