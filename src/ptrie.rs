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
  succ      : ~[PTrie]
}

impl PTrie
{
  /**
   * Compiled representation of our Trie. The output structure looks like that for each node:
   *
   * ```
   *    struct PTrie
   *    {
   *      nb_succ   : int,
   *      key_len   : int,
   *      word_stop : int,
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
    out.push(self.key.len());
    for self.key.iter().advance |c|
    {
      out.push(c as uint);
    }

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
