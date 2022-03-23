function compareFn(a, b) {
  return a - b
}

function binarySearch(arr, target, compareFn = (t, el) => t - el) {
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
function binaryWalk(arr, x){
  let index = binarySearch(arr, x)
  if (index < 0) return -index
  while (x === arr[index]) index++
  return index
}
function binarySearchBounds(inputs, refs){
  inputs.sort(compareFn)

  return refs.map(function(ref){
    return binaryWalk(inputs, ref)
  })
}

// https://stackoverflow.com/a/41956372/15995918
function boundSearch(array, predicate) {
  let lo = -1, hi = array.length
  while ((1 + lo) < hi) {
    // Bitwise version of Math.floor((hi-lo) / 2)
    const mi = lo + ((hi - lo) >> 1)
    if (predicate(array[mi])) {
      hi = mi
    } else {
      lo = mi
    }
  }
  return hi
}
function upperBound(array, item) {
  return boundSearch(array, function(j) {
    return item < j
  })
}
function boundedCount(inputs, refs){
  inputs.sort(compareFn)

  return refs.map(function(ref){
    return upperBound(inputs, ref)
  })
}

function binaryBound(arr, x){
  let left = -1
  let right = arr.length

  while ((1 + left) < right){
    const mid = left + Math.floor((right - left) / 2)
    if (x < arr[mid]){
      right = mid
    } else {
      left = mid
    }
  }
  return right
}
function binaryBoundMap(inputs, refs){
  inputs.sort(compareFn)

  return refs.map(function(ref){
    return binaryBound(inputs, ref)
  })
}


// ***************************************
// *** Tests

const tsts = [
  {
    inputs: [1, 4, 2, 4],
    refs: [3, 5],
    expected: [2, 4],
  },
  {
    inputs: [1, 4, 2, 4],
    refs: [3, 4],
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

const fns = [
  boundedCount,
  binaryBoundMap,
  binarySearchBounds
]

fns.forEach(function(fn){
  console.info(fn.name)
  tsts.forEach(function (test) {
    const a = test.inputs.slice(0)
    const b = test.refs.slice(0)
    const answer = fn(a, b)
    if (test.expected.toString() !== answer.toString()) {
      console.info('Fail', a, b, test.expected.toString(), answer.toString())
      return
    }
    console.info('Pass', a, b, test.expected.toString(), answer.toString())
  })
})