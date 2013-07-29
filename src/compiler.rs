use std::os;
use std::io;
use std::uint;
use ptrie::PTrie;

// FIXME: exit comme un grand
#[main]
fn main()
{
  let args : ~[~str] = os::args();

  if (args.len() != 3)
  {
    println("Usage: " + args[0] + " /path/to/word/freq.txt /path/to/output/dict.bin");
  }
  else
  {
    let path    = PosixPath(args[1]);
    let freader = io::file_reader(&path);

    if freader.is_err()
    {
      println("Error: could not open file");
    }
    else
    {
      let reader   = freader.unwrap();
      let lines    = reader.read_lines();
      let mut trie = PTrie{key : None, freq : 0, succ : ~[]};

      for lines.iter().advance |line|
      {
        let words = line.word_iter().collect::<~[&str]>();
        let word  = words[0];
        let freq  = uint::from_str(words[1]);

        if freq.is_none()
        {
          println("Error: could not convert `" + words[1] + "` to an int");
        }
        else
        {
          trie.add_word(word.to_str(), freq.unwrap());
        }
      }
      //trie.serialize();
    }

  // Serialize
  // Write
  }

}
