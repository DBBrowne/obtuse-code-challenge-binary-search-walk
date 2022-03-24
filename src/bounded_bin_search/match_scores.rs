use rand::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

const MAX_SCORE:u32 = 1e9 as  u32;
const MAX_MATCHES:u32 = 1e6 as u32;

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

fn counts_sort_walk(mut team_a: Vec<u32>, mut team_b:Vec<u32>)->Vec<u32>{
  team_a.sort_unstable();
  let mut _tb = team_b.to_vec();
  _tb.sort_unstable();
  let mut cache = HashMap::new();
  cache.insert(_tb[0], 0);
  let mut previous_a_index = 0;

  for b in _tb{
    while
      team_a.len() > previous_a_index && 
      &team_a[previous_a_index] <= &b
    {
      previous_a_index = previous_a_index + 1;
    };
    cache.insert(b, previous_a_index as u32);
  };
  
  for b in &mut team_b{
    *b = cache[&*b];
  };
  team_b
}

// * Binary Search with negative insertion index if not found
// https://stackoverflow.com/a/29018745/15995918
fn binary_search_insert_u32(
  arr: &Vec<u32>,
  target: u32,
  pred: &dyn Fn(u32, u32)->i32,
)-> i32 {
  let mut left: i32 = 0;
  let mut right: i32 = arr.len() as i32 - 1;
  while left <= right {
    let mid = left + ((right-left) >>1);
    let cmp = pred(target, arr[mid as usize]);

    if cmp>0 {
      left = mid + 1;
    } else if cmp<0{
      right = mid -1;
    } else{
      return mid
    }
  };

  -(right)-1
}
fn binary_find_then_walk(arr: &Vec<u32>, target: u32)->u32{
  let predicate = |t, el| {t as i32 - el as i32};
  let arr_length = arr.len() as i32;
  let mut index:i32 = binary_search_insert_u32(
    arr,
    target,
    &predicate
  );

  if index < 0 {
    return -index as u32
  };
  while target == arr[index as usize] && index < arr_length{
    index = index +1
  };

  index as u32
}
fn count_find_then_walk(mut inputs:Vec<u32>, refs: Vec<u32>)->Vec<u32>{
  inputs.sort_unstable();

  refs.into_iter().map(|r| {
    binary_find_then_walk(&inputs, r)
  }).collect::<Vec<u32>>()
}

// * Binary Bound
// https://stackoverflow.com/a/41956372/15995918
fn binary_search_u32(
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
fn upper_bound(arr: &Vec<u32>, target:u32)->usize{
  let predicate = |j|{target < j};
  binary_search_u32(
    &arr, 
    &predicate
  )
}
fn binary_bounds_count(mut inputs:Vec<u32>, refs:Vec<u32>)->Vec<u32>{
  inputs.sort_unstable();

  refs.into_iter().map(|r| {
    upper_bound(&inputs, r)  as u32
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
    array.push(rng.gen_range(0..MAX_SCORE));
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

pub fn match_scores_tests() {
  let mut rng = rand::thread_rng();

  let functions:[&dyn Fn(Vec<u32>,Vec<u32>)-> Vec<u32>; 3] = [
    &counts_sort_walk,
    &binary_bounds_count,
    &count_find_then_walk,
  ];
  let function_labels: [String;3] = [
    String::from("counts_sort_walk"),
    String::from("count_binary_bounds"),
    String::from("binary_find_then_walk"),
  ];

  let scenarios:Vec<TestScenario> = vec![
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
      inputs:10_000,
      refs:  10_000,
    },
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
    BulkTest{
      inputs: rng.gen_range(0..MAX_MATCHES) as usize,
      refs:rng.gen_range(0..MAX_MATCHES) as usize,
    },
    BulkTest{
      inputs:rng.gen_range(0..MAX_MATCHES) as usize,
      refs:rng.gen_range(0..MAX_MATCHES) as usize,
    },
  ];

  let mut now = Instant::now();
  scores_generator(100_000);
  let mut end = now.elapsed().as_micros() as f64;
  println!("Generator 100k: {}ms", end/1000.0);
  now = Instant::now();
  scores_generator(1_000_000);
  end = now.elapsed().as_micros() as f64;
  println!("Generator 1m: {}ms", end/1000.0);

  for scenario in scenarios{
    println!("Asserting: {:?}, {:?}, Expected: {:?}", scenario.inputs, scenario.refs, scenario.expected);
    for function in functions{
      assert_eq!(function(scenario.inputs.to_vec(), scenario.refs.to_vec()), scenario.expected);
    }
  }
  println!();
  timer(
    &counts,
    scores_generator(1_000),
    scores_generator(1_000),
    String::from("counts 1k"),
  );
  timer(
    &counts,
    scores_generator(10_000),
    scores_generator(10_000),
    String::from("counts 10k"),
  );
  timer(
    &counts,
    scores_generator(100_000),
    scores_generator(100_000),
    String::from("counts 100k"),
  );
  
  for scenario in bench{
    println!();
    println!("{:?}",scenario);

    let inputs = scores_generator(scenario.inputs);
    let refs = scores_generator(scenario.refs);

    for (index, func) in functions.iter().enumerate() {
      timer(
        &func,
        inputs.to_vec(),
        refs.to_vec(),
        String::from(&function_labels[index]),
      )
    }
  }
}
