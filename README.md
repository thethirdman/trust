trust
=====

Spell checker in Rust. It uses a compiled Patricia Trie to compute the Damerau-Levenshtein to find
the words nearest to a user input. It it composed of two tools:
  * **TextMiningCompiler** (compiler.rc): builds the Patricia Trie in compact form from a
    dictionary.
  * **TextMiningApp** (app.rc): reads the trie output by the compiler, and reads its standard input
    for requets with the form "approx 2 foo" (i.e. find in the dictionary all words which are at
      distance 2 from "foo".

**trust** is open-source under the BSD-3 licence, and available on github:
https://github.com/thethirdman/trust

Compilation
===========
Since this is a project for a school assignment, detailed instruction are given to compile the
Rust Compiler itself.

Make sure you are connected to the internet as the Rust Compiler requires do download something during the configure stage.
The Rust Compiler compilation may use several Go of RAM (3Go in our machines), and take from 30mn to 1h30.

## Automated
There is a `configure` file which basically do all the compilation process for you; including the
compilation of the Rust Compiler.

At some point, to install the Rust Compiler, you will be prompted for your sudoer password.

```
  ./configure
  make
```

This should install the Rust Compiler and create two binaries: `TextMimingCompiler` and
`TextMiningApp`.

## Manual
### Get the Rust Compiler
You will need one of the latest version of the Rust compiler (https://github.com/mozilla/rust).
The last version was `0.8-pre` from the `master` branch, sha1: `6296dc0d73527301f18ef55b5f2d07c3241b8a00`
The following should work in most platforms:

```
  git clone git://github.com/mozilla/rust.git
  cd rust
  git checkout 6296dc0d73527301f18ef55b5f2d07c3241b8a00
  ./configure
  make -j5
  sudo make install
```

### Compile the project
This should create two binaries: `TextMimingCompiler` and `TextMiningApp`:

```
git clone git://github.com/thethirdman/trust.git
cd trust
make
```

## Contributions
Keep in mind that this is a school assignment project. Thus, we do not plan to maintain this tool
beyond the assignment deadline (July 31 2013). Anyway, do not hesitate to contribute. If you want
do take-over the repository (and ensure a better future for this project) create an issue, or
contact one of the two lead developers.

## Additional informations
Here are some miscellaneous information about the **trust** project.

### How is designed trust?

### How did we test trust?
Two tools are built we tested them by:
  * manually checking that the Patricia Trie is correct (using a text representation of the
    Patricia Trie and the `dot` tool),
  * re-building the Patricia Trie from its compiled version and verify that its text
    representation is the same as the with the Patricia Trie obtained before compilation,
  * unit-testing the incremental Damerau-Levenshtein distance algorithm.

### Is the Damerau-Levenshtein distance based spell-checker accurate ? When is it not?

### Why did we implement a Patricia Trie (Radix Trie)?
A Patricia Trie is well known for its good performances and memory efficiency. Thus it was the most
appealing data structure choice given the assignment constraints.

### If we need an accurate spell-checking, how could we automate the choice of the maximal distance between the requested word and the dictionary words?

### Further improvements?
**trust** can be improved in several ways:
  * Parallelize the spell checker. Each request to the spell checker are deeply independents. Thus, handling
    batches of requests could easily be parallelized with one job per Patricia Trie traversal.
  * Parallelize the Patricia Trie construction. Depending on how fine grained we want the
    parallelism to be, this can be fairly easy. One interesting approach could be to sort all the
    dictionary words in lexicographic order. Build, in parallel, patricia-tries for words begining
    with different letters. Then, attach thote tries to an universal ancestor (the empty root).
Do not hesitate to contribute!

### Is trust a state-of-the-art spell checker? Why?
