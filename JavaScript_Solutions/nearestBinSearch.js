'use strict'

function sortFn(a, b) {
  return a - b
}

function ajCount(refs, inputs) {
  const outputs = []

  // starts just a little to the right
  const m = Math.floor(inputs.length / 2)
  inputs.sort(function (a, b) {
    return a - b
  })
  refs.forEach(function (ref) {
    let i = m
    let left = 0
    let right = inputs.length - 1

    for (;;) {
      const v = inputs[i]
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

function ajCountLeftAlloc(refs, inputs) {
  const outputs = []
  // starts just a little to the right
  inputs.sort(sortFn)

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

// eslint-disable-next-line no-unused-vars
function ajCountLeftAlloc2(refs, inputs) {
  const outputs = []

  // starts just a little to the right
  inputs.sort(sortFn)
  
  refs.forEach(function (ref) {
    let left = 0
    let right = inputs.length - 1
    let i = left

    // debug
    // let count = 0

    for (;;) {
      // debug
      // if (count > 10) {
      // break
      // }
      // count += 1

      const v = inputs[i]

      // Move to the left
      // (guarantee that it will move left if it can)
      if (v > ref) {
        right = i //- 1;
        const j = left + Math.floor((i - left) / 2)
        if (i === j) {
          break
        }
        i = j
        continue
      }

      // Move to the right
      if (v <= ref) {
        left = i
        if (i === right) {
          break
        }
        let j = i + Math.floor((right - i) / 2)
        if (i === j) {
          j += 1
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

function duncanCount(teamA, teamB) {
  // shallow copy teamB:
  const _teamB = [...teamB]

  // sort numerically
  teamA.sort(sortFn)
  _teamB.sort(sortFn)

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

function eachCount(teamA, teamB) {
  // shallow copy teamB:
  const _teamB = teamB.slice(0)

  // sort numerically
  teamA.sort(sortFn)
  _teamB.sort(sortFn)

  const cache = {}
  let previousAIndex = 0

  _teamB.forEach((score) => {
    while (teamA[previousAIndex] <= score) {
      previousAIndex++
    }
    cache[score] = previousAIndex
    return previousAIndex
  })

  teamB.forEach((score, i) => {
    teamB[i] = cache[score]
  })
  return teamB
}

function forCount(teamA, teamB) {
  // shallow copy teamB:
  const _teamB = teamB.slice(0)

  // sort numerically
  teamA.sort(sortFn)
  _teamB.sort(sortFn)

  const cache = {}
  let previousAIndex = 0
  for (const score of _teamB) {
    while (teamA[previousAIndex] <= score) {
      previousAIndex++
    }
    cache[score] = previousAIndex
  }

  teamB.forEach((score, i) => {
    teamB[i] = cache[score]
  })
  return teamB
}

// https://stackoverflow.com/a/41956372/15995918
function binarySearch(array, pred) {
  let lo = -1, hi = array.length
  while ((1 + lo) < hi) {
    // Bitwise version of Math.floor((hi-lo) / 2)
    const mi = lo + ((hi - lo) >> 1)
    if (pred(array[mi])) {
      hi = mi
    } else {
      lo = mi
    }
  }
  return hi
}
function upperBound(array, item) {
  return binarySearch(array, j => item < j)
}

function binaryCountMap(inputs, refs){
  
  inputs.sort(sortFn)

  return refs.map(function(ref){
    return upperBound(inputs, ref)
  })
}
function binaryCount(inputs, refs){
  const outputs = []
  
  inputs.sort(sortFn)

  refs.forEach(function(ref){
    outputs.push(
      upperBound(inputs, ref)
    )
  })

  return outputs
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
  forCount,
  ajCount,
  binaryCount,
  binaryCountMap
  // ajCountLeftAlloc2
  // ajCountLeftAlloc3
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

console.log('AJ reversed inputs');
[
  ajCount
  // ajCountLeftAlloc2
].forEach(function (fn) {
  console.info(fn.name)
  tests.forEach(function (test) {
    const a = test.inputs.slice(0)
    const b = test.refs.slice(0)
    const answer = fn(b, a)
    if (test.expected.toString() !== answer.toString()) {
      console.info('Fail', test.expected.toString(), answer.toString())
      return
    }
    console.info('Pass')
  })
})
const maxLength = 1e5
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

const maxScores = 1e9
function scoresGenerator(size) {
  const returnArray = []

  for (let i = 0; i < size; i += 1) {
    returnArray.push(Math.floor(Math.random() * maxScores))
  }

  return returnArray
}

const functionsToTime = [
  ajCount,
  ajCountLeftAlloc,
  duncanCount,
  eachCount,
  forCount,
  binaryCount,
  binaryCountMap
  // ajCountLeftAlloc2
  // ajCountLeftAlloc3
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

// function upperBound(array, item) {
//   return binarySearch(array, j => item < j)
// }
// function lowerBound(array, item) {
//   return binarySearch(array, j => item <= j)
// }
// const exampleTarget = 2
// console.log(binarySearch(
//   [1,2,4,4], 
//   function(j){
//     0 <= sortFn(j, 1)
//   })
// )