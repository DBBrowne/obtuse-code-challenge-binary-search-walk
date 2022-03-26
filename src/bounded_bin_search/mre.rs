use rand::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

const MAX_VALUE:u32 = 1e9 as  u32;
// const MAX_INPUT_LENGTH:u32 = 1e6 as u32;

fn counts_sort_walk(mut inputs: Vec<u32>, mut refs:Vec<u32>)->Vec<u32>{
  inputs.sort_unstable();
  let mut _tb = refs.to_vec();
  _tb.sort_unstable();
  let mut cache = HashMap::new();
  // cache.insert(_tb[0], 0);
  let mut previous_a_index = 0;

  for b in _tb{
    while
      inputs.len() > previous_a_index && 
      &inputs[previous_a_index] <= &b
    {
      previous_a_index = previous_a_index + 1;
    };
    cache.insert(b, previous_a_index as u32);
  };
  
  for r in &mut refs{
    *r = cache[&*r];
  };
  refs
}

// * Partition Branch prediction issue
// https://stackoverflow.com/questions/11227809/why-is-processing-a-sorted-array-faster-than-processing-an-unsorted-array/11227902#11227902
fn counts_partition_branch_pred_issue(mut inputs:Vec<u32>, refs: Vec<u32>)->Vec<u32>{
  let mut output : Vec<u32> = Vec::with_capacity(refs.len());
  inputs.sort_unstable();

  for r in refs{
    output.push(inputs.partition_point(|&el| el <= r) as u32);
  };
  output
}



// * Partition
// Obviously slower than counts_sort_walk, because it's doing all the same sorting and caching, but then also doing a binary search
fn counts_partition(mut inputs:Vec<u32>, refs: Vec<u32>)->Vec<u32>{
  let mut output : Vec<u32> = Vec::with_capacity(refs.len());
  inputs.sort_unstable();
  let mut _refs = refs.to_vec();
  _refs.sort_unstable();

  let mut cache = HashMap::new();

  for r in _refs{
    cache.insert(r,inputs.partition_point(|&el| el <= r) as u32);
  };
  
  for r in refs{
    output.push(cache[&r]);
  };
  output
}

fn binary_upper_bound_u32(
  arr:&Vec<u32>, 
  target: u32
)->u32{
  let mut left:i32 = -1;
  let mut right:i32 = arr.len() as i32;
  while (1+left) < right {
    // Bitshift version of Math.floor((hi-lo) / 2)
    let distance_to_mid = right-left >> 1;
    let mid: i32 = left + distance_to_mid;
    let cmd = ((target - arr[mid as usize]) >> 31) as i32;
    // println!("{} {} {} {} ", cmd,!cmd, !cmd+2, !(cmd-1));
    if cmd >0 {
      right = mid;
    } else {
      left = mid;
    }
  }
  right as u32
}
fn counts_branchless_handle(inputs:Vec<u32>, refs:Vec<u32>)->HashMap<u32, u32>{
  let mut cache = HashMap::new();

  for r in refs{
    cache.insert(r, binary_upper_bound_u32(&inputs, r));
  };

  cache
}
fn counts_branchless(mut inputs:Vec<u32>, refs:Vec<u32>)->Vec<u32>{
  inputs.sort_unstable();
  let mut _tb = refs.to_vec();
  _tb.sort_unstable();

  let cache = counts_branchless_handle(inputs, _tb);

  refs
  // refs.into_iter().map(|r| {
  //   cache[&r]
  // }).collect()
}


// * Binary Bound
// https://stackoverflow.com/a/41956372/15995918
fn binary_search_old(
  arr:&Vec<u32>, 
  pred: &dyn Fn(u32)->bool,
)->usize{
  let mut left:i32 = -1;
  let mut right:i32 = arr.len() as i32;
  while (1+left) < right {
    // Bitshift version of Math.floor((hi-lo) / 2)
    let mid: i32 = left + ((right -left) >> 1);
    if pred(arr[mid as usize]) {
      right = mid
    } else {
      left = mid
    }
  }
  right as usize
}
fn upper_bound_old(arr: &Vec<u32>, target:u32)->usize{
  let predicate = |j|{target < j};
  binary_search_old(
    &arr, 
    &predicate
  )
}
fn binary_bounds_count_old(mut inputs:Vec<u32>, refs:Vec<u32>)->Vec<u32>{
  inputs.sort_unstable();

  refs.into_iter().map(|r| {
    upper_bound_old(&inputs, r)  as u32
  }).collect::<Vec<u32>>()
}

// ****************************************************
// *** Tests

fn timer(
  fnc:&dyn Fn(Vec<u32>,Vec<u32>)-> Vec<u32>,
  inputs: Vec<u32>,
  refs: Vec<u32>,
  label: String
){
  let now = Instant::now();
  fnc(inputs, refs);
  let end = now.elapsed().as_micros() as f32;
  println!("{}: {}ms",label,  end/1000.0);
}

fn scores_generator(size: usize)-> Vec<u32>{
  let mut rng = rand::thread_rng();

  let mut array : Vec<u32> = Vec::with_capacity(size);
  for _ in 0..array.capacity() {
    array.push(rng.gen_range(0..MAX_VALUE));
  }
  array
}

struct TestScenario{
  inputs: Vec<u32>,
  refs: Vec<u32>,
  expected: Vec<u32>,
}
#[derive(Debug)]
struct BulkTest{
  inputs: usize,
  refs: usize,
}

pub fn compare_arrays() {
  let functions:[(&dyn Fn(Vec<u32>,Vec<u32>)-> Vec<u32>, String); 4] = [
    (&counts_sort_walk, String::from("counts_sort_walk")),
    (&counts_partition, String::from("counts_partition")),
    (&counts_partition_branch_pred_issue, String::from("counts_partition with branch prediction issue")),
    (&counts_branchless, String::from("counts_branchless"))
  ];

  let scenarios = vec![
    TestScenario {
      inputs: vec![1, 4, 2, 4],
      refs: vec![3, 5],
      expected: vec![2, 4],
    },
    TestScenario {
      inputs: vec![1, 2, 3],
      refs: vec![2, 4],
      expected: vec![2, 3],
    },
    TestScenario {
      inputs: vec![2, 16, 6000000, 5, 1, 79, 250, 3],
      // [1, 2, 3, 5, 16, 79, 250, 6000000];
      refs: vec![5, 100],
      expected: vec![4, 6],
    },
    TestScenario {
      inputs: vec![5, 100],
      // [1, 2, 3, 5, 16, 79, 250, 6000000];
      refs: vec![2, 16, 6000000, 5, 1, 79, 250, 3],
      expected: vec![0,1,2,1,0,1,2,0],
    }
  ];

  let bench:Vec<BulkTest> = vec![
    BulkTest{
      inputs:100_000,
      refs:  100_000,
    },
    BulkTest{
      inputs:1_000_000,
      refs:  1_000_000,
    },
    BulkTest{
      inputs:10_000_000,
      refs:  10_000_000,
    },
  ];

  // for scenario in scenarios{
  //   println!("Asserting: {:?}, {:?}, Expected: {:?}", scenario.inputs, scenario.refs, scenario.expected);
  //   for func in &functions{
  //     assert_eq!(func.0(scenario.inputs.to_vec(), scenario.refs.to_vec()), scenario.expected);
  //   }
  // }
  println!();
  
  for scenario in bench{
    println!();
    println!("{:?}",scenario);

    let inputs = scores_generator(scenario.inputs);
    let refs = scores_generator(scenario.refs);

    // inputs.sort();
    // refs.sort();

    for func in &functions {
      timer(
        &func.0,
        inputs.to_vec(),
        refs.to_vec(),
        String::from(&func.1),
      )
    }
  }
}
