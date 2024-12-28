use core::cmp::Ordering;
use std::collections::HashMap;

fn part_1(source: &str) -> usize {
    let (mut wires, codes) = parse_input(&source);

    for (key, _) in codes.clone() {
        calculate_wire(&key, &mut wires, &codes);
    }

    let mut wires_vec = wires
        .iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(&str, u8)>>();
    wires_vec.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(&k2).unwrap_or(Ordering::Equal));
    let bits = wires_vec.iter().map(|(_k, v)| *v).collect::<Vec<u8>>();
    let mut result = 0;
    for i in 0..bits.len() {
        result += (bits[i] as usize) * 2_usize.pow(i as u32);
    }

    result
}

fn part_2(source: &str) -> String {
    /*
       Formula:                               z_id (1)
                                                |
                          ppp (2)              XOR                qqq (3)
                           |                                       |
                           |                                 y_id XOR y_id (4)
            rrr (5)        OR             sss (6)
             |                             |
     y_id-1 AND x_id-1 (7)       ppp_id-1 AND qqq_id-1 (8)


        Attention: below code is not exhaustive. Handle Z00 and Z45 edge cases if required by other puzzles' inputs:
            if Z00 is not equal to X00 XOR Y00, the associated wire needs to be swapped with Z00.
            if Z45 is not equal to (rrr as X44 AND Y44) OR (sss of prev terms) they should be handled as well.
    */

    let (_wires, codes) = parse_input(&source);

    let mut swapped_wires: Vec<String> = vec![];

    let z_codes = codes
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>();
    let z_max = z_codes
        .iter()
        .map(|v| {
            v.strip_prefix("z")
                .unwrap_or("")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
        .unwrap_or(0);

    for id in 1..z_max {
        let z = "z".to_string() + format!("{:0>2}", id).as_str();
        let x = "x".to_string() + format!("{:0>2}", id).as_str();
        let y = "y".to_string() + format!("{:0>2}", id).as_str();

        let (rand_ppp, op_23, rand_qqq) = get_wire_assoc(&z, &codes);
        let (temp_c41, _op_t, temp_c42) = get_wire_assoc(&rand_qqq, &codes);
        //get ppp and qqq in correct order...
        let (_ppp, qqq) = if temp_c41 == x || temp_c42 == x {
            (rand_ppp, rand_qqq)
        } else {
            (rand_qqq, rand_ppp)
        };

        if let Some(exp_qqq) = find_wire(&x, &y, &codes) {
            if exp_qqq != qqq {
                if op_23 == "XOR" {
                    swapped_wires.push(exp_qqq.to_string());
                    swapped_wires.push(qqq.to_string());
                } else {
                    swapped_wires.push(z);
                    //find expected of the term (2) and (3)
                    for (key, (left, op, right)) in codes.clone() {
                        if op == "XOR" && (left == exp_qqq || right == exp_qqq) {
                            swapped_wires.push(key.to_string());
                        }
                    }
                }
            } else if op_23 != "XOR" {
                swapped_wires.push(z.clone());
                //find expected of the term (2) and (3)
                for (key, (left, op, right)) in codes.clone() {
                    if op == "XOR" && (left == exp_qqq || right == exp_qqq) {
                        swapped_wires.push(key.to_string());
                    }
                }
            }
        } else {
            panic!("invalid input!");
        }
    }

    swapped_wires.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal));

    swapped_wires.join(",")
}

fn get_wire_assoc<'a>(
    id: &str,
    codes: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) -> (&'a str, &'a str, &'a str) {
    if id.starts_with("x") || id.starts_with("y") {
        let z = if id.starts_with("x") {
            let wire_number = id.strip_prefix("x").unwrap_or("");
            "z".to_string() + wire_number
        } else {
            let wire_number = id.strip_prefix("y").unwrap_or("");
            "z".to_string() + wire_number
        };

        return get_wire_assoc(&z, codes);
    }

    if let Some((p, op, q)) = codes.get(id) {
        return (p, op, q);
    } else {
        panic!("Invalid input!");
    }
}

fn find_wire<'a>(
    first: &str,
    second: &str,
    codes: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
) -> Option<&'a str> {
    codes.iter().find_map(|(k, &(l, o, r))| {
        if o == "XOR" && ((l == first && r == second) || (l == second && r == first)) {
            Some(*k)
        } else {
            None
        }
    })
}

fn calculate_wire<'a>(
    key: &'a str,
    wires: &mut HashMap<&'a str, u8>,
    codes: &HashMap<&'a str, (&'a str, &str, &'a str)>,
) -> u8 {
    if let Some(value) = wires.get(&key) {
        *value
    } else {
        //calculate
        if let Some((left, operand, right)) = codes.get(&key).clone() {
            let left_value = calculate_wire(&left, wires, codes);
            let right_value = calculate_wire(&right, wires, codes);
            let res = match *operand {
                "OR" => left_value | right_value,
                "XOR" => left_value ^ right_value,
                "AND" => left_value & right_value,
                _ => panic!("operand not found!"),
            };
            wires.insert(key, res);
            res
        } else {
            10 //error value
        }
    }
}

fn parse_input<'a>(source: &str) -> (HashMap<&str, u8>, HashMap<&str, (&str, &str, &str)>) {
    let separate: Vec<&str> = source
        .split('\n')
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>();

    let wires = separate
        .iter()
        .filter(|v| v.len() == 6)
        .map(|l| {
            l.split(&[':', ' '][..])
                .filter(|l| l.len() > 0)
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|v| (v[0], v[1].parse::<u8>().unwrap_or(0)))
        .collect::<HashMap<&str, u8>>();

    let codes = separate
        .iter()
        .filter(|v| v.len() != 6)
        .map(|l| {
            l.split(&[' ', '-', '>'][..])
                .filter(|l| l.len() > 0)
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|v| (v[3], (v[0], v[1], v[2])))
        .collect::<HashMap<&str, (&str, &str, &str)>>();

    (wires, codes)
}

fn main() {
    let source = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    assert_eq!(2024, part_1(&source));

    let source = "x00: 1
x01: 1
x02: 0
x03: 0
x04: 0
x05: 1
x06: 0
x07: 1
x08: 1
x09: 0
x10: 1
x11: 0
x12: 0
x13: 1
x14: 0
x15: 1
x16: 0
x17: 1
x18: 0
x19: 0
x20: 1
x21: 0
x22: 1
x23: 1
x24: 0
x25: 1
x26: 0
x27: 1
x28: 1
x29: 0
x30: 1
x31: 0
x32: 0
x33: 0
x34: 1
x35: 1
x36: 1
x37: 0
x38: 0
x39: 0
x40: 0
x41: 1
x42: 1
x43: 1
x44: 1
y00: 1
y01: 0
y02: 1
y03: 1
y04: 0
y05: 0
y06: 1
y07: 1
y08: 0
y09: 1
y10: 1
y11: 1
y12: 1
y13: 1
y14: 0
y15: 1
y16: 1
y17: 0
y18: 0
y19: 1
y20: 0
y21: 0
y22: 0
y23: 0
y24: 1
y25: 0
y26: 0
y27: 0
y28: 0
y29: 0
y30: 0
y31: 1
y32: 1
y33: 0
y34: 1
y35: 1
y36: 0
y37: 1
y38: 0
y39: 1
y40: 1
y41: 0
y42: 1
y43: 1
y44: 1

y34 XOR x34 -> rgh
fcr XOR pqc -> z14
y23 AND x23 -> cms
rkw AND shk -> pbq
x43 XOR y43 -> jfh
y13 AND x13 -> skt
hsd OR sng -> z45
vpv AND ddq -> bjr
shk XOR rkw -> z04
hnw AND gvs -> vdr
x08 AND y08 -> htn
bcp AND rgc -> pbh
y20 AND x20 -> ccp
x10 XOR y10 -> ggd
qfb OR snk -> kmn
y24 XOR x24 -> wsv
vdr OR dkd -> pmm
pmm XOR vgm -> z26
x32 AND y32 -> htq
bgs OR dvp -> vdd
vdd AND dgk -> wdr
y44 AND x44 -> sng
jtd AND qwj -> bsf
dgm OR cth -> msm
cft AND gsf -> nhd
dsw OR pmr -> pgq
msm AND fdm -> cwb
y16 AND x16 -> ffm
y33 XOR x33 -> mrp
y17 XOR x17 -> vcs
hcg OR cwb -> djr
phn AND wph -> cgf
ggd XOR dfm -> z10
pgq AND dwq -> wpp
wvr AND gpq -> hsd
x31 XOR y31 -> kqk
qqt AND vdb -> mrt
x04 AND y04 -> dpv
y17 AND x17 -> vfn
y11 AND x11 -> wmn
x26 AND y26 -> dmw
y30 XOR x30 -> fdm
y19 XOR x19 -> tqj
cft XOR gsf -> z11
djr XOR kqk -> bgs
rjm AND ktp -> pwn
chd AND mrp -> cwg
rgh XOR nng -> z34
x27 AND y27 -> mgt
jfh AND kmm -> scm
psj XOR qbn -> z29
x14 AND y14 -> bsd
x12 XOR y12 -> bqw
vgj AND pms -> cjw
cwg OR vjs -> nng
wpv XOR cwh -> z36
x43 AND y43 -> htd
x02 XOR y02 -> wsw
vdb XOR qqt -> z28
bwr OR vqn -> bqd
y38 XOR x38 -> dtr
qgc XOR kfh -> z06
x27 XOR y27 -> cdb
x31 AND y31 -> dvp
y00 XOR x00 -> z00
wmn OR nhd -> mnh
rrf XOR qnw -> z35
vjb OR mrt -> psj
x21 XOR y21 -> cbv
vcs XOR bfg -> z17
csb XOR pfg -> z16
wps AND kmn -> pvw
bcp XOR rgc -> z08
vgm AND pmm -> sbq
y14 XOR x14 -> fcr
pbh OR htn -> ddq
rcd XOR rqt -> z37
y18 XOR x18 -> gbv
y02 AND x02 -> snk
x06 XOR y06 -> qgc
y06 AND x06 -> mwg
x34 AND y34 -> fvp
x35 AND y35 -> tnt
chd XOR mrp -> z33
nnt AND cbv -> mcs
tmr XOR pnf -> z05
x29 AND y29 -> dgm
x42 AND y42 -> bnc
msm XOR fdm -> z30
ppf OR ffm -> bfg
nnh OR bjr -> dfm
cdb AND rtm -> cdw
ccp OR vtg -> nnt
kpv XOR rvc -> swt
y15 AND x15 -> kgh
kqk AND djr -> z31
tqj XOR frj -> z19
x01 AND y01 -> hhp
pfg AND csb -> ppf
y20 XOR x20 -> ddh
y44 XOR x44 -> wvr
ngw OR srm -> frj
x03 XOR y03 -> wps
pgt XOR hbv -> z23
nng AND rgh -> npf
y28 AND x28 -> vjb
bmc AND wsw -> qfb
y40 XOR x40 -> bps
nnt XOR cbv -> z21
y26 XOR x26 -> vgm
x37 AND y37 -> khs
djt OR sbf -> kfh
npf OR fvp -> rrf
qsm XOR bqd -> z42
cff OR rbw -> qbf
x05 AND y05 -> sbf
cdb XOR rtm -> z27
x22 AND y22 -> fmg
bsd OR cpf -> phn
x42 XOR y42 -> qsm
y13 XOR x13 -> dwq
pqc AND fcr -> cpf
thc XOR brj -> z01
wvr XOR gpq -> z44
qgc AND kfh -> cfw
dtr AND brm -> rft
cfw OR mwg -> rvc
rft OR ncq -> qwj
x25 AND y25 -> dkd
rhn OR nmm -> gnd
tqj AND frj -> rhn
jtd XOR qwj -> z39
bqd AND qsm -> skc
x30 AND y30 -> hcg
x12 AND y12 -> pmr
x32 XOR y32 -> dgk
y41 AND x41 -> vqn
pnf AND tmr -> djt
dsf OR tnt -> wpv
cgf OR kgh -> pfg
cdw OR mgt -> vdb
mtg XOR gbv -> z18
pbq OR dpv -> tmr
x25 XOR y25 -> hnw
wdr OR htq -> chd
vgj XOR pms -> z22
x08 XOR y08 -> bcp
bsf OR ccs -> gqg
x05 XOR y05 -> pnf
x39 XOR y39 -> jtd
x21 AND y21 -> jsj
x07 AND y07 -> z07
x16 XOR y16 -> csb
mtg AND gbv -> ngw
gvs XOR hnw -> z25
gnd AND ddh -> vtg
x38 AND y38 -> ncq
btc OR pvw -> shk
bqw AND mnh -> dsw
dns AND qbf -> bwr
y18 AND x18 -> srm
cwh AND wpv -> kmg
skc OR bnc -> kmm
x09 AND y09 -> nnh
wgq OR kmg -> rcd
dtr XOR brm -> z38
x00 AND y00 -> brj
x19 AND y19 -> nmm
kmm XOR jfh -> z43
bfg AND vcs -> hnh
dns XOR qbf -> z41
y22 XOR x22 -> pms
x23 XOR y23 -> hbv
dfm AND ggd -> skw
x15 XOR y15 -> wph
sgv OR khs -> brm
mcs OR jsj -> vgj
brj AND thc -> ngv
htd OR scm -> gpq
wsw XOR bmc -> z02
x37 XOR y37 -> rqt
swt OR ghb -> rgc
skt OR wpp -> z13
gqg XOR bps -> z40
y11 XOR x11 -> gsf
x39 AND y39 -> ccs
wph XOR phn -> z15
x04 XOR y04 -> rkw
y36 XOR x36 -> cwh
rqt AND rcd -> sgv
y03 AND x03 -> btc
fmg OR cjw -> pgt
x28 XOR y28 -> qqt
y35 XOR x35 -> qnw
x41 XOR y41 -> dns
ngv OR hhp -> bmc
wps XOR kmn -> z03
cms OR qdw -> ktp
ktp XOR rjm -> z24
rvc AND kpv -> ghb
vdd XOR dgk -> z32
vpv XOR ddq -> z09
x10 AND y10 -> pgp
psj AND qbn -> cth
pgp OR skw -> cft
x24 AND y24 -> rjm
y40 AND x40 -> rbw
y07 XOR x07 -> kpv
y36 AND x36 -> wgq
y01 XOR x01 -> thc
pgt AND hbv -> qdw
y09 XOR x09 -> vpv
hnh OR vfn -> mtg
dwq XOR pgq -> pqc
wsv OR pwn -> gvs
x33 AND y33 -> vjs
gnd XOR ddh -> z20
y29 XOR x29 -> qbn
dmw OR sbq -> rtm
gqg AND bps -> cff
mnh XOR bqw -> z12
rrf AND qnw -> dsf";

    assert_eq!(65740327379952, part_1(&source));
    assert_eq!("bgs,pqc,rjm,swt,wsv,z07,z13,z31", part_2(&source));
}
