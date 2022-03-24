'use strict'

function compareAscending(a, b) {
  return a - b
}

function duncanCount(teamA, teamB) {
  // shallow copy teamB:
  const _teamB = [...teamB]

  // sort numerically
  teamA.sort(compareAscending)
  _teamB.sort(compareAscending)

  const cache = {}
  let previousAIndex = 0

  _teamB.reduce((previousMatches, score) => {
    while (teamA[previousAIndex] <= score) {
      previousMatches++
      previousAIndex++
    }
    cache[score] = previousMatches
    return previousMatches
  }, 0)

  teamB.forEach((score, i) => {
    teamB[i] = cache[score]
  })
  return teamB
}

function ajCountLeftAlloc(inputs, refs) {
  const outputs = []
  // starts just a little to the right
  inputs.sort(compareAscending)

  refs.forEach(function (ref) {
    let left = 0
    let right = inputs.length - 1
    let i = left

    for (;;) {
      const v = inputs[i]
      // Move to the left
      // (guarantee that i will move left if it can)
      if (v > ref) {
        right = i //- 1;
        const j = left + Math.floor((i - left) / 2)
        if (i === j) {
          break
        }
        i = j
        continue
      }

      if (v <= ref) {
        left = i
        const j = i + Math.floor((right - i) / 2)
        if (i === j) {
          break
        }
        i = j
        continue
      }
    }
    while (inputs[i] === inputs[i + 1] && i < inputs.length) {
      i += 1
    }
    outputs.push(i + 1)
  })
  return outputs
}

function binarySearchThenWalk(arr, x){
  let left = 0
  let right = arr.length
  // let mid = left + Math.floor((right - left) / 2)

  while (left <= right){
    let mid = left + Math.floor((right - left) / 2)
    
    // Standard binary search would return mid here, if arr[mid] === x, 
    // As we want to find the rightmost index if multiple equal values are sorted together, walk right from a  matching value until a non-matching is found.
    if (x === arr[mid]) {
      while (x === arr[mid + 1]){
        mid++
      }
      return mid + 1
    }
    // If we're at the insert point for our target, return the next index
    if (arr[mid] < x && arr[mid + 1] > x){
      return mid + 1
    }

    if (arr[mid] < x){
      left = mid + 1
    } else {
      right = mid - 1
    }
  }
  // target is not found, so return 
  return right + 1
}
function ajCount(inputs, refs){
  const outputs = []
  inputs.sort(compareAscending)

  refs.forEach(function(ref){
    outputs.push(
      binarySearchThenWalk(inputs, ref)
    )
  })
  return outputs
}

// * Binary Search with negative insertion index if not found
// https://stackoverflow.com/a/29018745/15995918
function binaryFindOrInsertionIndex(arr, target, compareFn = (t, el) => t - el) {
  // Returns 0 if target found at arr[0].  
  // Returns -(indexToInsert) if target is not found
  var left = 0
  var right = arr.length - 1
  while (left <= right) {
    var mid = (right + left) >> 1
    var cmp = compareFn(target, arr[mid])
    if (cmp > 0) {
      left = mid + 1
    } else if (cmp < 0) {
      right = mid - 1
    } else {
      return mid
    }
  }
  return -right - 1
}
function binaryFindThenWalk(arr, target){
  let index = binaryFindOrInsertionIndex(arr, target)
  if (index < 0) return -index

  while (target === arr[index]) index++
  return index
}
function ajCountExtended(inputs, refs){
  inputs.sort(compareAscending)

  refs.forEach(function(ref, index){
    refs[index] = binaryFindThenWalk(inputs, ref)
  })
  return refs
}

// * Binary Bound
// https://stackoverflow.com/a/41956372/15995918
function binarySearch(array, pred) {
  let left = -1
  let right = array.length
  while ((1 + left) < right) {
    // Bitwise version of Math.floor((hi-lo) / 2)
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
function binaryBoundsCount(inputs, refs){
  inputs.sort(compareAscending)

  refs.forEach(function(ref, index){
    refs[index] = upperBound(inputs, ref)
  })
  return refs
}

// ***************************************
// *** Tests

const tests = [
  {
    inputs: [1, 4, 2, 4],
    refs: [3, 5],
    expected: [2, 4],
  },
  {
    inputs: [1, 2, 3],
    refs: [2, 4],
    expected: [2, 3],
  },
  {
    inputs: [2, 16, 6000000, 5, 1, 79, 250, 3],
    // [1, 2, 3, 5, 16, 79, 250, 6000000];
    refs: [5, 100],
    expected: [4, 6],
  },
  {
    inputs: [5, 100],
    // [1, 2, 3, 5, 16, 79, 250, 6000000];
    refs: [2, 16, 6000000, 5, 1, 79, 250, 3],
    expected: [0,1,2,1,0,1,2,0],
  }
]

const functions = [
  duncanCount,
  ajCount,
  ajCountExtended,
  binaryBoundsCount
]

functions.forEach(function(fn){
  console.info(fn.name)
  tests.forEach(function (test) {
    const a = test.inputs.slice(0)
    const b = test.refs.slice(0)
    const answer = fn(a, b)
    if (test.expected.toString() !== answer.toString()) {
      console.info('Fail', test.expected.toString(), answer.toString())
      return
    }
    console.info('Pass')
  })
})

const maxLength = 1e5
const maxScores = 1e9

const bench = [
  {
    inputs: 10_000,
    refs: 10_000,
  },
  {
    inputs: 100_000,
    refs: 100_000,
  },
  {
    inputs: 1_000_000,
    refs: 1_000_000,
  },
  {
    inputs: 10_000_000,
    refs: 10_000_000,
  },
  {
    inputs: Math.floor(Math.random() * maxLength),
    refs: Math.floor(Math.random() * maxLength),
  },
  {
    inputs: Math.floor(Math.random() * maxLength),
    refs: Math.floor(Math.random() * maxLength),
  }
]
function scoresGenerator(size) {
  const returnArray = []

  for (let i = 0; i < size; i += 1) {
    returnArray.push(Math.floor(Math.random() * maxScores))
  }

  return returnArray
}

const functionsToTime = [
  duncanCount,
  ajCountLeftAlloc,
  ajCount,
  ajCountExtended,
  binaryBoundsCount
]

bench.forEach(function (sizes) {
  console.info()
  console.info(sizes)
  const oinputs = scoresGenerator(sizes.inputs)
  const orefs = scoresGenerator(sizes.refs)
  let inputs
  let refs

  functionsToTime.forEach(function (fn){
    inputs = oinputs.slice(0)
    refs = orefs.slice(0)
    console.time(fn.name)
    fn(refs, inputs)
    console.timeEnd(fn.name)
  })
})
