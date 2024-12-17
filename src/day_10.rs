use std::collections::HashSet;

#[derive (PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn calculate_routs(map: &Vec<Vec<usize>>, distinct: bool)->usize {
    let heads = find_heads(&map);
    let mut count = 0;
    for th in heads {
        let mut visited: HashSet::<(usize, usize)>= HashSet::new();
        let mut path: HashSet::<Vec<Direction>>= HashSet::new();
        let mut steps: Vec<Direction> = Vec::new();
        follow_trail(&map, th.0, th.1, &mut visited, &mut path, &mut steps);
        if distinct {
            //part 2
            count += path.iter().count();
        } else {
            //part 1
            count += visited.iter().count();
        }
    }
    count
}

fn follow_trail(map: &Vec<Vec<usize>>, 
                x: usize, y: usize, 
                visited: &mut HashSet::<(usize, usize)>,
                path: &mut HashSet::<Vec<Direction>>,
                steps: &mut Vec<Direction>) 
{
    if map[y][x]==9 {
        visited.insert((x,y));
        path.insert(steps.clone());
        return;
    }
    
    let expected_next = map[y][x]+1;
    
    if x>0 && map[y][x-1]==expected_next {
        steps.push(Direction::Left);
        follow_trail(&map, x-1, y, visited, path, steps);
    } 
    if x<map[y].len()-1 && map[y][x+1]==expected_next {
        steps.push(Direction::Right);
        follow_trail(&map, x+1, y, visited, path, steps);
    } 
    if y>0 && map[y-1][x]==expected_next {
        steps.push(Direction::Up);
        follow_trail(&map, x, y-1, visited, path, steps);
    } 
    if y<map.len()-1 && map[y+1][x]==expected_next {
        steps.push(Direction::Down);
        follow_trail(&map, x, y+1, visited, path, steps);
    }
}

fn find_heads(map: &Vec<Vec<usize>>)->Vec<(usize, usize)> {
    let mut trail_heads = vec![];
    
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] == 0 {
                trail_heads.push((i, j));
            }
        }
    }
    
    trail_heads
}

fn parse_map(source: &str)->Vec<Vec<usize>> {
    source.split('\n')
        .map(|line| 
            line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap_or(10))
            .collect()
        ).collect()
}  

fn main() {
    let source = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let map = parse_map(&source);
    assert_eq!(36, calculate_routs(&map, false));
    assert_eq!(81, calculate_routs(&map, true));
    
    let source = 
"987123434330121232101001234730123456781067632
876076576521010345692340349823212347892398701
945087689432105676787659856714503210987445610
332196576587654989801456787609654502376530923
211543210298923215432321098128778901430121894
300692340147210106523543210039569876589836765
456781678236103267015693016543410231276745650
576890549345234178106782187612320140345654321
985098432100125089235493498109876056034765012
834127102345456978340362569018765487123876678
123236221976347869651251078729034398101985589
014545340889298958707867897430120987012834432
105965456770107843216950956541231276543124501
896872378761016930345441019876501345678023670
787901069654325321210332398545432330589012981
107821543213034321089206787638901421432103210
215430694102123475670115896129876548901210349
126989780210014984308924925014578037654321458
037878921001235675217833210123669123109452367
549865438901045102346542106548754321278501476
678954987432696201256430087239689870347699985
230143006501787349961021298101236787656788014
123272112981010458872787034010345691875107623
054387623472129867763698125676210010961234510
565694502561036789854567012980387121250129878
676783411051045672343218763901296030343278569
989872123432345891050109654812345145467303450
012763094321056700891760345765432256958912341
103450185789763211709851236876301967843211032
814321276656854345612345654954101878701208983
923434434565956745678036783063210989870345674
874532345410345832989123192178981876781456564
265101654323234901808765013265432185692387565
103216765432101267814554323476501094501893474
232109856321011876923601098789678923432102985
343898707896540945498712367765672310567891078
456789010987231234321203456894581455454986569
556776125670102343100157654503490166303890432
543895434894321765212348983212321876212761201
432104898765010894301054581200110955211654300
301256567656987105498765690341034567300563212
434567430547896234787654785652123498456767843
321798121032345375696543098743096567877854952
210899021121036789781232143456787656928923761
326765430110145678710123232109876543210010890";
    
    let map = parse_map(&source);
    assert_eq!(566, calculate_routs(&map, false));
    assert_eq!(1324, calculate_routs(&map, true));
}
