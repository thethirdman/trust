use std::uint;
use std::os;
use std::str;
use ptrie::PTrie;

/// Word with its frequency and the distance from a reference word.
pub struct Word
{
  /// The actual word.
  word: ~str,
  /// Word aparition frequency on the internet.
  freq: uint,
  /// Distance between this word and a referance word.
  dist: uint
}

impl Word
{
  /// Creates a new word.
  pub fn new(word: ~str, freq: uint, dist: uint) -> Word
  {
    Word {
      word: word,
      freq: freq,
      dist: dist
    }
  }
}

impl Eq for Word
{
  fn eq(&self, other: &Word) -> bool
  { self.dist == other.dist && self.freq == other.freq && self.word == other.word }
}

impl Ord for Word
{
  fn lt(&self, other: &Word) -> bool
  {
    self.dist < other.dist ||
    (self.dist == other.dist && (self.freq > other.freq ||
                                 (self.freq == other.freq && self.word < other.word)))
  }
}

impl ToStr for Word
{
  pub fn to_str(&self) -> ~str
  { 
    "{\"word\":\"" + self.word +
    "\",\"freq\":" + self.freq.to_str() + 
    ",\"distance\":" + self.dist.to_str() +
    "}"
  }
}

#[deriving(ToStr)]
struct PTrieHeader
{
  nb_succ: uint,
  key_len: uint,
  freq:    uint 
}

/// Rebuilds the patricia trie from its compact version
pub fn rebuild_ptrie(mem: &~os::MemoryMap) -> ~PTrie
{ do_rebuild_ptrie(mem.data as *uint, mem.data as *uint) }

fn do_rebuild_ptrie(start: *uint, curr: *uint) -> ~PTrie
{
  unsafe {
    let header = curr as *PTrieHeader;

    let kbegin    = curr + 3;
    let succbegin = curr + 3 + (*header).key_len;

    /*
     * Extract key
     */
    let mut key = ~"";

    for uint::iterate(0u, (*header).key_len as uint) |i|
    {
      key.push_char(*(kbegin + i) as char);
    }

    /*
     * Build the new node with the key
     */
    let mut ptrie = ~PTrie::new(key);
    ptrie.freq = (*header).freq as uint;

    /*
     * Build successors array
     */
    for uint::iterate(0u, (*header).nb_succ as uint) |i|
    {
      // We have to lookup the first character of the successor
      let child_addr = start + *(succbegin + i);

      ptrie.push(do_rebuild_ptrie(start,child_addr));
      //ptrie.succ[child_first_letter] = Some(do_rebuild_ptrie(start, child_addr))
    }

    ptrie
  }
}

/// Structure to incrementally compute the Damerau-Levenshtein distance between a known reference
/// word and a stream of characters.
pub struct DLDist
{
  /// The reference word.
  original: ~str,
  /// Current characters read from the stream.
  current:  ~[u8],
  /// Table used to compute the Damerau-Levenshtien distance using dynamic programming.
  table:    ~[uint],
  /// Max distance allowed between the reference word and the character stream. Once this distance
  /// is reached the `take` method will return false.
  max_dist: uint
}

impl ToStr for DLDist
{
  fn to_str(&self) -> ~str
  {
    let mut res = ~"";

    res = res + "< " + self.original + " > vs < " + str::from_bytes(self.current) + " >\n";

    res = res + "Table:\n";

    for uint::iterate(0u, self.current.len() + 1) |i|
    {
      for uint::iterate(0u, self.original.len() + 1) |j|
      {
        let idx = i * (self.original.len() + 1) + j;
        if  idx < self.table.len()
        { res = res + " " + self.table[idx].to_str() }
      }

      res = res + "\n";
    }

    res = res + "### raw: " + self.table.to_str();

    res
  }
}

impl DLDist
{
  /// Create a new structure to compute the Damerau-Levenshtein incrementally. It needs to be
  /// initialized using the `reset` method.
  pub fn new()-> DLDist
  {

    DLDist {
      current:  ~[],
      table:    ~[],
      original: ~"",
      max_dist:  0
    }
  }

  /// Returns on the past. All operations done after the `new_len` character are discarded.
  pub fn truncate(&mut self, new_len: uint)
  {
    self.current.truncate(new_len);
    self.table.truncate((new_len + 1) * (self.original.len() + 1));
  }

  /// Reinitializes the algorithm.
  ///
  /// # Arguments
  ///   * word - the reference word.
  ///   * max_dist - reachable distance before the algorithm gives up.
  pub fn reset(&mut self, word : ~str, max_dist : uint)
  {
    self.current.clear();
    self.table.clear();
    self.original = word;
    self.max_dist = max_dist;

    for uint::iterate(0u, self.original.len() + 1) |i|
    { self.table.push(i) }
  }

  /// Takes a character and computes the Damerau-Levenshtein distance between the newly formed word
  /// and the reference word.
  pub fn take(&mut self, c: u8) -> (bool, bool)
  {
    let mut should_stop = true;

    self.current.push(c);

    self.table.push(self.current.len());

    for uint::iterate(0u, self.original.len()) |j|
    {
      let i    = self.current.len();
      let l    = self.table[self.table.len() - 1];
      let u    = self.table[self.table.len() - (self.original.len() + 1)];
      let ul   = self.table[self.table.len() - (self.original.len() + 1) - 1];

      let sub_weight = if c == self.original[j]
      { 0 }
      else
      { 1 };

      let distance = (l + 1).min(&(u + 1)).min(&(ul + sub_weight));

      if i > 1 && j > 0
      {
        let uull = self.table[self.table.len() - 2 * (self.original.len() + 1) - 2];

        let trans_weight = if self.current[i - 1] == self.original[j - 1] &&
                              self.current[i - 2] == self.original[j]
        { 1 }
        else
        { Bounded::max_value::<uint>() - uull };

        self.table.push(distance.min(&(uull + trans_weight)));
      }
      else
      {
        self.table.push(distance);
      }

      if distance <= self.max_dist
      { should_stop = false }
    }

    (should_stop, self.dist() <= self.max_dist)
  }

  /// Damerau-Levenshtein distance between `self.curr` and `self.original`.
  pub fn dist(&self) -> uint
  { *self.table.last() }
}

/// Finds every dictionary words at a certain distance from a reference word.
///
/// # Arguments:
///   * `mem`      - the compiled patricia trie.
///   * `word`     - the reference word to search.
///   * `distance` - the maximum distance between the reference word and the dictionary words.
///   * `algo`     - pre-allocated incremental Damerau-Levenshtein algorithm cache.
pub fn find_candidates(mem: &os::MemoryMap, word: ~str, distance: uint, algo : &mut DLDist) -> ~[Word]
{
  let mut res  = ~[];
  //let mut algo = DLDist::new(word, distance);

  algo.reset(word, distance);
  do_find_candidates(mem.data as *uint, mem.data as *uint, algo, &mut res);
  res
}

fn do_find_candidates(start: *uint, curr: *uint, algo: &mut DLDist, out: &mut ~[Word])
{
  unsafe {
    let header    = curr as *PTrieHeader;
    let kbegin    = curr + 3;
    let succbegin = curr + 3 + (*header).key_len;

    /*
     * Consume the current word’s letters
     */
    let mut accept = false;

    for uint::iterate(0u, (*header).key_len as uint) |i|
    {

      let (should_stop, accept_this) = algo.take(*(kbegin + i) as u8);

      accept = accept_this;

      if should_stop
      { return }
    }

    if accept && (*header).freq != 0
    { out.push(Word::new(str::from_bytes(algo.current), (*header).freq as uint, algo.dist())) }

    /*
     * Go to the children (recursive call)
     */
    let curr_len = algo.current.len();

    for uint::iterate(0u, (*header).nb_succ as uint) |i|
    {
      // first, partially reinitialise the algorithm
      algo.truncate(curr_len);

      let child_addr = start + *(succbegin + i);

      // and start the recursion on the next branch
      do_find_candidates(start, child_addr, algo, out);
    }
  }
}

mod test
{
  #[test]
  use compact_ptrie::DLDist;

  #[test]
  fn test_dist()
  {
    // hellow vs hlelow
    let mut algo = DLDist::new(~"hellow", 2);

    algo.take('h' as u8);

    assert!(algo.dist() == 5);

    algo.take('l' as u8);
    algo.take('e' as u8);
    algo.take('l' as u8);
    algo.take('o' as u8);
    algo.take('w' as u8);

    assert!(algo.dist() == 1);

    // hellow vs helpow
    let mut algo = DLDist::new(~"hellow", 2);

    algo.take('h' as u8);
    algo.take('e' as u8);
    algo.take('l' as u8);
    algo.take('p' as u8);
    algo.take('o' as u8);
    algo.take('w' as u8);

    assert!(algo.dist() == 1);
  }
}
