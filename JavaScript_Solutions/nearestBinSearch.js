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

  _teamB.forEach(score => {
    // where previousAIndex > teamA.length, the while loop ends automatically as teamA[length] === undefined, and undefined<=int === false.
    while (teamA[previousAIndex] <= score) {
      previousAIndex++
    }
    cache[score] = previousAIndex
  })

  teamB.forEach((score, i) => {
    teamB[i] = cache[score]
  })
  return teamB
}

// * Binary Bound
// https://stackoverflow.com/a/41956372/15995918
function binarySearch(array, pred) {
  let left = -1
  let right = array.length
  while ((1 + left) < right) {
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
