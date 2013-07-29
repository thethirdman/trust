use std::uint;
use std::os;
// use std::vec;
use ptrie::PTrie;

#[deriving(ToStr)]
struct PTrieHeader
{
  nb_succ: u64,
  key_len: u64,
  freq:    u64 
}

/// Rebuilds the patricia trie from its compact version
pub fn rebuild_ptrie(mem: &~os::MemoryMap) -> ~PTrie
{ do_rebuild_ptrie(mem.data as *u64, mem.data as *u64) }

fn do_rebuild_ptrie(start: *u64, curr: *u64) -> ~PTrie
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
      let child_addr                 = start + *(succbegin + i);

      ptrie.push(do_rebuild_ptrie(start,child_addr));
      //ptrie.succ[child_first_letter] = Some(do_rebuild_ptrie(start, child_addr))
    }

    ptrie
  }
}

/*
pub fn find_candidates(word: ~str, distance: uint) -> ~[~str]
{
  let distance_table = vec::with_capacity(word.len() * word.len());

  for uint::iterate(0u, word.len()) |i|
  { distance_table.push(i) } // first line
}
*/

/*
fn do_find_candidates(word: ~str, w_index: uint, d: ~[uint], res: &mut ~[~str])
{
  let wlen     = word.len();
  let w_letter = word[w_index];
  let t_letter = ???;

  for wlen.times
  {
    let l    = d.len() - 1;
    let u    = d.len() - wlen;
    let ul   = u - 1;
    let uull = d.len() - 2 - 2 * wlen;

    let sub_weight = if t_letter == w_letter
                     { 0 }
                     else
                     { 1 };
    let trans_weight = if ???
                       { 1 }
                       else
                       { Bounded::max_value<uint>() - d[uull] }

    let distance = (d[l] + 1).min(d[u] + 1).min(d[ul] + sub_weight).min(d[uull] + trans_weight);

    d.push(distance);
  }
}
*/
