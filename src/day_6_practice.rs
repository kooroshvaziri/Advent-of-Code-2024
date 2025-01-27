#[derive (Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive (Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive (Debug, Clone)]
struct Guard {
    pos: Position,
    dir: Direction,
}

fn calculate_moves(map_src: &Vec<Vec<char>>, guard_initial: &Guard)->usize {
    let mut map = map_src.clone();
    let mut guard = guard_initial.clone();

    map[guard.pos.y][guard.pos.x]='X';

    loop {
        if (guard.dir == Direction::Up && guard.pos.y==0) ||
            (guard.dir == Direction::Down && guard.pos.y==map.len()-1) ||
            (guard.dir == Direction::Right && guard.pos.x==map[guard.pos.y].len()-1) ||
            (guard.dir == Direction::Left && guard.pos.x==0)
        {
                //have exit map
                return map.iter()
                    .map(|line| 
                            line.iter()
                            .filter(|&c| c==&'X')
                            .count()
                        ).collect::<Vec<usize>>().iter().sum();
                
        }
        
        if guard.dir == Direction::Up && guard.pos.y > 0 {
            if map[guard.pos.y-1][guard.pos.x]=='#' {
                guard.dir = Direction::Right;     
            } else {
                guard.pos.y-=1;
            }
        } else if guard.dir == Direction::Down && guard.pos.y < map.len()-1 {
            if map[guard.pos.y+1][guard.pos.x]=='#' {
                guard.dir = Direction::Left;
            } else {
                guard.pos.y+=1;
            }
        } else if guard.dir == Direction::Right && guard.pos.x < map[guard.pos.y].len()-1 {
            if map[guard.pos.y][guard.pos.x+1]=='#' {
                guard.dir = Direction::Down;
            } else {
                guard.pos.x+=1;
            }
        } else if guard.dir == Direction::Left && guard.pos.x > 0 {
            if map[guard.pos.y][guard.pos.x-1]=='#' {
                guard.dir = Direction::Up;
            } else {
                guard.pos.x-=1;
            }
        } 
        map[guard.pos.y][guard.pos.x]='X';
    }
    
}

fn parse_guard(map: &Vec<Vec<char>>)->Option<Guard> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(guard) = match map[y][x] {
                '^' => Some(Guard{pos: Position{x, y}, dir: Direction::Up}),
                'v' => Some(Guard{pos: Position{x, y}, dir: Direction::Down}),
                '>' => Some(Guard{pos: Position{x, y}, dir: Direction::Right}),
                '<' => Some(Guard{pos: Position{x, y}, dir: Direction::Left}),
                _ => None
            } {
                return Some(guard);
            }
        }
    }
    
    None
}

fn parse_map(source: &str)->Vec<Vec<char>> {
    source.split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|l| l.len()>0)
        .map(|line|
            line.chars().collect()
        ).collect::<Vec<Vec<char>>>()
}

fn main() {
    let source =
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    
    let map = parse_map(&source);
    if let Some(guard) = parse_guard(&map) {
        assert_eq!(41, calculate_moves(&map, &guard));        
    } else {
        println!("Error: cannot find guard in the map.");
    }
    
    let source =
".........#...................................................................................................#.................#..
...........................#.......................................#..............................................................
.............#.................................................................................#............................#.....
...#.....................#.......#....................................#..............#.............................#...........#.#
..........#.....#..#.....#.............#..................#............#.....#........#.#.......#............................#....
...#...................#..........#...................#.........#..#..............##.......................................#......
..................................................................................................#...............................
...............#.........#....#.#..#.........#...........#...#....................#.........#.......#....#........................
..........................................#......#.#...................................................#.............#............
....................#............................#..........................................#.....................#.........#.....
..............#..................................#................#..#...........................................#................
.........#....###.......#........................................................................................#.#.....#...#..#.
.#....................................#.....................................#.................................#..................#
........................#...................................#.#....................#......##.#.........#..........................
............#.......#..................#..........................................................................................
....#.............#.#.....#...#.....#......................#...............#............##...#....................................
...#....#.....................................##........#........#....##.....................................#....................
...........................................................................#........#..........##................................#
..............................#.#......................#..........................#..#........#...............................#...
..............#.........###.......................................#...........#...............#......#.........#........#.....#...
............#............................#....................#........................#............#.............................
.........................................#.....................................................#.....#...............#............
.#...##.......................#.......#..........................................................................#...#...........#
....#..............#............#......#........................#.......................#...#.......................#.............
.......#...............#......#......#..........................................#..................#....................#...#.....
.......................................................................#.........#................................................
.#.......#........................................................##..............................................................
.........................#...................#..#......##..................................#......#......#........................
......................................#..............................................#......#..........#.#.........#..............
.....#..............#..#........#..............................#...##.............................................................
..##............................#.......................................................#..........#..............................
...........................................................##................................................#....................
#...............#.........#.....................................................#......................###........................
#.#.......................#........................#..#.....................#....#........#........................#...#.........#
............#.............#.#............#........................#..........#....................................................
............#...................#.....................................................................................#.......#...
.............................#...#..#...........................#...#.........................................................#...
........#.......................##..........................................................#.........#.....................#.....
............##............#..........................................................#.#.....#......#.#.......#...................
...........#.....#......#.................#.............#......................................#.....#...#........................
#............#.........#.......##.....#......#...........#...........#.......#................#...................................
......#..#...#.....#................................................#............#...#..........^.................................
............................................................................................................#....#................
......................................#.............................#........................#...............#.....#...#..........
...#.........................................................................#....................................................
.....................................#.........................................................#.............#.........#..........
#..............................#...#.................................#............................................................
..........#..................#............#.......#...........................#...................#.....#.........................
............................#...##.............................................................#............#.....................
.................#..................................................................#..#..............#...........................
..................#...............................#................#............#...........#......#............#.................
.....#......................#..............#.......................#......................#.......................................
...............................#.....#.......................#.......#......................#......#............#...........#.....
......#.............##....#.....#...............#.....#............................#....................................#.........
...#.#.......#..#...##.............................................................#.#.#...........#.....#................#.......
...........................................#...#......#...........................................................................
.........#.................................#........................................................................#........#.#..
........................#..........................................#.......#..............#.......................................
....................................................#...............#..........................#..................................
............................#................#......#...............#.....................#.......................................
#.#............#......................................................................................#.....#.....................
................................................................#....#..........................................#........#......#.
.......#........#......................................................................#.................#..................#.#...
...............................#...#......................................................................#.........#.........#...
...........#..#..................##..#.........................................#..............##............................#.....
#........#...............#.....#.....#.................#............#....#.............................#.......#.................#
...............#...................................................#.......#.......................#......#.............#.........
...#............................................................................................................................#.
#.....................#......................................................#...........................................#........
...........#..........................................#.......................................#.....................#.............
....#.##.......#..............#.....#..............................#....................................#.....................#...
...............................................................................................................#...............#..
.#...........................#...#.....#...#...................#....#........#.........#................................#.........
.................#............................................................................#........................#..#.......
.............#....#.....................................................#............#.........................#.......#..........
#....#........................#....................................................................##.............................
...#.#.....................................#...............................................#.....................#....#.........#.
.........#..............................#....................................#.................#............#.....................
................................................................#.................................................................
#.......#..............................................#.............................#...........#................................
.#......#.........#.............................#...............................................................#.................
.........#.....#......#.......................................................................................................#...
#..............................#........#............................#.................................#.....#.....#..............
......#.....................................#.......#........#....................................................................
#.......#.....................................#......................................................................#............
.....................................#......................................#......#.##..................#.....#.#................
.......................#.........#......#...........................................................................##............
.....#......#.#.................................................#.............................#............#.#.......#............
..................#.......................................#.#.............................#..................................#....
......................#.......#.....................#...................#......#.........................................#........
.........#...................................................................................................#....#...............
.........................#...........#..........#...............#.....#...................................................##......
#...#......#.#.....................#..................#.................#..................#......................................
..............................#.....#................#.....................#............#...#....#............#...#...............
.........................#..................#.............................................................................#.......
.......#....................#.......................#.....................#..........#...........#..........#...................#.
....#..........#..#................#.....#..........#........#..................#.....................##...............#.........#
.....................................................................................#......................................#.....
.......................................#...................................#.......................#..........................#...
.......#...............................#............#.....#..............................................#.................#......
.#..........................#..................................................................#...#.#.......#.#..........#.......
#...............................................................#.#............................#...............#......#...........
...#..........................#.#..#.#.............................................................#......#.......................
..................#........#.....................................................#...............................................#
....................#.............#.........#........##................#.............#............#..#............................
.........................#.................................#...........................................#.....#....................
..........................................##..........#................................#........................................#.
.................................................................................#..........#....................##........##.....
.....###......#.....#...................................................................................#..#...................#..
........#.....#.....#.....#..........#.................................................#.#......................#.................
...........................................#....#..............#..#..............#................................................
....#.............#...................................#.....#........................................................#............
.........#........#.............#..................#..........#..........................###.......#....#.#.#.....................
..................#....................#...................#...#..................................................................
.......#...............#................................................................................#................#........
....#..........................#......#....................................#.............#...#......................#...#.........
.#..#....................#.....................#.............#...............................#....................#....##.........
..#..#........#......#.................##..#.......#....................#.............................................#...........
..................#......#.............#.##.................................##.....................................#...#..........
.#...##........#........................................................................................#.....#...#....#..........
............#........................#............................................#.........#.............#.......................
................#......#........#...........................................................#............#........................
..............................#.#......#................................#...........#......................................#......
...................#...................#.............#............#.............#.................................................
.........#.......#....#........#.........#.....................#..#.......#..#...#.............................#..................
.........#...#.........................................................................#.....#.......#............................
.........................#..........#...............................................#.....................................#....#..
....#..................##..##...........#.........#..................................................#...#...........#...........#
.......#.......#.....#...............#.#.....#.............#........................#..............................#..#.........#.
........................#........................................#.............................##.................................";
    
    let map = parse_map(&source);
    if let Some(guard) = parse_guard(&map) {
        assert_eq!(5067, calculate_moves(&map, &guard));        
    } else {
        println!("Error: cannot find guard in the map.");
    } 
}
