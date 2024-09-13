use graph::dijkstra::dijkstra;
use graph::dijkstra::Graph;
use std::collections::BTreeMap;

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new);
}
fn main() {
    let mut graph = BTreeMap::new();
    add_edge(&mut graph, 'a', 'c', 12);
    add_edge(&mut graph, 'a', 'd', 60);
    add_edge(&mut graph, 'b', 'a', 10);
    add_edge(&mut graph, 'c', 'b', 20);
    add_edge(&mut graph, 'c', 'd', 32);
    add_edge(&mut graph, 'e', 'a', 7);

    let mut dists_a = BTreeMap::new();
    dists_a.insert('a', None);
    dists_a.insert('c', Some(('a', 12)));
    dists_a.insert('d', Some(('c', 44)));
    dists_a.insert('b', Some(('c', 32)));
    assert_eq!(dijkstra(&graph, &'a'), dists_a);

    let mut dists_b = BTreeMap::new();
    dists_b.insert('b', None);
    dists_b.insert('a', Some(('b', 10)));
    dists_b.insert('c', Some(('a', 22)));
    dists_b.insert('d', Some(('c', 54)));
    assert_eq!(dijkstra(&graph, &'b'), dists_b);

    let mut dists_c = BTreeMap::new();
    dists_c.insert('c', None);
    dists_c.insert('b', Some(('c', 20)));
    dists_c.insert('d', Some(('c', 32)));
    dists_c.insert('a', Some(('b', 30)));
    assert_eq!(dijkstra(&graph, &'c'), dists_c);

    let mut dists_d = BTreeMap::new();
    dists_d.insert('d', None);
    assert_eq!(dijkstra(&graph, &'d'), dists_d);

    let mut dists_e = BTreeMap::new();
    dists_e.insert('e', None);
    dists_e.insert('a', Some(('e', 7)));
    dists_e.insert('c', Some(('a', 19)));
    dists_e.insert('d', Some(('c', 51)));
    dists_e.insert('b', Some(('c', 39)));
    assert_eq!(dijkstra(&graph, &'e'), dists_e);
}
