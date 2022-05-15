Runs the first few thousand programs in the binary lambda calculus.

To try it:

- [install rust](https://rustup.rs/)
- from the root of this repo, run `$ cargo run`

Sample output:

```
0010 ->
00100 -> 0
00101 -> 1
001000 -> 00
001001 -> 01
001010 -> 10
001011 -> 11
0010000 -> 000
0010001 -> 001
0010010 -> 010
0010011 -> 011
0010100 -> 100
0010101 -> 101
0010110 -> 110
0010111 -> 111
00000010 ->
00100000 -> 0000
00100001 -> 0001
00100010 -> 0010
00100011 -> 0011
00100100 -> 0100
00100101 -> 0101
00100110 -> 0110
00100111 -> 0111
00101000 -> 1000
00101001 -> 1001
00101010 -> 1010
00101011 -> 1011
00101100 -> 1100
00101101 -> 1101
00101110 -> 1110
00101111 -> 1111
000000100 ->
000000101 ->
```

This program interprets bitstrings as BLC programs and executes them as follows:

- parse a BLC program `P` from the prefix of the string
- for the remainder of the string after that prefix, [encode](https://justine.lol/lambda/#definitions) the literal bitstring as a list `L` of `true` and `false`
- apply `P` to `L` and reduce
- assume the result is again a list of `true` and `false`, and [decode](https://justine.lol/lambda/#definitions) it back to a literal bitstring
- if any of the above steps fail, skip the bitstring as an invalid program
