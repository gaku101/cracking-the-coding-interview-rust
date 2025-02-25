mod q_4_1;

use q_4_1::Graph;

fn main() {
    let mut graph = Graph::new();
    // グラフの辺を定義 (例: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 5, 4 -> 6, 5 -> 6)
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 6);
    graph.add_edge(5, 6);

    println!("Route between 1 and 6: {}", graph.has_route(1, 6)); // true
    println!("Route between 2 and 5: {}", graph.has_route(2, 5));
}
