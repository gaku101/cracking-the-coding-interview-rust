use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph<T: Eq + std::hash::Hash + Copy> {
    adj_list: HashMap<T, Vec<T>>,
}

impl<T: Eq + std::hash::Hash + Copy> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.adj_list.entry(from).or_default().push(to);
    }

    pub fn has_route(&self, start: T, end: T) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                return true;
            }

            if let Some(neighbors) = self.adj_list.get(&current) {
                for &neightbor in neighbors {
                    if !visited.contains(&neightbor) {
                        visited.insert(neightbor);
                        queue.push_back(neightbor);
                    }
                }
            }
        }
        false
    }
}

/*
時間計算量: O(V + E)（Vはノード数、Eは辺の数。BFSの探索全体の計算量）
空間計算量: O(V)（キューや visited ハッシュセットが最悪全ノードを保持するため）
*/

/*
1. 有向グラフ (Directed Graph) とは
有向グラフは、ノード（頂点）と、それらのノードをつなぐ辺（エッジ）から構成されるデータ構造です。
ここでの「有向」とは、各辺に方向があることを意味します。
つまり、もしノード A からノード B へ向かう辺があった場合、これは A から B への一方向の関係を示し、逆方向の関係（B から A）は自動的には成立しません。

例:
以下の図は有向グラフの一例です。
    A
   ↘ ↘
    B → C

A → B：AからBへの一方向の辺があります。
A → C：AからCへの一方向の辺があります。
B → C：BからCへの一方向の辺があります。
このように、辺の向きにより情報の流れや依存関係などを表現できるのが有向グラフの特徴です。

2. 隣接リスト (Adjacency List) とは
隣接リストは、グラフを表現するための一般的な方法の一つです。
この方法では、各ノードに対して、そのノードから直接つながっている（隣接している）ノードのリストを持ちます。

例:
上記の有向グラフを隣接リストで表すと以下のようになります。

ノード A: [B, C]
(AからはBとCへ向かう辺がある)
ノード B: [C]
(BからはCへ向かう辺がある)
ノード C: []
(Cからは他のノードへ向かう辺はない)
特徴:
省メモリ: ノードが少ない場合、あるいはグラフが疎（すべてのノードが互いにつながっていない）な場合、隣接リストは非常に効率的です。
探索が容易: あるノードに直接つながるノードをすぐに取り出せるため、探索アルゴリズム（例: 幅優先探索、深さ優先探索）との相性が良いです。

有向グラフ: ノードと、方向付きの辺で構成されるグラフ。辺は一方向の関係を示し、A→Bがある場合でもB→Aは自動的には成立しません。
隣接リスト: 各ノードごとに、そのノードに隣接する（直接つながっている）ノードのリストを管理するグラフの表現方法。疎なグラフの表現に適しています。
*/

/*
1.HashMap<K, V> は、キーと値のペアを管理する連想配列です。

特徴:
キーを使って値を高速に検索できます（平均 O(1) のアクセス）。
内部ではハッシュ関数を用いて、キーから格納場所を決定しています。
キーの順序は保証されません（挿入順やソート順ではない）。
例: { "Apple": 3, "Banana": 5, "Cherry": 7 }


2.HashSet は、重複しない要素の集合 を管理するデータ構造です。
内部的にはハッシュテーブルを利用しており、以下の特徴があります。

重複しない
同じ値は 1 回しか保持されません。
高速な検索・挿入・削除
平均して O(1) の時間計算量で要素の存在確認や操作が行えます。
順序保証なし
要素の順序は保証されず、挿入順も保存されません。
例: {1, 2, 3}

VecDeque
VecDeque は、両端キュー（Double-Ended Queue） として実装されるデータ構造です。
以下の特徴があります。

両端での高速な追加・削除
push_front, pop_front（先頭）と push_back, pop_back（末尾）の操作が平均 O(1) で行えます。
リングバッファ
内部は循環バッファとして実装され、メモリの再配置が効率的に行われます。
キューやデック（両端キュー）の用途に最適
両端からデータを操作したい場合に便利です。

まず先頭に 0 を追加し、末尾に 1, 2 を追加した状態で [0, 1, 2] となり、
pop_front() で先頭の 0 を取り出し、
pop_back() で末尾の 2 を取り出しています。
*/

/*
1. Eq と Hash
Eq

Eq は完全な同値性（等しいかどうかの判定）を示すトレイトです。
グラフのノード同士が「同じノード」であるかどうかを判定するため、比較が必要になります。
例えば、ハッシュマップやハッシュセットでキーとして T を使う場合、キーの等価性が正しく判断される必要があります。
Hash

Hash は、型 T の値からハッシュ値を生成するためのトレイトです。
HashMap や HashSet は、内部でハッシュ値を使ってデータの位置を高速に決定します。
そのため、グラフの隣接リストなどで T をキーとして使用する際に、T がハッシュ可能でなければなりません。
この2つの制約により、グラフの実装で HashMap<T, Vec<T>> のようなデータ構造を使うとき、各ノード（T）を正しくキーとして管理し、同じノードかどうかの判定ができるようになります。

2. Copy
Copy
Copy は、型 T の値が単純にビットコピーできることを保証するトレイトです。
これは、変数を他の場所に代入する際や、関数に渡す際に所有権を移動するのではなく、コピーが行われるため、軽量で効率的な操作が可能になります。
グラフのノードとして使う場合、例えば整数や小さな構造体などは Copy を実装していると、値をそのまま複数の場所で利用できるので扱いやすくなります。
一方、もし T が大きなデータ構造であったり、コピーが重いものであれば、Copy を要求するのは適さないかもしれません。その場合は、Clone や参照を使う設計に変更する必要があります。

まとめ
Eq と Hash
グラフの隣接リストなどで、ノードをキーとして管理し、正しく同一性を判断するために必要です。

Copy
軽量なノードであれば、値のコピーが容易にできるようにするために用いられます。
これにより、グラフ内でノードを複数箇所で使っても、所有権の問題が発生せず、シンプルに扱えます。
*/