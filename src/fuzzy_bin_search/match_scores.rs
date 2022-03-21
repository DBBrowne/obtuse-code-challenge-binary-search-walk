use rand::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

const MAX_SCORE:u32 = 1e9 as  u32;
//const MAX_MATCHES:usize = 1e6 as usize;


fn scores_generator(size: usize)-> Vec<u32>{
  let mut rng = rand::thread_rng();

  let mut array : Vec<u32> = Vec::with_capacity(size);
  for _ in 0..array.capacity() {
    array.push(rng.gen_range(0..MAX_SCORE));
  }
  array
}

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

fn timer(
  fnc:&dyn Fn(Vec<u32>,Vec<u32>)-> Vec<u32>,
  size: usize,
  label:String
){
  let team_a = scores_generator(size);
  let team_b = scores_generator(size);

  let now = Instant::now();
  fnc(team_a, team_b);
  let end = now.elapsed().as_micros() as f64;
  println!("{}: {}ms",label,  end/1000.0);
}

struct TestScenario{
  inputs: Vec<u32>,
  refs: Vec<u32>,
  expected: Vec<u32>,
}

pub fn match_scores() {
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
    assert_eq!(counts(scenario.inputs.to_vec(), scenario.refs.to_vec()), scenario.expected);
    assert_eq!(counts_sort_walk(scenario.inputs.to_vec(), scenario.refs.to_vec()), scenario.expected);
  }

  timer(&counts, 10_000, "counts 10k".to_string());
  timer(&counts_sort_walk, 10_000, "fast 10k".to_string());

  timer(&counts, 100_000, "counts 100k".to_string());
  timer(&counts_sort_walk, 100_000, "fast 100k".to_string());
  
  timer(&counts_sort_walk, 1_000_000, "fast 1m".to_string());
  timer(&counts_sort_walk, 10_000_000, "fast 10m".to_string());
}