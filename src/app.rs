use std::os;
use std::libc::consts::os::posix88::{O_RDONLY, S_IREAD};
use std::libc::funcs::posix88::fcntl::open;
use compact_ptrie;

#[main]
fn main()
{
  let args: ~[~str] = os::args();

  if args.len() != 2
  {
    fail!("Usage: " + args[0] + " /path/to/word/dict.bin");
  }

  let path = PosixPath(args[1]);

  let dico = map_file(path);

  let trie = compact_ptrie::rebuild_ptrie(&dico);

  println(trie.to_dot_str())
}

fn map_file(path: Path) -> ~os::MemoryMap
{
  let fd = do path.to_str().as_c_str |path| {
             unsafe { open(path, O_RDONLY, S_IREAD) }
           };

  let min_sz = os::page_size() * 2;

  match os::MemoryMap::new(min_sz, ~[os::MapReadable, os::MapFd(fd), os::MapOffset(0)])
  {
    Ok(mem)  => mem,
    Err(msg) => fail!(msg.to_str())
  }
}
