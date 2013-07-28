/* Compiled representation of our Trie
struct PTrie
{
  nb_succ : int,
  key_len : int,
  word_stop : int,
  freq : int
  key     : [char],
  succ    : [int]
}
*/

struct PTrie
{
  key       : ~str,
  freq      : int,
  word_stop : bool,
  succ      : ~[PTrie]
}
