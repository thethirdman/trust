use std::util;
use my_list::{HasKey, MyList, Cons, Nil};

/**
 * Patricia Trie node structure. A patricia trie is the same as a Trie but each node contains a
 * prefix instead of a character.
 */
#[deriving(Clone)]
pub struct PTrie
{
  /// Prefix stored on this node.
  key       : ~str,

  /// Weight given to this node. If equal to zero, this node does not represent the end of a word.
  /// If different from zero, it represents the frequency of apparition of this world on the
  /// internet.
  freq      : uint,

  /// Children of this node.
  succ      : ~MyList<~PTrie, u8>
}

impl HasKey<u8> for ~PTrie
{
  fn key(&self) -> u8
  { self.key[0] }
}

impl HasKey<u8> for PTrie
{
  fn key(&self) -> u8
  { self.key[0] }
}


impl PTrie
{
  /// Creates a new patricia trie having the key `key`, with no successors, and a `freq` of 0
  pub fn new(key : ~str) -> PTrie
  {
      PTrie{key : key, freq : 0, succ : ~Nil}
  }

  /// Number of nodes in this patricia trie.
  pub fn len(&self) -> uint
  {
    let mut add_this = 1;

    do self.succ.iter |c|
    { add_this = add_this + c.len() }

    add_this
  }

  /// Adds an element to the successors list
  pub fn push(&mut self, elt : ~PTrie)
  {
    let mut newthing = ~Nil;
    util::swap(&mut self.succ, &mut newthing);
    self.succ = ~Cons(elt, newthing);
  }

  /// Dot reprensentation of the patricia trie.
  pub fn to_dot_str(&self) -> ~str
  {
    let mut res = ~"digraph ptrie {\n";

    self.lbl_to_dot_str(&mut 0, &mut res);
    self.edg_to_dot_str(&mut 0, &mut res);

    res = res + "}\n";

    res
  }

  fn lbl_to_dot_str(&self, id: &mut uint, out: &mut ~str)
  {
    if self.freq != 0
    {
      *out = *out + id.to_str() +
             " [label=\"" + self.key + " ( " + self.freq.to_str() + " ) " + "\", color = red];\n";
    }
    else
    {
      *out = *out + id.to_str() + " [label=\"" + self.key + "\"];\n";
    }
    *id  = *id + 1;

    self.succ.iter(|b| b.lbl_to_dot_str(id, out))
  }

  fn edg_to_dot_str(&self, id: &mut uint, out: &mut ~str)
  {
    let me = *id;

    do self.succ.iter |b|
    {
      *id  = *id + 1;
      *out = *out + me.to_str() + " -> " + id.to_str() + "\n";
      b.edg_to_dot_str(id, out)
    }

  }

  fn create_if(&mut self, word : ~str, succ_index: uint,  w_index : uint, freq : uint)
  {
    let child = match self.succ.find_mut(|c| c == word[succ_index])
       {
         None               =>
         {
           let suffix = word.slice_from(w_index).to_str();
           let mut child = ~PTrie::new(suffix);
           assert!(child.freq == 0);
           child.freq = freq;
           Some(child)
         },
         Some(ref mut trie) =>
         {
           trie.add_word_index(word, w_index, freq);
           None
         }
       };
    match child
    {
      None => { },
      Some (child) => self.push(child)
    }
  }

  fn add_word_index(&mut self, word : ~str, w_index : uint, freq : uint)
  {
    // If the trie is empty
    if self.key.len() == 0
    {
      self.create_if(word, w_index, w_index, freq)
    }
    else
    {
      let mut k_index = 0;
      let mut w_index = w_index;

      // While the strings are the same
      while k_index < self.key.len() && w_index < word.len() && self.key[k_index] == word[w_index]
      {
        k_index = k_index + 1;
        w_index = w_index + 1;
      }
      // If we reached the end on both the word and the key, then we just update
      // the freq
      if k_index == self.key.len() && w_index == word.len()
      {
        assert!(self.freq == 0);
        self.freq = freq
      }
      // If we reach the end of the key, we just continue the insertion on the next node
      else if k_index == self.key.len()
      {
        self.create_if(word, w_index, w_index, freq)
      }
      // If we reach the end of the word, we need to split the key
      else if w_index == word.len()
      {
        let new_k       = self.key.slice_to(k_index).to_str();
        let rest_k      = self.key.slice_from(k_index).to_str();

        let mut ptrie_k = ~PTrie::new(rest_k);

        self.key = new_k;

        util::swap(&mut self.succ, &mut ptrie_k.succ);

        ptrie_k.freq = self.freq;
        self.freq    = freq;

        self.push(ptrie_k);
      }
      // We have a common prefix: we split the key, create a new ptrie for
      // it, and create a new ptrie for the word
      else
      {
        let new_k       = self.key.slice_to(k_index).to_str();
        let rest_k      = self.key.slice_from(k_index).to_str();
        let rest_w      = word.slice_from(w_index);
        let mut ptrie_k = ~PTrie::new(rest_k);

        ptrie_k.freq = self.freq;
        self.key = new_k;


        util::swap(&mut self.succ, &mut ptrie_k.succ);

        let mut ptrie_w    = ~PTrie::new(rest_w.to_str());

        ptrie_w.freq = freq;

        self.push(ptrie_k);
        self.push(ptrie_w);
        self.freq = 0;
      }
    }
  }

  /**
   * Add a word to the trie.
   *
   * # Argument:
   *  * `word` - contains the word to add to the trie.
   *  * `freq` - the frequency of the word
   */
  pub fn add_word(&mut self, word : ~str, freq : uint)
  {
    self.add_word_index(word, 0, freq);
  }
  /**
   * Compiled representation of our Trie. The output structure looks like that for each node:
   *
   * ```
   *    struct PTrie
   *    {
   *      nb_succ   : int,
   *      key_len   : int,
   *      freq      : int
   *      key       : [char, ..key_len],
   *      succ      : [int,  ..nb_succ]
   *    }
   * ```
   */
  pub fn serialize(&self) -> ~[uint]
  {
    let mut res = ~[];

    self.do_serialize(&mut res);

    res
  }

  fn do_serialize(&self, out: &mut ~[uint])
  {
    let num_succ = self.succ.len();
    out.push(num_succ);
    out.push(self.key.len());
    out.push(self.freq);

    for self.key.iter().advance |c|
    {
      out.push(c as uint);
    }

    let mut succ_id = out.len();

    for num_succ.times
    { out.push(0) }

    do self.succ.iter |succ|
    {
      out[succ_id] = out.len();
      succ_id      = succ_id + 1;
      succ.do_serialize(out)
    }
  }
}
