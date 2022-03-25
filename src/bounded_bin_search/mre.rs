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
  cache.insert(_tb[0], 0);
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


// * Partition
fn counts_partition(mut inputs:Vec<u32>, refs: Vec<u32>)->Vec<u32>{
  let mut output : Vec<u32> = Vec::with_capacity(refs.len());
  inputs.sort_unstable();

  for r in refs{
    output.push(inputs.partition_point(|&el| el <= r) as u32);
  };
  output
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
  let functions:[(&dyn Fn(Vec<u32>,Vec<u32>)-> Vec<u32>, String); 2] = [
    (&counts_sort_walk, String::from("counts_sort_walk")),
    (&counts_partition, String::from("count_partition"))
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

  for scenario in scenarios{
    println!("Asserting: {:?}, {:?}, Expected: {:?}", scenario.inputs, scenario.refs, scenario.expected);
    for func in &functions{
      assert_eq!(func.0(scenario.inputs.to_vec(), scenario.refs.to_vec()), scenario.expected);
    }
  }
  println!();
  
  for scenario in bench{
    println!();
    println!("{:?}",scenario);

    let inputs = scores_generator(scenario.inputs);
    let refs = scores_generator(scenario.refs);

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
