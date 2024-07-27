use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Num{
    NUMBER(i32),
    INF
}

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        distances.insert(from_node_id, 0);

        for _ in 0..self.nodes.len() {
            for node_id in self.nodes.keys() {
                if let Some(edges) = self.edges.get(node_id) {
                    for edge in edges {
                        let new_distance = distances
                            .get(node_id)
                            .and_then(|d: &i32| d.checked_add(edge.weight))
                            .unwrap_or(i32::MAX);
                        let current_distance = distances.get(&edge.node_b_id).unwrap_or(&i32::MAX);
                        if new_distance < *current_distance {
                            distances.insert(edge.node_b_id, new_distance);
                        }
                    }
                }
            }
        }

        distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    }

    pub fn shortest_path2(&self, from_node_id: i32, to_node_id: i32) -> i32{
        // key is node_id
        let mut hashnode_with_cost:HashMap<i32, (Num, &Node)>=HashMap::new();
        // 全てのノードに到達コストを追加したもの
        for (i,node) in &self.nodes{
            hashnode_with_cost.insert(*i, (Num::INF, node));
        }
        let mut pre_node:i32 = -1;
        let mut current_node:i32 = from_node_id;
        let mut current_cost:i32= 0;
        let mut cost_node_vec:Vec<(i32,i32)> = Vec::new();
        let mut footprint:Vec<i32> = Vec::new();
        //cost_node_vec.push((from_node_id,0));
        hashnode_with_cost.insert(current_node,(Num::NUMBER(0),hashnode_with_cost.get(&current_node).unwrap().1));

        loop {
            if current_node == to_node_id {
                break;
            }
            let next_edges = self.edges.get(&current_node).unwrap();
            footprint.push(current_node);
            for edge in next_edges{
                if edge.node_a_id == pre_node || edge.node_b_id == pre_node{
                    continue;
                }
                let current_versus_node = if current_node == edge.node_a_id{
                    edge.node_b_id
                }else{
                    edge.node_a_id
                };
                if let (Num::NUMBER(old_cost), node) = hashnode_with_cost.get(&current_versus_node).unwrap(){
                    // nodeが既に探索済み
                    let new_cost = current_cost + edge.weight;
                    if new_cost < *old_cost{
                        hashnode_with_cost.insert(current_versus_node, (Num::NUMBER(new_cost),node));
                        cost_node_vec.push((new_cost/*新しい最小のコスト*/,current_versus_node/*新しい最小のコストのノード*/));
                    }
                }else{
                    let node = hashnode_with_cost.get(&current_versus_node).unwrap().1;
                    // nodeのコストが無限だった場合 値を更新
                    let new_cost = current_cost + edge.weight;
                    hashnode_with_cost.insert(current_versus_node, (Num::NUMBER(new_cost),node));
                    cost_node_vec.push((new_cost/*新しい最小のコスト*/,current_versus_node/*新しい最小のコストのノード*/));
                }
            }
            pre_node = current_node;
            (current_cost,current_node) = *cost_node_vec.iter().min_by_key(|&&(a,_)| a).unwrap();
            cost_node_vec.retain(|&(_,n)| !footprint.contains(&n));
        }
        return current_cost;
    }
    
}
