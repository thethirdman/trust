use std::os;
use std::libc::consts::os::c95::SEEK_END;
use std::libc::consts::os::posix88::{O_RDONLY, S_IREAD};
use std::libc::funcs::posix88::fcntl::open;
use std::libc::funcs::posix88::unistd::lseek;
use extra::sort;
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

  let mut candidates = compact_ptrie::find_candidates(dico, ~"secin", 2);

  sort::quick_sort(candidates, |a, b| a < b);
  print_array_without_spaces(candidates);
  // let trie = compact_ptrie::rebuild_ptrie(&dico);
  // println(trie.to_dot_str())
}

fn map_file(path: Path) -> ~os::MemoryMap
{
  let fd = do path.to_str().as_c_str |path| {
             unsafe { open(path, O_RDONLY, S_IREAD) }
           };

  // FIXME: no better way to get he file size?
  let file_size = unsafe { lseek(fd, 0, SEEK_END) } as uint;
  let min_sz = (file_size / os::page_size() + 1) * os::page_size();

  match os::MemoryMap::new(min_sz, ~[os::MapReadable, os::MapFd(fd), os::MapOffset(0)])
  {
    Ok(mem)  => mem,
    Err(msg) => fail!(msg.to_str())
  }
}

fn print_array_without_spaces<T: ToStr>(arr: &[T])
{
  print("[");

  if arr.len() != 0
  {
    let mut it = arr.iter();
    print(it.next().unwrap().to_str());

    for it.advance |e|
    {
      print(",");
      print(e.to_str());
    }
  }

  println("]")
}
