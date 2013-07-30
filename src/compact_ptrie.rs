use std::uint;
use std::os;
use ptrie::PTrie;

#[deriving(ToStr)]
struct PTrieHeader
{
  nb_succ: u64,
  key_len: u64,
  freq:    u64 
}

/// Rebuilds the patricia trie from its compact version
pub fn rebuild_ptrie(mem: &~os::MemoryMap) -> @mut PTrie
{ do_rebuild_ptrie(mem.data as *u64, mem.data as *u64) }

fn do_rebuild_ptrie(start: *u64, curr: *u64) -> @mut PTrie
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
    let ptrie = @mut PTrie::new(key);
    ptrie.freq = (*header).freq as uint;

    /*
     * Build successors array
     */
    for uint::iterate(0u, (*header).nb_succ as uint) |i|
    {
      // We have to lookup the first character of the successor
      let child_addr                 = start + *(succbegin + i);
      let child_first_letter         = *(child_addr + 3);
      ptrie.push((child_first_letter as char, do_rebuild_ptrie(start,child_addr)));
      //ptrie.succ[child_first_letter] = Some(do_rebuild_ptrie(start, child_addr))
    }

    ptrie
  }
}
