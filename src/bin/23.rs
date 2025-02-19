use std::collections::{BTreeSet, HashMap, HashSet};

advent_of_code::solution!(23);

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }
    connections
}

pub fn part_one(input: &str) -> Option<u32> {
    let connections = parse(input);
    let mut sets = HashSet::new();
    for pc1 in connections.keys() {
        if !pc1.starts_with("t") {
            continue;
        }
        for pc2 in &connections[pc1] {
            for pc3 in connections[pc1].intersection(&connections[pc2]) {
                let set = BTreeSet::from([pc1, pc2, pc3]);
                sets.insert(set);
            }
        }
    }
    Some(sets.len() as u32)
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,  // 图的邻接表表示
    all: HashSet<&'a str>,  // 当前确定的团成员（R集合）
    mut some: HashSet<&'a str>,  // 候选扩展节点（P集合）
    mut none: HashSet<&'a str>, // 已排除节点（X集合）
    cliques: & mut Vec<HashSet<&'a str>>, // 存储结果的极大团集合
) {
    // ↓ 终止条件：当候选节点为空时
    if some.is_empty() {
        // ↓ 若排除集合也为空，说明当前团是极大团
        if none.is_empty() {
            cliques.push(all);  // 将当前团加入结果
        }
        return;  // 无论是否极大团，递归终止
    }

    // ↓ 遍历当前候选节点的克隆（避免遍历时修改原集合）
    for node in some.clone() {  // 克隆产生迭代副本
        // ↓ 获取当前节点的邻居集合
        let neighbours = &graph[node];

        // ↓ 计算新的候选集合：some ∩ neighbours（必须与当前节点相连）
        let new_some = some.intersection(neighbours).copied().collect();

        // ↓ 计算新的排除集合：none ∩ neighbours（排除不相连的已处理节点）
        let new_none = none.intersection(neighbours).copied().collect();

        // ↓ 创建新的团：包含当前节点
        let mut new_all = all.clone();  // 克隆当前团
        new_all.insert(node);           // 将当前节点加入新团

        // ↓ 递归调用（关键分支展开）
        bron_kerbosch(graph, new_all, new_some, new_none, cliques);

        // ↓ 将处理过的节点从候选移动到排除（避免重复处理）
        some.remove(node);  // 从候选集合中移除
        none.insert(node);  // 加入排除集合
    }
}
pub fn part_two(input: &str) -> Option<String> {
    let connections = parse(input);
    let mut cliques = Vec::new();
    let pcs = connections.keys().copied().collect();
    bron_kerbosch(&connections,HashSet::new(),pcs,HashSet::new(),&mut cliques);
    let mut clique: Vec<_> = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .collect();
    clique.sort();
    Some(clique.join(","))
}
pub fn part_two_other(input: &str) -> Option<String> {
    let mut largest = HashSet::new();
    let connections = parse(input);
    for (&name, neighbours) in &connections {
        let mut group = HashSet::new();
        group.insert(name);
        for neighbour in neighbours {
            let new_neighbours = connections.get(neighbour).unwrap();
            if group.is_subset(new_neighbours) {
                group.insert(neighbour);
            }
        }
        if group.len() > largest.len() {
            largest = group;
        }
    }
    let mut names: Vec<_> = largest.into_iter().collect();
    names.sort();
    Some(names.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
