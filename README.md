# Simple-ascon-hash-implementation-rust
A first simple implementation of ascon-hash in Rust. 

**The implementation is subject to change and is a work in progress.**


**No security review of the codebase has been done yet.**

# Caveat 
- All tests, including integration tests, have passed. However, integration testing needs to be extended 
- The implementation is still unoptimised and some features need to be tweaked.
- The integration test only uses the string "some bytes". The result was taken from the lib.rs file by Sebastian Ramacher
- the test from asconhash.txt does not pass
  - here I assume that my understanding of the file and the test might have been wrong -> still to do
 
# To do

## General stuff 
- Comply with https://github.com/RustCrypto/hashes/tree/master
- Security review
- Multithreading (especially the permutation function)

### for compliance
- Implement the hash and hash map traits and requirements.

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

## benches 
- only a very rudementary bench has been implemented. 
- benches setup has to be expended 
