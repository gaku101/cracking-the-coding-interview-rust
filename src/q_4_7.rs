use std::collections::{HashMap, VecDeque};

// build_order関数は、プロジェクトと依存関係のリストから有効なビルド順序を返します。
// プロジェクト: Vec<&str> -- 各プロジェクトの名前のリスト
// dependencies: Vec<(&str, &str)> -- (依存プロジェクト, 対象プロジェクト)のペア。つまり、対象プロジェクトは依存プロジェクトに依存している
// 戻り値: 正常な順序の場合は順序を示すVec<String>、循環依存などで順序が定まらない場合はErrを返す
pub fn build_order(
    projects: Vec<&str>,
    dependencies: Vec<(&str, &str)>,
) -> Result<Vec<String>, String> {
    // 各プロジェクトが「何個のプロジェクトに依存しているか」を示すカウンター
    let mut in_degree: HashMap<&str, i32> = HashMap::new();
    // グラフ: 各プロジェクトに対してそのプロジェクトに依存しているプロジェクトのリストを保持するマップ
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    // まずすべてのプロジェクトを初期化
    for &project in &projects {
        in_degree.insert(project, 0);
        graph.insert(project, Vec::new());
    }

    // 依存関係をグラフに反映
    // (dependency, project) の関係は、「projectはdependencyに依存している」という意味なので
    // projectの入次数を1増やし、dependencyからprojectへのエッジを張る
    for &(dependency, project) in &dependencies {
        if let Some(degree) = in_degree.get_mut(project) {
            *degree += 1;
        }
        if let Some(neighbors) = graph.get_mut(dependency) {
            neighbors.push(project);
        }
    }

    // 入次数が0のプロジェクトをキューに追加（依存関係がないため、ビルド可能）
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (&project, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(project);
        }
    }

    let mut order: Vec<String> = Vec::new();

    // キューから順にプロジェクトを取り出し、出て行くプロジェクトに依存するプロジェクトの入次数を減らす
    while let Some(project) = queue.pop_front() {
        order.push(project.to_string());
        if let Some(neighbors) = graph.get(project) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    if order.len() != projects.len() {
        return Err("有効なビルド順序が存在しません（循環依存の可能性があります）".to_string());
    }

    Ok(order)
}
/*
計算量

時間計算量: O(V + E)
（V: プロジェクト数、E: 依存関係数。各プロジェクトと依存関係を一度ずつ処理するため）

空間計算量: O(V + E)
（グラフの保存にプロジェクトと依存関係数分の領域を使用するため）
*/
