use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

/// 定义了一种图的类型，是一个嵌套的 BTreeMap，表示从顶点 V 到另一个顶点 V 的带权边，
/// 其中权重为 E。这种表示方法相比于矩阵表示更适合存储稀疏图。
///
/// - 外层的 `BTreeMap<V, BTreeMap<V, E>>` 将每个顶点映射到它的邻居及对应的边权。
pub type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// performs Dihsktra's algorithm on the given graph from the given start
// the grach is a positively-weighted undirected graph
///
/// returns a map that for each reachable vertex associates the distance and the predecessor
/// since the start has no predecessor but is reachable, `map[start]` will be None
pub fn dijkstra<V, E>(graph: &Graph<V, E>, start: &V) -> BTreeMap<V, Option<(V, E)>>
where
    V: Ord + Copy, // 要求顶点类型和边的权重类型都实现 Copy，方便接下来的操作
    E: Ord + Copy + Add<Output = E>,
{
    // 存储从起点start 到各个顶点的最短路径结果
    let mut ans = BTreeMap::new();
    // 优先队列，从过 Reverse 包装实现从小到大的排序
    let mut prio = BinaryHeap::new();

    // 起点的最短路径是0
    // start is the special case that doesn't have a predecessor
    ans.insert(*start, None);

    // 下面开始处理对于 start 的每一个领居 neighbor
    for (neighbor, weight) in &graph[start] {
        // 将领居的距离插入到 ans 之中
        ans.insert(*neighbor, Some((*start, *weight)));
        // 将路径信息（包括距离和前驱）加入到优先队列 prio 之中
        prio.push(Reverse((*weight, neighbor, start))); // 注意这个三元组是按照字典序来判断大小的，
                                                        // 所以实际上会根据 weight 来进行排序，
                                                        // 剩下的都是附带信息
    }

    // 函数主循环：处理优先队列(最小)
    while let Some(Reverse((dist_new, neighbor, prev))) = prio.pop() {
        // 这里的 match 用于检查当前节点 neighbor 的最短路径是否已经计算过。
        // 如果 ans[neighbor] 之中的路径信息是我们当前弹出的路径，就继续处理，
        // 否则跳过当前节点。
        match ans[neighbor] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {}
            // otherwise it's not interesting
            _ => continue,
        }

        for (next, weight) in &graph[neighbor] {
            match ans.get(next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    ans.insert(*next, Some((*neighbor, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, next, neighbor)));
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::{dijkstra, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
        graph.entry(v2).or_insert_with(BTreeMap::new);
    }

    #[test]
    fn single_vertex() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        let mut dists = BTreeMap::new();
        dists.insert(0, None);

        assert_eq!(dijkstra(&graph, &0), dists);
    }

    #[test]
    fn single_edge() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 2);

        let mut dists_0 = BTreeMap::new();
        dists_0.insert(0, None);
        dists_0.insert(1, Some((0, 2)));

        assert_eq!(dijkstra(&graph, &0), dists_0);

        let mut dists_1 = BTreeMap::new();
        dists_1.insert(1, None);

        assert_eq!(dijkstra(&graph, &1), dists_1);
    }

    #[test]
    fn tree_1() {
        let mut graph = BTreeMap::new();
        let mut dists = BTreeMap::new();
        dists.insert(1, None);
        for i in 1..100 {
            add_edge(&mut graph, i, i * 2, i * 2);
            add_edge(&mut graph, i, i * 2 + 1, i * 2 + 1);

            match dists[&i] {
                Some((_, d)) => {
                    dists.insert(i * 2, Some((i, d + i * 2)));
                    dists.insert(i * 2 + 1, Some((i, d + i * 2 + 1)));
                }
                None => {
                    dists.insert(i * 2, Some((i, i * 2)));
                    dists.insert(i * 2 + 1, Some((i, i * 2 + 1)));
                }
            }
        }

        assert_eq!(dijkstra(&graph, &1), dists);
    }

    #[test]
    fn graph_1() {
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
}
