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

// FIXME: exit comme un grand
#[main]
fn main()
{
  let args : ~[~str] = os::args();

  if (args.len() != 3)
  {
    fail!("Usage: " + args[0] + " /path/to/word/freq.txt /path/to/output/dict.bin");
  }
  else
  {
    let path    = PosixPath(args[1]);
    let freader = io::file_reader(&path);

    if freader.is_err()
    {
      fail!("Error: could not open file");
    }
    else
    {
      let reader   = freader.unwrap();
      let lines    = reader.read_lines();
      let mut it = lines.consume_iter().transform(|l| get_word(l));
      let (w, f) = it.next().unwrap();
      let mut trie = PTrie::new(w);
      trie.freq = f;

      for it.advance |(word, freq)|
      {
          trie.add_word(word, freq);
      }
      //trie.serialize();
    }

  // Serialize
  // Write
  }

}
