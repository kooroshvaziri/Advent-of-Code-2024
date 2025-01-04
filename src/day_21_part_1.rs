use petgraph::algo;
use petgraph::prelude::*;
use petgraph::Graph;
use std::cmp::Ordering;
use std::collections::HashSet;

const KEYPAD_PARENT_MOVE: [[u32; 6]; 6] = [
    //outcome is parent new position
    //   -  ^  A  <  v  > // <- Child Current Pos
    [0, 0, 0, 0, 0, 0], //0   Parent Current Pos
    [0, 0, 1, 0, 4, 2], //1 ^
    [0, 0, 2, 1, 5, 0], //2 A
    [0, 0, 3, 0, 0, 4], //3 <
    [0, 1, 4, 3, 0, 5], //4 v
    [0, 2, 5, 4, 0, 0], //5 >
];

const NUMPAD_PARENT_MOVE: [[u32; 6]; 12] = [
    //outcome is parent new position
    // 9 is invalid
    //   -  ^  A  <  v  > // <- Child Current Pos
    [9, 9, 0, 9, 3, 1],   //7   Parent Current Pos
    [9, 9, 1, 0, 4, 2],   //8
    [9, 9, 2, 1, 5, 9],   //9
    [9, 0, 3, 9, 6, 4],   //4
    [9, 1, 4, 3, 7, 5],   //5
    [9, 2, 5, 4, 8, 9],   //6
    [9, 3, 6, 9, 9, 7],   //1
    [9, 4, 7, 6, 10, 8],  //2
    [9, 5, 8, 7, 11, 9],  //3
    [9, 9, 9, 9, 9, 9],   //gap
    [9, 7, 10, 9, 9, 11], //0
    [9, 8, 11, 10, 9, 9], //A
];

fn minimum_path(codes: &Vec<Vec<usize>>, num_of_robots: &usize) -> usize {
    let mut g_numpad = Graph::new_undirected();
    let mut numpad: Vec<NodeIndex> = vec![];

    for _ in 0..12 {
        numpad.push(g_numpad.add_node(()));
    }
    /*
       7(0)  8(1)  9(2)
       4(3)  5(4)  6(5)
       1(6)  2(7)  3(8)
       -(9)  0(10) A(11)
    */
    g_numpad.extend_with_edges(&[
        (numpad[0], numpad[1], 1.0),
        (numpad[0], numpad[3], 1.0),
        (numpad[1], numpad[2], 6.0),
        (numpad[1], numpad[4], 1.0),
        (numpad[2], numpad[5], 1.0),
        (numpad[3], numpad[4], 6.0),
        (numpad[3], numpad[6], 1.0),
        (numpad[4], numpad[5], 1.0),
        (numpad[4], numpad[7], 1.0),
        (numpad[5], numpad[8], 1.0),
        (numpad[6], numpad[7], 1.0),
        (numpad[7], numpad[8], 1.0),
        (numpad[7], numpad[10], 1.0),
        (numpad[8], numpad[11], 1.0),
        (numpad[10], numpad[11], 1.0),
    ]);
  
    let numpad_mappings = vec![10, 6, 7, 8, 3, 4, 5, 0, 1, 2, 11];

    let mut g_keypad1 = Graph::new_undirected();
    let mut keypad1: Vec<NodeIndex> = vec![];
    let mut keypad1_idx: Vec<NodeIndex> = vec![];

    for _ in 0..6 {
        keypad1.push(g_keypad1.add_node(()));
    }
    /*
       gap(0)  ^(1)  A(2)
        <(3)   v(4)  >(5)
    */
    g_keypad1.extend_with_edges(&[
        (keypad1[1], keypad1[2], 1.0),
        (keypad1[1], keypad1[4], 1.0),
        (keypad1[2], keypad1[5], 1.0),
        (keypad1[3], keypad1[4], 1.0),
        (keypad1[4], keypad1[5], 1.0),
    ]);

    let keypad_mappings = vec![2, 1, 5, 4, 3];
    for m in keypad_mappings.clone() {
        keypad1_idx.push(keypad1[m]);
    }

    let mut cursors: Vec<NodeIndex> = vec![keypad1_idx[0]; *num_of_robots + 1];
    let numpad_a_key: NodeIndex = numpad[numpad_mappings[10]];
    let mut total_cost = 0;

    for seq in codes {
        let mut path: Vec<char> = vec![];
        let mut code: usize = 0;
        
        cursors[0] = numpad_a_key;

        for key in seq {
            if *key < 10 {
                code = code * 10 + *key;
            }

            let mut all_possible: HashSet<(Vec<NodeIndex>, Vec<char>)> = HashSet::new();

            let a = cursors[0];
            let f: NodeIndex = numpad[numpad_mappings[*key]];

            let all_paths =
                algo::all_simple_paths::<Vec<_>, _>(&g_numpad, a, f, 0, None).collect::<Vec<_>>();

            for apath in all_paths {
                let mut cursors_copy = cursors.clone();
                let mut this_path: Vec<char> = vec![];

                for num_key in apath.clone() {
                    if cursors_copy[0] == num_key {
                        continue;
                    }
                    let req_key = get_direction(&cursors_copy[0], &num_key);
                    this_path.extend(move_robot_arm(
                        &keypad1_idx[req_key],
                        &1,
                        &mut cursors_copy,
                        &g_keypad1,
                    ));
                    cursors_copy[0] = num_key;
                }
                this_path.extend(move_robot_arm(
                    &keypad1_idx[0],
                    &1,
                    &mut cursors_copy,
                    &g_keypad1,
                )); //move back to press
                cursors_copy[0] = (numpad_mappings[*key] as u32).into();

                all_possible.insert((cursors_copy, this_path.clone()));
            }

            let min_path = all_possible
                .iter()
                .map(|(k, v)| (k, v.len(), v))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                .unwrap();
            cursors = min_path.0.clone();

            path.extend(min_path.2.clone());
        }

        total_cost = total_cost + code * path.len();
    }

    total_cost
}

fn move_robot_arm(
    target: &NodeIndex,
    id: &usize,
    keypads_ptr: &mut Vec<NodeIndex>,
    g: &Graph<(), f32, Undirected>,
) -> Vec<char> {
    if *id >= keypads_ptr.len() {
        return vec![];
    }

    let keypad_mappings = vec![2, 1, 5, 4, 3, 0];

    let a = keypads_ptr[*id];
    let f = *target;
    let a_key: NodeIndex = 2_u32.into();

    if a == f {
        return vec!['A'];
    }

    let all_paths = algo::all_simple_paths::<Vec<_>, _>(g, a, f, 0, None).collect::<Vec<_>>();

    let mut res_path: HashSet<(Vec<NodeIndex>, Vec<char>)> = HashSet::new();

    for apath in all_paths {
        let mut new_ptr = keypads_ptr.clone();
        let mut path: Vec<char> = vec![];

        for key in apath {
            if new_ptr[*id] == key {
                continue;
            }

            let req_dir = get_direction(&new_ptr[*id], &key);
            if *id < new_ptr.len() - 1 {
                //execute next robot
                path.extend(move_robot_arm(
                    &(keypad_mappings[req_dir] as u32).into(),
                    &(*id + 1),
                    &mut new_ptr,
                    g,
                ));
            } else {
                path.push(get_literal(&req_dir));
            }
            new_ptr[*id] = key;
        }

        if *id == new_ptr.len() - 1 {
            //append a push!
            path.push('A');
        }

        //update previous keypad...
        if *id - 1 > 0 {
            new_ptr[*id - 1] =
                KEYPAD_PARENT_MOVE[new_ptr[*id - 1].index()][new_ptr[*id].index()].into();
        } else {
            new_ptr[*id - 1] =
                NUMPAD_PARENT_MOVE[new_ptr[*id - 1].index()][new_ptr[*id].index()].into();
        }

        if *id < new_ptr.len() - 1 && new_ptr[*id + 1] != a_key {
            //move back arm to A
            path.extend(move_robot_arm(&a_key, &(*id + 1), &mut new_ptr, g));
        }

        res_path.insert((new_ptr, path));
    }

    if res_path.len() > 0 {
        let min_path = res_path
            .iter()
            .map(|(k, v)| (k, v.len(), v))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            .unwrap();
        *keypads_ptr = min_path.0.clone();

        return min_path.2.clone();
    }

    vec![]
}

fn get_direction(source: &NodeIndex, target: &NodeIndex) -> usize {
    let idx0 = source.index();
    let idx1 = target.index();
    let first = (idx0 % 3, idx0 / 3);
    let second = (idx1 % 3, idx1 / 3);

    match (
        second.0 as i32 - first.0 as i32,
        second.1 as i32 - first.1 as i32,
    ) {
        (0, -1) => 1,
        (1, 0) => 2,
        (0, 1) => 3,
        (-1, 0) => 4,
        (0, 0) => 0,
        _ => panic!("bad move."),
    }
}

fn get_literal(direction: &usize) -> char {
    match direction {
        1 => '^',
        2 => '>',
        3 => 'v',
        4 => '<',
        _ => 'A',
    }
}

fn parse_input(source: &str) -> Vec<Vec<usize>> {
    source
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap_or(10))
                .collect()
        })
        .collect()
}

fn main() {
    let source = "029A
980A
179A
456A
379A";

    assert_eq!(126384, minimum_path(&parse_input(source), &2));

    let source = "413A
480A
682A
879A
083A";

    assert_eq!(177814, minimum_path(&parse_input(source), &2));
}
