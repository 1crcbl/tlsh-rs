# TLSH-RS

This is a Rust port of Trend Micro Locality Sensitive Hash (TLSH) [[github]](https://github.com/trendmicro/tlsh), [[website]](https://tlsh.org/) algorithm to compute a hash value of a byte stream. These generated hash values can then be used for similarity detection, data clustering or nearest neighbour search.

## Overview

The algorithm to construct a TLSH digest is as follows (for more detail, see [[1]](#1)):
- **Step 1**: processes an input stream by using a sliding window of length 5 and populates the hash buckets.
    Each triplet is passed through a hash function (in this implementation, the hash function is the  [Pearson hashing](https://en.wikipedia.org/wiki/Pearson_hashing)).
- **Step 2**: calculates the quartile points from the hash bucket obtained in step 1. This step might requires the sorting of the bucket array:
    ```q1```: the lowest 25% of the array
    ```q2```: the lowest 50% of the array
    ```q3```: the lowest 75% of the array
- **Step 3**: computes the digest header. The first three bytes of a hash is reserved for the header. The header of a TLSH hash consists of three parts:
    - The first byte is a checksum (with some modulo) of the byte string
    - The second byte is computed from the logarithm of the byte string's length (with some modulo)
    - The third byte is the result of ```q1_ratio <<< 4 | q2_ratio```, where
        ```q1_ratio =  (q1 * 100 / q3) MOD 16```
        ```q2_ratio =  (q2 * 100 / q3) MOD 16```
- **Step 4**: constructs the digest body from the bucket array. Note: in this step, the reversing order in reading the bucket is assumed. This means, the last element is read first while the first is read last. Their value is converted into hex form and appended into the final hash value.

## Examples
The example ```examples/tlsh_files.rs``` shows how we calculate hash values from files and measure their difference (distance). To run the example, use the following command in command line:
```
cargo run --release --example tlsh_files ../path/to/folder/with/files
```

## References
<a id="1">[1]</a> J. Oliver, C. Cheng and Y. Chen (2013). "TLSH - A Locality Sensitive Hash" [[pdf]](https://documents.trendmicro.com/assets/wp/wp-locality-sensitive-hash.pdf).

## License
TLSH is provided for use under two licenses: Apache OR BSD. Users may opt to use either license depending on the license restictions of the systems with which they plan to integrate the TLSH code. 
