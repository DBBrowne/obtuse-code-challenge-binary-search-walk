<!----><a name="title_link"></a>
# Obtuse technical tests - "football" scores

## Contents
- [Obtuse technical tests - "football" scores](#title_link)
- [Intro](#intro)
  - [Performance](#performance)
  - [Solutions](#solutions)
- [Usage](#usage)
- [Running Tests](#running-tests)
- [Known Issues](#known-issues)

### Intro
This project came from an obtuse interview question.  After failing the initial 1 hour test, I worked through the "sort-then-walk" solutions in JS and Python in about another 1.5-2 hours.

A better solution using a binary search exists however, which is much faster than my attempt.

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
// inputB[1] = 4, which is >= 3 members of inputA
```

#### Performance
A summary:
- Node does respectably, excellent given how easy it is to use.
> Trying to optimise execution time in Node, beyond choosing a better algorithm, is largely [an exercise in futility](https://gist.github.com/coolaj86/2310b00d6eebb3f752f4ca803f1423d1).  Execution times can vary widely following apparently unrelated changes in code which appear to modify how the v8 JIT decides to compile the code.

- Python is passable where it can use implementations written in C.  It can keep up with the v8 compiled Node as long as most of the work is being handled by those C implementations.
  - See `.sort()` and `bisect` ([`Sort Then Walk`](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/Python_Solutions/nearest_binary_search.py#L13-L34) and [`Binary Insertion Point`](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/Python_Solutions/nearest_binary_search.py#L108-L116)). 
  - If Python itself is handling the the binary search ([`Binary S then Walk`](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/Python_Solutions/nearest_binary_search.py#L81-L106)) then execution times are terrible.

- Rust is a compiled language, so dominates the performance competition.  For a low level language, it is very easy to work with, thanks largely to the extremely helpful compiler.
  - The poor relative execution times for the Binary Search based solutions in Rust seem to come from a branch prediction issue with the random input data.  Although [my StackOverflow question](https://stackoverflow.com/questions/71623880/translation-of-two-solutions-from-node-to-rust-leads-to-inversion-in-relative-ex) was rejected, the comments contained a useful [link to an explanation](https://stackoverflow.com/questions/11227809/why-is-processing-a-sorted-array-faster-than-processing-an-unsorted-array) of branch prediction and why processing sorted data can be much faster than unsorted.
  - Sorting the `refs` before passing them to the function cuts the execution time of the searches.  By the time both arrays are sorted, it is quicker to just walk through the arrays as in `Sort then walk` than to binary search for each member of `refs`.  My knowledge of binary mathematics is limited, so there may be much faster approaches.  [Branchless code here](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/src/bounded_bin_search/match_scores.rs#L164-L190).

  - HOWEVER.  The binary Search based solution opens the door to multi-threading. Rust supports race-safe multi-threading via the Rayon Crate.  By multithreading the binary searches, we get execution times down to 500ms at the 10m input length level.  Multi-thread the `inputs` sort, and we get to 350ms when both inputs have length of 10m.


- Go is a reasonable balance of usability and speed.  There may well be a lot of remaining on the table however, especially as the multithreading is not currently offering the expected acceleration.
  - Goroutine setup overhead is quite large.  Perhaps a limited number of goroutines, each assigned a portion of the search work, would be more optimal.
  - Learning how to use channels may be the appropriate route forwards here.

|Algo|Input Length||Python|Go|JS / Node|Rust||
|---|---:|---|---:|---:|---:|---:|---:|
|Naive Count|10_000||2216ms|44ms|928ms|7ms|
|Sort Then Walk|10_000||3.6ms|2.5ms|6.0ms|0.85ms|duncanCount|
|Binary S then Walk|10_000||19ms||4.2ms|0.7ms|ajCount|
|Binary Insertion Point|10_000||3.2ms|1.6ms|4.0ms|0.8ms|binaryBoundsCount|
|Binary Insertion Multi|10_000||---|3.45ms|---|0.23ms|counts_partition_multithread|
||||||
|Naive Count|100_000||--||~70 s|742ms|
|Sort Then Walk|100_000||57ms|30.2ms|62ms|8.6ms|
|Binary S then Walk|100_000||270ms||37ms|9.7ms|
|Binary Insertion Point|100_000||44ms|22ms|29.5ms|9.3ms|
|Binary Insertion Multi|100_000||---|31ms|---|1.4ms||
|||||||
|Sort Then Walk|1_000_000||946ms|397ms|795ms|127ms|
|Binary S then Walk|1_000_000||3632ms||492ms|133ms|
|Binary Insertion Point|1_000_000||967ms|279ms|407ms|130ms|
|Binary Insertion Multi|1_000_000||---|329ms|---|16ms||
|||||||
|Sort Then Walk|10_000_000||11,031ms|4825ms|9126ms|1711ms|
|Binary S then Walk|10_000_000||46,000ms||7213ms|2700ms|
|Binary Insertion Point|10_000_000||16,200ms|4794ms|6080s|2710ms|
|Binary Insertion Multi|10_000_000||---|3566ms|---|348ms||
|||||||

> Obviously, execution times are hardware dependant.  

> There's a good argument that we should not be mutating the inputs in any of these solutions, but instead creating local copies of the sorted arrays in each case.  There original question made no specification here, so as long as we're consistent, sorting the original array is deemed acceptable as the point here is to compare solutions.


#### Solutions
Performance became an interesting part of this problem and, as I had already built simple solutions in JavaScript and Python, this seemed like a good opportunity to engage with my first statically typed language.  So, there are implementations of each potential solution in:
- [JS/Node](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/JavaScript_Solutions/nearestBinSearch.js)
- [Python](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/Python_Solutions/nearest_binary_search.py)
- [Rust](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search)
- [Go](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/Golang_Solutions/main.go)

There are more workings and variations on the [JS and Python implementations](https://github.com/DBBrowne/code-challenges-public/blob/main/other/fuzzybinarysearch/) in my general code-challenges repo.

This was one of the first coding challenges I saw which required a bit of comp-sci knowledge, so I failed to identify that this was a binary search problem.  
The input element value space (0 <= input[i] <= 1e9) being much larger than the input length space (2< i <=1e5) would have made a hashmap-style count solution almost as slow as the naive approach, and helped me fail to realise that there was a Binary Search solution.

Many thanks to @coolaj86 ([Github](https://github.com/coolaj86) / [Twitter](https://twitter.com/coolaj86)) for his help, advice, and friendly competition.  
Check him out on youtube, optimizing solutions here:  
https://www.youtube.com/watch?v=0mmi44ZB2C0  
And walking me through a better solution here:  
https://www.youtube.com/watch?v=BLrCXtbmG4Q  

AJ's binary search then walk approach is demonstrated in [nearestBinSearch.js :: ajCount](https://github.com/DBBrowne/obtuse-code-challenge-fuzzy-binary-search/blob/main/JavaScript_Solutions/nearestBinSearch.js), and is only beaten by the later discovery of a way to use binary search to discover the upper bound of matching values.


The simple, naive implementations is:
```
  for each element in teamB, count members of teamA with value < element
```
|Rust|JavaScript|
|:---|:---|
|<pre>fn counts(team_a: Vec<u32>, team_b:Vec<u32>)->Vec<u32>{<br>  let mut output : Vec<u32> = Vec::with_capacity(team_b.len());<br>  for b in team_b {<br>    let mut counter = 0;<br>    for a in &team_a {<br>      if a<=&b {<br>        counter = counter + 1<br>      }<br>    };<br>    output.push(counter);<br>  };<br>  output<br>}</pre>|<pre>function counts(teamA, teamB){<br>  return teamB.map(scoreB=>{<br>    return (<br>      teamA.filter(scoreA=>{<br>        return scoreA <= scoreB<br>      }).length<br>    )<br>  })<br>}</pre>|

This solution has O(N^2) scaling however. As we move above input lengths of 10_000, this becomes completely unmanageable.

I didn't realise that this was a binary-search problem, or that binary-search would be fast enough to avoid the need to sort both arrays, so sought to reduce the scaling in other ways.
Sorting the inputs, then walking through them offered the opportunity to reduce complexity to O(2NlogN + 2N) = O(NlogN).  A great improvement,  but still much slower than a binary search.

In more detail:
- Copy `inputB` (so we can sort but still retain order)
- For each element `j` of sortedB, walk through A until the value of `A[i]` exceeds `B[j]`  
- The index of our walker will be the count of A with lower values than our target B, so record this in a HashMap `[B[j]:i]`
- Continue walking until end of A or value of `A` exceeds `max(B)`
- Replace each value in our un-sorted inputB with the relevant value from the hashmap
- Return the modified `inputB`

This isn't terrible time-wise.  It's more than fast enough to pass the timeout limit on the original test, but certainly not as fast as a fully-correct implementation.

The "Correct" solution is to:
- sort inputA
- search A for each element of inputB with a binary search, then walk to the end of any matching groups
>(ie returns index of nearest match to each element of inputB (walking right to find max index = max count of lower scores).

After further research, a method to continue the binary search to find the insertion point / upper bound of matching elements exists, offering the fastest solution to this problem:
https://stackoverflow.com/a/41956372/15995918

```js
function binarySearch(array, pred) {
  let left = -1
  let right = array.length
  while ((1 + left) < right) {
    // left + (right-left)/2 is used to avoid overflow issue with large input lengths
    // Bitshift version of Math.floor((hi-lo) / 2)
    const mid = left + ((right - left) >> 1)
    if (pred(array[mid])) {
      right = mid
    } else {
      left = mid
    }
  }
  return right
}
function upperBound(array, target) {
  return binarySearch(array, j => target < j)
}
function binarySearchBounds(inputs, refs){
  inputs.sort(compareAscending)

  return refs.map(function(ref){
    return upperBound(inputs, ref)
  })
}
```

### Usage
#### Rust
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
#### JavaScript
```console
node ./JavaScript_Solutions/nearestBinSearch.js
```

#### Python
```console
python3 ./Python_Solutions/nearest_binary_search.py
```

#### Go

```console
./Golang_Solutions/obtuse_interview_go
```

Or re-build with 
```console
pushd ./Golang_Solutions
go build

./obtuse_interview_go
```

### Running Tests

This whole app is a test!

### Known Issues

- [Branch prediction failures](https://stackoverflow.com/questions/11227809/why-is-processing-a-sorted-array-faster-than-processing-an-unsorted-array) lead to very slow execution for the binary search approach.  A viable workaround would be useful knowledge.

- Go multithreading runs slower than non-multithreaded. Goroutine setup overhead is quite large.  Need to spread operations over a limited number of threads?
- Go changing order of functions in benchmarks affects execution times, which it should not.

- Julia solution?