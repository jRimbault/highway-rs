v0.1.2 - 2018-09-23

- Fix: AVX enabled hash could segfault on unaligned loads of user input.

v0.1.1 - 2018-09-20

- Fix: SIMD enabled hash functions would return the improper response when not compiled with either an explicit `target-cpu=native` or if `target-feature=+avx2` was omitted

v0.1.0 - 2018-09-19

- Initial Release