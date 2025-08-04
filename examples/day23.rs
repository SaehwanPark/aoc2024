use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(content: &str) -> HashMap<String, HashSet<String>> {
  let mut graph = HashMap::new();

  for line in content.lines().filter(|line| !line.is_empty()) {
    let parts: Vec<&str> = line.split('-').collect();
    if parts.len() == 2 {
      let a = parts[0].to_string();
      let b = parts[1].to_string();

      graph
        .entry(a.clone())
        .or_insert_with(HashSet::new)
        .insert(b.clone());
      graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }
  }

  graph
}

fn find_triangles(graph: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
  let mut triangles = HashSet::new();

  for (node_a, neighbors_a) in graph {
    let neighbors_vec: Vec<_> = neighbors_a.iter().collect();

    // check all pairs of neighbors of node_a
    for i in 0..neighbors_vec.len() {
      for j in (i + 1)..neighbors_vec.len() {
        let node_b = neighbors_vec[i];
        let node_c = neighbors_vec[j];

        // check if node_b and node_c are connected
        if let Some(neighbors_b) = graph.get(node_b) {
          if neighbors_b.contains(node_c) {
            // we have a triangle: node_a, node_b, node_c
            let mut triangle = vec![node_a.clone(), node_b.clone(), node_c.clone()];
            triangle.sort();
            triangles.insert(triangle);
          }
        }
      }
    }
  }

  triangles
}

fn count_triangles_with_t(triangles: &HashSet<Vec<String>>) -> usize {
  triangles
    .iter()
    .filter(|triangle| triangle.iter().any(|name| name.starts_with('t')))
    .count()
}

fn bron_kerbosch(
  r: &mut HashSet<String>,
  p: &mut HashSet<String>,
  x: &mut HashSet<String>,
  graph: &HashMap<String, HashSet<String>>,
  cliques: &mut Vec<HashSet<String>>,
) {
  if p.is_empty() && x.is_empty() {
    // found a maximal clique
    cliques.push(r.clone());
    return;
  }

  // choose pivot to minimize branching
  let pivot = p.union(x).next().cloned();
  let pivot_neighbors = pivot
    .as_ref()
    .and_then(|p| graph.get(p))
    .cloned()
    .unwrap_or_default();

  // iterate over vertices in P that are not neighbors of pivot
  let candidates: Vec<String> = p.difference(&pivot_neighbors).cloned().collect();

  for v in candidates {
    let v_neighbors = graph.get(&v).cloned().unwrap_or_default();

    r.insert(v.clone());

    let mut new_p: HashSet<String> = p.intersection(&v_neighbors).cloned().collect();
    let mut new_x: HashSet<String> = x.intersection(&v_neighbors).cloned().collect();

    bron_kerbosch(r, &mut new_p, &mut new_x, graph, cliques);

    r.remove(&v);
    p.remove(&v);
    x.insert(v);
  }
}

fn find_maximum_clique(graph: &HashMap<String, HashSet<String>>) -> Vec<String> {
  let mut cliques = Vec::new();
  let mut r = HashSet::new();
  let mut p: HashSet<String> = graph.keys().cloned().collect();
  let mut x = HashSet::new();

  bron_kerbosch(&mut r, &mut p, &mut x, graph, &mut cliques);

  // find the largest clique
  let max_clique = cliques
    .into_iter()
    .max_by_key(|clique| clique.len())
    .unwrap_or_default();

  let mut result: Vec<String> = max_clique.into_iter().collect();
  result.sort();
  result
}

fn solve(input: &str, part: u8) -> String {
  let graph = parse_input(input);
  match part {
    1 => {
      let triangles = find_triangles(&graph);
      count_triangles_with_t(&triangles).to_string()
    }
    2 => {
      let max_clique = find_maximum_clique(&graph);
      max_clique.join(",")
    }
    _ => panic!("Only part 1 or 2 is possible."),
  }
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, 1));
  println!("Part 2 result = {}\n", solve(&input, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day23_simple.txt", "Simple puzzle")?;
  print_result("input/day23_full.txt", "Full puzzle")?;
  Ok(())
}
