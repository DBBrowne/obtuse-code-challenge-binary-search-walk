# Obtuse technical tests - "football" scores<a id="title-link>"></a>

## Contents
- [Obtuse technical tests - "football" scores](#title-link)
- [Intro](#intro)
  - [Performance](#performance)
- [Usage](#usage)
- [Running Tests](#running-tests)
- [Known Isues](#known-issues)

### Intro
This project came from an obtuse interview question.  After failing the initial 1 hour test, I worked through the "sort-then-walk" solutions in JS and Python about another 1.5-2 hours.

A better solution, using a fuzzy binary search exists however, which is much faster than my attempt.

Although obfuscated by language about leagues with 10^5 matches and football games with scores up to 10^9, the question was fundamentally:

Given two arrays of integers, return an array containing the counts of members of inputA with values lower than or equal to each member of inputB, retaining the order of inputB.

```
2 <= n <= 10^5
1 <= input[i] <= 10^9 where 0 <= i < n
```
eg:
```js
counts([1,2,3], [2,4]) 
>>>
[2,3] 
// because:
// inputB[0] = 2, which is >= two members of inputA, 
// inputB[1] = 4, which is >= 3 members of inputA]
```

#### Performance
A summary:
- Node does respectably, excellent given how easy it is to use.
- Python starts to struggle at the larger input sizes, but handles increases in the maximum score better han Node does, retaining similar execution times when each member of the input limit moves from 1e6 to 1e9.
- Rust is a compiled language, designed for speed, so dominates.  For a low level language, it is very easy to work with, with an excellent and extremely helpful compiler.
> Obviously, execution speeds are hardware dependant.  
> Trying to optimise execution time in Node, beyond choosing a better algorithm, is largely [an exercise in futility](https://gist.github.com/coolaj86/2310b00d6eebb3f752f4ca803f1423d1).  Execution times can vary widely following apparently unrelated changes in code which appear to modify how the v8 JIT decides to compile the code.


|Algo|Input Length|Python|JS / Node|Rust|
|---|---:|---:|---:|---:|
|naive Count|10_000|2216ms|928ms|7ms|
|sort then walk|10_000|4.4ms|6.0ms|0.85ms|
|Binary Search|10_000|tbc|4.2ms|tbc|
|naive Count|100_000|--|~70 s|742ms|
|sort then walk|100_000|65ms|62ms|8.6ms|
|Binary Search|100_000|tbc|37ms|tbc|
|sort then walk|1_000_000|1112ms|504ms|127ms|
|Binary Search|1_000_000|tbc|385ms|tbc|
|sort then walk|10_000_000|13145ms|5457ms|1711ms|
|Binary Search|10_000_000|tbc|3934ms|tbc|


Performance became an interesting part of this problem and, as I had already built simple solutions in JavaScript and Python, this seemed like a good opportunity to engage with my first statically typed language.  So, there are implementations of each potential solution in:
- [JS/Node](https://github.com/DBBrowne/code-challenges-public/blob/main/other/fuzzybinarysearch/)
- [Python](https://github.com/DBBrowne/code-challenges-public/blob/main/other/fuzzybinarysearch/)
- [Rust](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search)

Please see [the JS and Python implementations](https://github.com/DBBrowne/code-challenges-public/blob/main/other/fuzzybinarysearch/) in my general code-challenges repo.

This was one of the first coding challenges I saw which required a bit of comp-sci knowledge, so I failed to see that this was a binary search problem.  Additionally, the input element value space (0 <= input[i] <= 1e9) being much larger than the input length space (2< i <=1e5) made a hashmap-style count solution almost as slow as the naive approach, and helped me fail to realise that there was a Fuzzy Binary Search solution.

Many thanks to @coolaj86 ([Github](https://github.com/coolaj86) / [Twitter](https://twitter.com/coolaj86) for his help, advice, and friendly competition.
Check him out on youtube, optimizing solutions here:
https://www.youtube.com/watch?v=0mmi44ZB2C0
And walking me through a better solution here:
https://www.youtube.com/watch?v=BLrCXtbmG4Q

AJ's fuzzy binary search solution code is in [this Gist](https://gist.github.com/coolaj86/2310b00d6eebb3f752f4ca803f1423d1).


The simple, naive implementations is:
```rust
fn counts(team_a: Vec<u32>, team_b:Vec<u32>)->Vec<u32>{
  let mut output : Vec<u32> = Vec::with_capacity(team_b.len());

  for b in team_b {
    let mut counter = 0;
    for a in &team_a {
      if a<=&b {
        counter = counter + 1
      }
    };
    output.push(counter);
  };
  output
}
```

This solution has O(N^2) scaling however. As we move above input lengths of 10000, this becomes completely unmanageable.

I didn't realise that this was a binary-search problem, or that binary-search would be fast enough to avoid the need to sort both arrays, so sought to reduce the scaling in other ways.
Sorting the inputs, then walking through them offered the opportunity to reduce complexity to O(2logN + 2N) = O(N).  A great improvement,  but still much slower than a fuzzy binary search.

In more detail:
- Copy `inputB` (so we can sort but still retain order)
- For each element `j` of sortedB, walk through A until the value of `A[i]` exceeds `B[j]`.  
- The index of our walker will be the count of A with lower values than our target B, so record this in a HashMap `[B[j]:i]`.
- Continue walking until end of A or value of `A` exceeds `max(B)`
- Replace each value in our un-sorted inputB with the relevant value from the hashmap.
- Return the modified `inputB`

This isn't terrible time-wise, more than fast enough to pass the timeout limit on the test, but certainly not as fast as a fully-correct implementation.

The "Correct" solution is to:
- sort inputA
- search A for each element of inputB with a fuzzy binary search
>(ie returns index of nearest match to each element of inputB (walking right to find max index = max count of lower scores).
- Pre-allocating the output array offers a significant speed up at larger input sizes.


### Usage
Install Rust if necessary. Webi offers the easiest, most reliable way to do this: https://webinstall.dev/rustlang/

```console
curl -sS https://webinstall.dev/rustlang | bash
```

Build and run the app:
```console
cargo build --release
./target/release/obtuse_interview_scores
>>>
<Assertions>      // will panic if failing
<Execution times> // for different functions and input scales, from 10k to 10m.
```

### Running Tests

This whole app is a test!

### Known Issues

"Correct" solution sorts inputA, then searches A for each element of inputB with a fuzzy binary search (ie returns index of nearest match to each element of inputB (walking right to find max index = max count of lower scores).