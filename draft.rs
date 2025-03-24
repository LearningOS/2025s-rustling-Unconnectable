use std::collections::{HashMap, HashSet};
use std::fmt;

// 定义一个错误类型：节点不在图中
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "尝试访问图中不存在的节点")
    }
}

// 无向图结构体
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>, // 邻接表：节点 -> [(邻居节点, 边权重)]
}

// 图的 trait
pub trait Graph {
    // 创建一个新图
    fn new() -> Self;

    // 获取邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    // 获取邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    // 添加一个节点
    fn add_node(&mut self, node: &str) -> bool {
        // 待实现
        true
    }

    // 添加一条边
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 待实现
    }

    // 检查图中是否包含某个节点
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    // 获取所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    // 获取所有边的列表
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

// 为 UndirectedGraph 实现 Graph trait
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 待实现
    }
}