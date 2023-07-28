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
  - especially input validation has to be implemented or left to the environment 
- **Multithreading (especially the permutation function)** <br> $\Rightarrow$ implementation planned after other optimizations   

### for compliance
- Implement the hash and hash map traits and requirements.

## Tests
- Implement the tests from the asconhash.txt file
- Export of the integration test into the test folder in accordance with Rust conventions.

## lib
- Implementing further traits for additional operability with different types (str, string, array, tuple)
- Clean up intermediate values, wherever possible
- Avoid unnecessary copying to speed up code 
  - use chunk_exact instead of chunk
  - possibly use tuple instead of array -> faster for smaller numbers (test in bench)
  - Replace vectors by arrays where possible
  - Where vectors cannot be implemented replace with VecDeque (double ended ring buffer) 
    $\Rightarrow$ try with and without make_contiguous
- multithreading 

## Benches 
- Benches setup has to be expended 
- Benches for the permutation function have been implemented and used for optimisations that increased performance up by a factor of 2
- Differences between an iterative approach and a simple for loop have been investigated
  - Conclusion: basically no real difference.
  - "Playing" around with compiler directives / attributes may lead to further changes in performance
- A baseline for the permutation function has been recorded.
  - saved in the target folder, which was not uploaded

 # Regarding Security Review
- **A proper security review will be left until the end of the project**
 - Automated checks with Clippy have been performed to prevent data leaks <br> $\rightarrow$ This is not sufficient as Clippy is a linting tool, not specialised for security.
   <br> $\Rightarrow$ further automated checking with dedicated SAST tools recommended
 - Several outliers occurred during benchmarking <br> $\rightarrow$ This could be due to background tasks performed by the PC <br> $\Rightarrow$ Further research is needed to determine if this could be used in timing attacks.
- Functions need to be updated so that they can fail if necessary <br> $\rightarrow$ may be needed to prevent certain attacks aimed at a heap dump or similar (more research needed; recovering the state from a data dump should not compromise the security of the hash function)
- More input validation is required
