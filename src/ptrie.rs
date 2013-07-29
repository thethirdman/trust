use std::util;

/**
 * Patricia Trie node structure. A patricia trie is the same as a Trie but each node contains a
 * prefix instead of a character.
 */
pub struct PTrie
{
  /// Prefix stored on this node.
  key       : Option<~str>,

  /// Weight given to this node. If equal to zero, this node does not represent the end of a word.
  /// If different from zero, it represents the frequency of apparition of this world on the
  /// internet.
  freq      : uint,

  /// Children of this node.
  succ      : ~[PTrie]
}

impl PTrie
{
  /// Creates a new patricia trie having the key `key`, with no successors, and a `freq` of 0
  pub fn new(key : ~str) -> PTrie
  {
    PTrie{key : Some(key), freq : 0, succ : ~[]} // XXX
  }

  fn add_word_index(&mut self, word : ~str, w_index : uint, freq : uint)
  {
    match self.key.clone()
    {
      None => self.succ[word[w_index] as uint].add_word_index(word, w_index, freq),
      Some (clef) =>
      {
        let mut k_index = 0;
        let mut w_index = w_index;
        while k_index < clef.len() && w_index < word.len() && clef[k_index] == word[w_index]
        {
          k_index = k_index + 1;
          w_index = w_index + 1;
        }
        if k_index == clef.len() && w_index == word.len()
        {
          self.freq = freq
        }
        else if k_index == clef.len()
        {
          self.succ[word[w_index] as uint].add_word_index(word, w_index, freq)
        }
        else if w_index == word.len()
        {
          let new_k  = clef.slice_to(w_index);
          self.key = Some(new_k.to_str());
          self.succ[word[w_index] as uint].add_word_index(clef.to_str(), w_index + 1, freq);
        }
        else
        {
          let new_k = clef.slice_to(w_index);
          let rest_k = clef.slice_from(w_index);
          let rest_w = word.slice_from(w_index);

          self.key = Some(new_k.to_str());

          let mut ptrie_k = PTrie::new(rest_k.to_str());

          util::swap(&mut self.succ, &mut ptrie_k.succ);

          let ptrie_w = PTrie::new(rest_w.to_str());

          let tmp = ptrie_k.key.map(|s| s[0] as uint).unwrap();
          self.succ[tmp] = ptrie_k;
          let tmp = ptrie_w.key.map(|s| s[0] as uint).unwrap();
          self.succ[tmp] = ptrie_w;
        }
      }
    }

  // word == clef -> rien
  // pref(word) == clef -> {recurse}
  // sinon splitte
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
    match self.key
    {
      None        => out.push(0),
      Some(ref k) =>
        {
          out.push(k.len());
          for k.iter().advance |c|
          {
            out.push(c as uint);
          }
        }
    };

    let mut succ_id = out.len();

    for self.succ.len().times
    { out.push(0) }

    for self.succ.iter().advance |s|
    {
      out[succ_id] = out.len();
      succ_id      = succ_id + 1;

      s.do_serialize(out);
    }
  }
}
