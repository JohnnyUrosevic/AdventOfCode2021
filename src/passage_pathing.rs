use crate::get_input::get_input;

use std::collections::{HashMap, HashSet};

pub fn depth_first_search(graph: &HashMap<String, Vec<&str>>,
    start: String, seen: &mut HashSet<String>) -> u64 {
  if start == "end" {
    return 1;
  }

  if start.chars().all(char::is_lowercase) {
    seen.insert(start.clone());
  }

  let destinations = graph.get(&start.clone()).expect("Invalid Node");
  let mut paths = 0;

  for dest in destinations {
    let dest = dest.to_string();
    if seen.get(&dest).is_none() {
      paths += depth_first_search(&graph, dest, seen);
    }
  }
  
  // backtracking
  seen.remove(&start);
  
  paths
}

pub fn passage_pathing() -> (u64, u64) {
  let input = get_input(12).expect("Could not get input");
  
  let mut graph = HashMap::new();
  input.iter()
    .for_each(|e| {
      let mut endpoints = e.split('-');

      let start = endpoints.next().expect("Bad input");
      let end = endpoints.next().expect("Bad input");

      let destinations = graph.entry(start.to_string()).or_insert(vec![]);
      destinations.push(end); 

      // paths are bidirectional
      let destinations = graph.entry(end.to_string()).or_insert(vec![]);
      destinations.push(start); 
    });

  let mut seen = HashSet::new();
  let num_paths = depth_first_search(&graph, "start".to_string(), &mut seen);

  (num_paths, 0)
}
