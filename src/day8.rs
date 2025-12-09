use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solve_part_1(input: &str, max_nodes_in_heap: usize) -> u64 {
    let mut nodes = vec![];

    for line in input.trim().lines() {
        let mut coords = line.splitn(3, ',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();

        nodes.push(Node { x, y, z });
    }

    let mut distance_heap = BinaryHeap::with_capacity(max_nodes_in_heap);

    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let distance = distance_squared(&nodes[i], &nodes[j]);

            if distance_heap.len() < max_nodes_in_heap {
                distance_heap.push(NodeDistance {
                    node_1: i,
                    node_2: j,
                    distance: distance,
                });
            } else {
                let max = distance_heap.peek().unwrap();

                if distance < max.distance {
                    distance_heap.pop();
                    distance_heap.push(NodeDistance {
                        node_1: i,
                        node_2: j,
                        distance: distance,
                    });
                }
            }
        }
    }

    let mut circuits: Vec<Vec<usize>> = vec![];

    for distance in &distance_heap {
        let mut node_1_circuit = None;
        let mut node_2_circuit = None;

        for (circuit_index, circuit) in circuits.iter().enumerate() {
            for node in circuit {
                if *node == distance.node_1 {
                    node_1_circuit = Some(circuit_index);

                    if node_2_circuit.is_some() {
                        break;
                    }
                }

                if *node == distance.node_2 {
                    node_2_circuit = Some(circuit_index);

                    if node_1_circuit.is_some() {
                        break;
                    }
                }
            }
        }

        match (node_1_circuit, node_2_circuit) {
            (None, None) => {
                circuits.push(vec![distance.node_1, distance.node_2]);
            }
            (None, Some(i)) => {
                circuits[i].push(distance.node_1);
            }
            (Some(i), None) => {
                circuits[i].push(distance.node_2);
            }
            (Some(node_1_circuit), Some(node_2_circuit)) => {
                if node_1_circuit != node_2_circuit {
                    let [dest, src] = circuits
                        .get_disjoint_mut([node_1_circuit, node_2_circuit])
                        .unwrap();

                    dest.append(src);
                    circuits.remove(node_2_circuit);
                }
            }
        }
    }

    circuits.sort_by_key(|v| std::cmp::Reverse(v.len()));

    (circuits[0].len() * circuits[1].len() * circuits[2].len()) as u64
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut nodes = vec![];

    for line in input.trim().lines() {
        let mut coords = line.splitn(3, ',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();

        nodes.push(Node { x, y, z });
    }

    let mut distance_heap = BinaryHeap::with_capacity((nodes.len() * nodes.len() + 1) / 2);

    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let distance = distance_squared(&nodes[i], &nodes[j]);

            distance_heap.push(Reverse(NodeDistance {
                node_1: i,
                node_2: j,
                distance: distance,
            }));
        }
    }

    let mut circuits: Vec<Vec<usize>> = Vec::with_capacity(nodes.len());

    for i in 0..nodes.len() {
        circuits.push(vec![i]);
    }

    let mut last_connected_nodes = (0, 0);
    while circuits.len() > 1 {
        let distance = distance_heap.pop().unwrap().0;

        let mut node_1_circuit = None;
        let mut node_2_circuit = None;

        for (circuit_index, circuit) in circuits.iter().enumerate() {
            for node in circuit {
                if *node == distance.node_1 {
                    node_1_circuit = Some(circuit_index);

                    if node_2_circuit.is_some() {
                        break;
                    }
                }

                if *node == distance.node_2 {
                    node_2_circuit = Some(circuit_index);

                    if node_1_circuit.is_some() {
                        break;
                    }
                }
            }
        }

        let node_1_circuit = node_1_circuit.unwrap();
        let node_2_circuit = node_2_circuit.unwrap();

        if node_1_circuit != node_2_circuit {
            let [dest, src] = circuits
                .get_disjoint_mut([node_1_circuit, node_2_circuit])
                .unwrap();

            dest.append(src);
            circuits.swap_remove(node_2_circuit);

            last_connected_nodes = (distance.node_1, distance.node_2);
        }
    }

    let last_connected_node_1 = &nodes[last_connected_nodes.0];
    let last_connected_node_2 = &nodes[last_connected_nodes.1];

    (last_connected_node_1.x * last_connected_node_2.x) as u64
}

#[derive(Debug)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct NodeDistance {
    node_1: usize,
    node_2: usize,
    distance: i64,
}

impl PartialEq for NodeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for NodeDistance {}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn distance_squared(one: &Node, two: &Node) -> i64 {
    (one.x - two.x).pow(2) + (one.y - two.y).pow(2) + (one.z - two.z).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn p1() {
        let result = solve_part_1(TEST_DATA, 10);

        assert_eq!(result, 40);
    }

    #[test]
    fn p2() {
        let result = solve_part_2(TEST_DATA);

        assert_eq!(result, 25272);
    }
}
