# Simple-ascon-hash-implementation-rust
A first simple implementation of ascon-hash in Rust. 

**The implementation is bound to change and a work in progress**


**no security review of the code base has been done as of now**

# Caveat 
- All tests, including the integration test work for now.
- The implementation is still unoptimzed and some functions have to be adjusted.
- The integration test only uses the string "some bytes". The solution has been taken from the lib.rs file by Sebastian Ramacher
- the test from the file asconhash.txt do not pass
  - here I assume my understanding of the file and test might have been flawed -> still to do
 
# To-do

## General 
- write a bench mark / test bench
- make complient with https://github.com/RustCrypto/hashes/tree/master
- security review
- multithreading (especially the permutation function)

### for complience
- implement the hash and hash map traits and requirements

## Tests
- implement the tests fom the file asconhash.txt
- export the integration test to the test folder as is convention in rust

## lib
- implementing further traits for additional operability with different types (str, string, array, tuple)
- purge intermediary values
- reduce copying of code
  - use chunk_exact instead of chunk
  - potentionaly use tuple instead of array -> faster for smaller numbers (test in bench)
  - replace vectors by arrays where possible
  - where vectors cannot be implemented replace with VecDeque (double ended ring buffer) <br>
    $\Rightarrow$ try with and without make_contiguous
- multithreading 
