use std::os;
use std::io;
use std::uint;
use ptrie::PTrie;

fn get_word(line : ~str) -> (~str, uint)
{
  let words = line.word_iter().collect::<~[&str]>();
  let word  = words[0];
  let freq  = uint::from_str(words[1]);

  match freq
  {
    None => fail!("Error: could not convert `" + words[1] + "` to an int"),
    Some (ret) => (word.to_str(), ret)
  }
}

fn make_trie(args : &[~str]) -> ~PTrie
{
  if (args.len() != 3)
  {
    fail!("Usage: " + args[0] + " /path/to/word/freq.txt /path/to/output/dict.bin")
  }
  else
  {
    let path    = PosixPath(args[1]);

    match io::file_reader(&path)
    {
      Err (_) =>
      {
        fail!("Error: could not open file");
      },
      Ok (freader) =>
      {
        let lines    = freader.read_lines();
        let mut it = lines.consume_iter().transform(|l| get_word(l));
        let (w, f) = it.next().unwrap();
        let mut trie = ~PTrie::new(w);
        trie.freq = f;

        for it.advance |(word, freq)|
        {
          trie.add_word(word, freq);
        }
        trie
      }
    }
  }
}

fn serialize_trie(args : &[~str], trie : ~PTrie)
{
  let path          = PosixPath(args[2]);
  let fwriter       = io::buffered_file_writer(&path).unwrap();
  trie.serialize(fwriter);
  // println(trie.to_dot_str());
}

#[main]
fn main()
{
  let args : ~[~str] = os::args();
  serialize_trie(args, make_trie(args));
}

