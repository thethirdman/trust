use std::util;
use std::cast;
use std::ptr;
use std::vec;

/**
 * Patricia Trie node structure. A patricia trie is the same as a Trie but each node contains a
 * prefix instead of a character.
 */
pub struct PTrie
{
  /// Prefix stored on this node.
  key       : ~str,

  /// Weight given to this node. If equal to zero, this node does not represent the end of a word.
  /// If different from zero, it represents the frequency of apparition of this world on the
  /// internet.
  freq      : uint,

  /// Children of this node.
  succ      : ~[Option<~PTrie>]
}

impl PTrie
{
  /// Creates a new patricia trie having the key `key`, with no successors, and a `freq` of 0
  pub fn new(key : ~str) -> PTrie
  {
    unsafe
    {
      // Workaround a bug of the compiler
      let tmp : *PTrie = ptr::null();
      PTrie{key : key, freq : 0, succ : cast::transmute(vec::from_elem(256, tmp))}
    }
  }

  fn create_if(&mut self, word : ~str, succ_index: uint,  w_index : uint, freq : uint)
  {
    let succ_index = word[succ_index] as uint;
    match self.succ[succ_index]
    {
      None =>
      {
        let mut child = ~PTrie::new(word);
        child.freq = freq;
        self.succ[succ_index] = Some(child);
      },
      Some(ref mut trie) => trie.add_word_index(word, w_index, freq)
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
        let new_k = self.key.slice_to(k_index).to_str();
        let rest_k = self.key.slice_from(k_index).to_str();
        let mut ptrie_k = ~PTrie::new(rest_k);

        self.key = new_k;

        util::swap(&mut self.succ, &mut ptrie_k.succ);

        let tmp = ptrie_k.key[0] as uint;
        self.succ[tmp] = Some(ptrie_k);
      }
      // We have a common prefix: we split the key, create a new ptrie for
      // it, and create a new ptrie for the word
      else
      {
        let new_k = self.key.slice_to(k_index).to_str();
        let rest_k = self.key.slice_from(k_index).to_str();
        let rest_w = word.slice_from(w_index);
        let mut ptrie_k = ~PTrie::new(rest_k);

        self.key = new_k;


        util::swap(&mut self.succ, &mut ptrie_k.succ);

        let ptrie_w = ~PTrie::new(rest_w.to_str());
        let tmp = ptrie_k.key[0] as uint;
        self.succ[tmp] = Some(ptrie_k);
        let tmp = ptrie_w.key[0] as uint;
        self.succ[tmp] = Some(ptrie_w);
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

  fn do_serialize(&self, out: &mut ~[uint]) // FIXME: use ~[u8] ?
  {
    // XXX This is NOT a breadth first search!
    out.push(self.succ.len());
    if self.key.len() == 0
    {
      out.push(0)
    }
    else
    {
      out.push(self.key.len());
      for self.key.iter().advance |c|
      {
        out.push(c as uint);
      }
    }

    //let mut succ_id = out.len();

    for self.succ.len().times
    { out.push(0) }

    // FIXME
    /*for self.succ.iter().advance |s|
    {
      out[succ_id] = out.len();
      succ_id      = succ_id + 1;

      match s
      {
      None => { },
      Some (ref succ) => succ.do_serialize(out)
      }
    }
    */
  }
}
