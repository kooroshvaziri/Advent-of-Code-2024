use petgraph::algo;
use petgraph::prelude::*;
use petgraph::Graph;
use std::cmp::Ordering;
use std::collections::HashMap;

const KEYPAD_MAPPINGS: [usize; 5] = [2, 1, 5, 4, 3];
const KEYPAD_A: usize = KEYPAD_MAPPINGS[0];
const NUMPAD_MAPPINGS: [usize; 11] = [10, 6, 7, 8, 3, 4, 5, 0, 1, 2, 11];
const NUMPAD_A: usize = NUMPAD_MAPPINGS[10];

fn minimum_path(codes: &Vec<Vec<usize>>, num_of_robots: &usize) -> usize {
    let mut local_cache: HashMap<(usize, usize, Vec<usize>), (usize, Vec<usize>)> = HashMap::new();

    codes
        .iter()
        .map(|seq| {
            let mut cursors: Vec<usize> = vec![KEYPAD_MAPPINGS[0]; *num_of_robots + 1];
            cursors[0] = NUMPAD_A;

            seq.iter()
                .map(|key| {
                    let numpad_paths = create_numpad_paths();

                    let a = cursors[0];
                    let f = NUMPAD_MAPPINGS[*key];

                    let (best_path, best_cursors) = numpad_paths
                        .get(&(a, f))
                        .unwrap_or(&vec![vec![a]])
                        .iter()
                        .map(|apath| {
                            let keypad_paths = create_keypad_paths();
                            let mut cursors_copy = cursors.clone();

                            let this_path = apath
                                .iter()
                                .filter(|&f| *f != cursors[0])
                                .map(|num_key| {
                                    let req_key = get_direction(cursors_copy[0], *num_key);
                                    let this_path = move_robot_arm(
                                        &KEYPAD_MAPPINGS[req_key],
                                        1,
                                        &mut cursors_copy,
                                        &keypad_paths,
                                        &mut local_cache,
                                    );
                                    cursors_copy[0] = *num_key;
                                    this_path
                                })
                                .sum::<usize>()
                                + move_robot_arm(
                                    //move back arm to press the button
                                    &KEYPAD_MAPPINGS[0],
                                    1,
                                    &mut cursors_copy,
                                    &keypad_paths,
                                    &mut local_cache,
                                );

                            (this_path, cursors_copy)
                        })
                        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
                        .unwrap();

                    cursors = best_cursors;
                    best_path
                })
                .sum::<usize>()
                * (seq[0] * 100 + seq[1] * 10 + seq[2])
        })
        .sum()
}

fn move_robot_arm(
    target: &usize,
    id: usize,
    keypads_ptr: &mut Vec<usize>,
    keypad_paths: &HashMap<(usize, usize), Vec<Vec<usize>>>,
    local_hits: &mut HashMap<(usize, usize, Vec<usize>), (usize, Vec<usize>)>,
) -> usize {
    if id >= keypads_ptr.len() {
        return usize::MAX;
    }

    let a = keypads_ptr[id];
    let f = *target;

    if a == f {
        return 1;
    }

    if let Some((hit_path, hit_cursors)) = local_hits.get(&(a, f, keypads_ptr[id..].to_vec())) {
        for i in id..keypads_ptr.len() {
            keypads_ptr[i] = hit_cursors[i];
        }
        return *hit_path;
    }

    let (best_path, best_cursors) = keypad_paths
        .get(&(a, f))
        .unwrap_or(&vec![vec![a]])
        .iter()
        .map(|apath| {
            let mut new_ptr = keypads_ptr.clone();

            let mut path = apath
                .iter()
                .filter(|&f| *f != keypads_ptr[id])
                .map(|key| {
                    let req_dir = get_direction(new_ptr[id], *key);
                    let this_path = if id < new_ptr.len() - 1 {
                        //execute next robot
                        move_robot_arm(
                            &KEYPAD_MAPPINGS[req_dir],
                            id + 1,
                            &mut new_ptr,
                            keypad_paths,
                            local_hits,
                        )
                    } else {
                        1
                    };
                    new_ptr[id] = *key;
                    this_path
                })
                .sum::<usize>();

            if id == new_ptr.len() - 1 {
                //append a push!
                path += 1;
            }

            if id < new_ptr.len() - 1 && new_ptr[id + 1] != KEYPAD_A {
                //move back arm to A
                path += move_robot_arm(&KEYPAD_A, id + 1, &mut new_ptr, keypad_paths, local_hits);
            }

            (path, new_ptr)
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
        .unwrap();

    local_hits.insert(
        (a, f, keypads_ptr[id..].to_vec()),
        (best_path, best_cursors.clone()),
    );

    *keypads_ptr = best_cursors;
    best_path
}

fn get_direction(source: usize, target: usize) -> usize {
    let first = (source % 3, source / 3);
    let second = (target % 3, target / 3);

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

fn create_numpad_paths() -> HashMap<(usize, usize), Vec<Vec<usize>>> {
    let mut numpad_paths: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();
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

    for i in 0..=11 {
        let first = (i % 3, i / 3);

        for j in 0..=11 {
            let a = numpad[i];
            let f = numpad[j];

            let all_paths =
                algo::all_simple_paths::<Vec<_>, _>(&g_numpad, a, f, 0, None).collect::<Vec<_>>();

            let mut all_steps: Vec<Vec<usize>> = vec![];

            let second = (j % 3, j / 3);
            let dist: usize = ((second.0 as i32 - first.0 as i32).abs()
                + (second.1 as i32 - first.1 as i32).abs()
                + 1) as usize;

            for apath in all_paths {
                if apath.len() > dist {
                    continue;
                }
                let mut steps: Vec<usize> = vec![];
                for node in apath {
                    steps.push(node.index());
                }
                all_steps.push(steps);
            }

            numpad_paths.insert((i, j), all_steps);
        }
    }

    numpad_paths.clone()
}

fn create_keypad_paths() -> HashMap<(usize, usize), Vec<Vec<usize>>> {
    let mut keypad_paths: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();
    let mut g_keypad = Graph::new_undirected();
    let mut keypad: Vec<NodeIndex> = vec![];

    for _ in 0..6 {
        keypad.push(g_keypad.add_node(()));
    }
    /*
       gap(0)  ^(1)  A(2)
        <(3)   v(4)  >(5)
    */
    g_keypad.extend_with_edges(&[
        (keypad[1], keypad[2], 1.0),
        (keypad[1], keypad[4], 1.0),
        (keypad[2], keypad[5], 1.0),
        (keypad[3], keypad[4], 1.0),
        (keypad[4], keypad[5], 1.0),
    ]);

    for i in 0..=5 {
        let first = (i % 3, i / 3);

        for j in 0..=5 {
            let a = keypad[i];
            let f = keypad[j];

            let all_paths =
                algo::all_simple_paths::<Vec<_>, _>(&g_keypad, a, f, 0, None).collect::<Vec<_>>();

            let mut all_steps: Vec<Vec<usize>> = vec![];

            let second = (j % 3, j / 3);
            let dist: usize = ((second.0 as i32 - first.0 as i32).abs()
                + (second.1 as i32 - first.1 as i32).abs()
                + 1) as usize;

            for apath in all_paths {
                if apath.len() > dist {
                    continue;
                }
                let mut steps: Vec<usize> = vec![];
                for node in apath {
                    steps.push(node.index());
                }
                all_steps.push(steps);
            }

            keypad_paths.insert((i, j), all_steps);
        }
    }

    keypad_paths.clone()
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
    assert_eq!(220493992841852, minimum_path(&parse_input(source), &25));
}
