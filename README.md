trust
=====

Spell checker in Rust. It uses a compiled patricia trie to compute the Damerau-Levenshtein to find
the words nearest to a user input. It it composed of two tools:
  * **TextMiningCompiler** (compiler.rc): builds the patricia trie in compact form from a
    dictionary.
  * **TextMiningApp** (app.rc): reads the trie output by the compiler, and reads its standard input
    for requets with the form "approx 2 foo" (i.e. find in the dictionary all words which are at
      distance 2 from "foo".

Compilation
===========
Since this is a project for a school assignment, detailed instruction are given to compile the
rust compiler itself.

# Automated way
There is a `configure` file which basically do all the compilation process for you; including the
compilation of the rust compiler.

At some point, to install the rust compiler, you will be prompted for your sudoer password.

```
  ./configure
  make
```

This should install the rust compiler and create two binaries: `TextMimingCompiler` and
`TextMiningApp`.

# Manual way
## Get the rust compiler
You will need one of the latest version of the Rust compiler.
The last version was 0.8-pre from the master branch, sha1: 6296dc0d73527301f18ef55b5f2d07c3241b8a00
The following should work in most platforms:

```
  git clone git://github.com/mozilla/rust.git
  cd rust
  git checkout 6296dc0d73527301f18ef55b5f2d07c3241b8a00
  ./configure
  make
  sudo make install
```

## Compile the project
This should create two binaries: `TextMimingCompiler` and `TextMiningApp`:

```
git clone git://github.com/thethirdman/trust.git
cd trust
make
```
