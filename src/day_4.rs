fn calculate_occurances(map: &Vec<Vec<char>>)->usize {
    let mut count = 0;
    let find_vec = ['X','M','A','S'];
    
    for j in 0..map.len() {
        let row_len = map[j].len();
        for i in 0..row_len {
            let mut dy:i32=-1;
            while dy<=1 {
                let mut dx:i32=-1;
                while dx<=1 {
                    if (dx<1 || i<row_len-3) && (dx>-1 || i>2) &&  
                       (dy<1 || j<map.len()-3) && (dy>-1 || j>2) &&
                       ((dx==0 && dy!=0)||(dx!=0 && dy==0)||(dx!=0 && dy!=0)) && 
                        [map[j][i], map[(j as i32+dy) as usize][(i as i32+dx) as usize], 
                         map[(j as i32 +2*dy) as usize][(i as i32+2*dx) as usize], 
                         map[(j as i32+3*dy) as usize][(i as i32+3*dx) as usize]] == find_vec {
                        count += 1; 
                    } 
                    dx+=1;
                }
                dy+=1;
            }
        }
    }
    
    count
}

fn calculate_xmas(map: &Vec<Vec<char>>)->usize {
    let mut count = 0;
    let find_vec = ['M','A','S'];
    
    for j in 1..map.len()-1 {
        let row_len = map[j].len();
        for i in 1..row_len-1 {
            if ( [map[j-1][i-1], map[j][i], map[j+1][i+1]] == find_vec || [map[j+1][i+1], map[j][i], map[j-1][i-1]] == find_vec) &&
                ( [map[j-1][i+1], map[j][i], map[j+1][i-1]] == find_vec || [map[j+1][i-1], map[j][i], map[j-1][i+1]] == find_vec) {
                    count+=1;
                }
        }
    }
    
    count
}


fn convert_block_to_vec(source: &str)->Vec<Vec<char>> {
    source.split('\n').collect::<Vec<&str>>().iter().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>()
}

fn main() {
    let source = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    assert_eq!(18, calculate_occurances(&convert_block_to_vec(source)));
    assert_eq!(9, calculate_xmas(&convert_block_to_vec(source)));

    let source = "SMMMXMASMSSSMSMMSAMXSSMMSXMMAMXMXXXSMMSXXAMAASMSSMSSSMSASMMMXSXSMMSSSMSMXSXMAMAAXAXMMAMXMMMMSXSASMXMASAAMAMXMXAMAXMAXAXSAMSMXMMMAMAASASXMAMX
SAMSAMXSAAAAXMASXXMAXAAAXMSMMMSSMMMXAAMMSMASMMMAMAAAAASASAXSAMXSXMAAMAAMAMASAMXSMMMSMAMASAMXAAAAMXAMXMAMSAMAMMAMXMXMASMAXMXAAMASAMMSMASAMSSM
SAMMMMAMMMSMMMSMMMMMSSMMXXSAAMAAAMMXMMSAMMAMAAMASMXSMMMAMAXMASAXAMMSMXAMMMAMAMAAAAAMAASMSXSMMSMSMSSMMXAMSASXSAAMXSSXMXAMXSMSMSASAXSAMXMAMAAX
MXMAXMXSAXAMXMMAMAAMXMASMASMMMSSMMSASAMASMASXMMAXAMXMXMAMXMAAMMSSMAXMMSASMSMAMXSSMSSSMSXSXXXAAXAMAAAXSAXSAMASMSXSMMXSMSXASAMXMMSMMSASASAMSSM
SAXMMXAMMSMSAMXAMMASASAMMAMAXAAMMMSASASXMMMSASMXSASXXXXASASMXSASAMXSXXMAXAXSMSXMAMAAAAXXMASMSSSMMXSMMSMMMAMXMXMMAAAASAXMXSXMAMMXMAXAMXMAXXAA
SMSSSMMSXAXMASXSMSASXSSXMSMXMMMMAAMMMAMMAMXSAMAXMASXMSSMSASAAMMSXMSXMXMSMSMAMSXSAMMMMMMAMXSXXAMXAAMAMSASMSAMXASMSMMXSAMMXMMSXSMSSSMSMASMMSSM
XAAMASAAMMSSMMAAAMAMAXAXMAMXMSMSMSSXMAMMMXMMMMMMMAMAMAAXMAMXMXMXAXMAXSAMAAXSMSASMSSSXSXMAAXMMAMSMMMAMSAMAMXXMXXAMMMXMMMXAMAMXAAAAAAAMXMAAAXM
MMMSAMXSAMAAAMMMMMSMMMMSMASAAAAAXAAXMSSSMAAAXAMAMAXAXXAXMXMXXAMSAMSAMXAMSMMXAMAMXMXAASMMMSSMMMMSASXSMMAMXMAMSSMMMASAMMSSMSAMSMMMSMSMXSSMMMSS
XSAMXSAAMMSSMMXMAAAAAXXXMAMMSMSSSMMXAAAASASMSXSSSXSMSSMMMSMMSXMAMXASASMMXXAMXMXMAXMMMMAAAXAMASMSAMAAAXAXMMMMAAXMSASAXSAAAMAXAMXMAMXMAXAXMSAX
AMAMSMMMMMMAXSAMMSSSMMMMMMSAAXAAAXXSMMSMSMAMMXXMMMAMAXMASAAMAMAASMAMXMAAXMSSXMXSXSMMXSSMMSAMXSXMAMSSMMXAXAAMMSMXMAMAMMMMSXSMSAMSMSAMMSAMXMMS
MXSSMAAXSSSSMMSSMXMAXAAAAAMMMMXSAMASAXXXXXMAMMMAXSAMSASMSSSMMSMMMMAMASMMMAXMAMASAAAAXXAMAXMMMMXSXMAXAMXSXXXSAXXXMSMSMSXAMAMAXAXMXSASASXXAMXS
MAMAMXMXAMAMAMXXMASAMSSMMSMXXMAMAXASMMMSMMSAAXSAXXXMXASAXAMXMAXSXSMSMSAMMMASXMASMSMMMSAMSSMSAAAAAMASXMAXSASMMXASMXAAASMSMAMASMMAASXMASXMAMXM
MAMAMXSAMMMMSMXAXAMAMMXMAAMSASMSMMMXMAAAAASMSAMXSXMXMAMAMMMSAMXMASAAXXMXASMMXMASMMAAAMAMXAASMSMSMMASAMASMMSAMSAMAMSMMMAAMMMXXMAMXXAMMMMASMSM
SMSXSASAXXXMAMSSMSSXMXAMXSMSAXMAMAMMMSSMSMMMXXMASAMSMXMXMAAMAXMSAMMMMSMSMSAMXMASASMMSSSMSMMMMAMXAMXSXXMAXXXAMSAMXMXSAMXMSXMSMSMSSMMMSAAXXAAS
AAAAMASXMXMMAMXXAAAXSXMSXMAMXMSASAXMAMXXXXMASASASAMXMAXAMXMSSMAMASXXXAAAASAMXMAMAMAMXMXMASAAMMXSAMXMAMXMXMMSMSMMMSAMXXMAMXXAAAAAXAAASMSXMSMS
SMMSMMMMMMSXMASASMMMSAAMMSAXXXSASASMASMMSAMXSAMASMMXXSSMXXMAXAMSAMMSSMSMXSAMXMXSSXXMXMASMMXMSMASAMXSMSAMMSXXAXAAAMASASAAXMSXMMSMSMMXSAMXAAAM
XAXMAXAAAAMAXAMXXAXASMMMAXMMSAMXMXAXXSAASAMAMXMAMAMAMMAMSSMMSSXMMXSAAXXMAXAMXMMMMAMSXSASXSSXAMASAMXAXSASAMMMMMSMMSAMASXMAMMSMAXXAXSMMAMMMMSM
SMMSXXSSSXSMMSSMSXMAXASMXMAAMASASMXMXMMMSAMXXXMAXAMAXXAMXXAAAMXMAMMSSMMXSMSSSSXAMMMAAMASAAAMAMASAMXMASAMXMAAAMAMAXMMAMAXXAAAMXMMMXSASXXAAAAM
MXXMXAMAMMAXAAAAXMMMSMMASXMMSAMASAASXMSASMASXXSSSMSSXSAMMMMMXSAXMAXXMAXAAMXAAXSAMXMMXMXMMMSMMMXMAXAXMMXMASMSMSASXXMMASMSSMSMXSXXSAXMMMMXMSSM
MMSSMMMAMSAMXSMMMMAAAMMMMASAMXSMMXMSAAXAXMASMAMXAAAAXSAMXAAAXSAXMXMASAMSSMMMMMMSAXXSXXSASAMAXXXSMSSSXXXXASXMMXAXXAMSXSXAMAXAASMMMXSXMASMMAAX
SAXMAXMMMMXMMXXAAMMMXMAXXAMASAAMXXSMMMMAMMMSMAMASMMSMMAMSSMSAMXMSSSMMAXAAASAMXMAXSMSAASAMAMSMSMSAAAMMMSMXXAMASMXSAMXMMMMMAMMMMAXAXSASASMMSSM
MXXMXMMMSMSMSASXSMSXSSSSMASAMMXSAMMAMAMMMSAMXMMAXXXXMXSMMAXAAXXAXAAMSXMMMMXAMAAMXSAMMMMAMSMMMAAXMMMXMASMSSSMAXSAMXSAAXMXMASXSSMMXXSAMMSAMMAM
SMSMMASAAMXAMAMAMASAAAAMXAMAASAXMAXAMAMSAMAXASMASXXMSAMAXMMSMMMMMSMMMMMSSSMSSSSXSMMMAAXXAAXXAMSMXASAMXSAAAAMASMMMAMXXMMSAMXAMAMXXMMMMASAMSAM
AMAASAMXSXMSMAMSMAMMMMMMMMSXMMXSXSSXSASMMMSMAXXAMMASMASXMMXXSXAAMAXAAMMAMAAAAXMXXAASMSMXSXASXMMASASASAMMMSMMMXAXMASMSMASAMMSMMMXXSSMMXMMMMSM
MMSXMASAXXAMSXSXMSSSXMXSAMXMSAMXAMAAMXXAXAMMMMMAAMSXSXMMAMXMASMMSASXMSMAMMMMSMSAXSMSAMXAXXXXAXXXMASAMMSMAMAMMSMMMXAAAXAMAXSAAASMMMAMSSMSAAAX
XXMAXAMAMMSMMMXAXAAMXMAXMXAAMXSMMMSMMXSSMMSAAAXSAMAAXXXSAMMAMXXAMMMXAXXSXSAMXAMAXMAMAMMMMMSSMMXXMXMASAXMASASMAAAXMMSMMASXXSXSMXAAXAMSMASMXMS
MMSSMMMSMMXAMASMMMSMAMAMASXSMAXASAXXSAXMAMSXMXXAMXMMMSMSASASXXAXXSAMXMXMXMASMMMXAMAMAXAAAMAAAXMMSXMAMAXSASASXSSMMAXXXXAMMMMXXMSSMSSSXMXMMSMM
MAAXAXAMMXMAMAXSAXMXAXSAMMXAMXSXMASAMMSSXMXAMXMAMSMXAAAMASAMXMASXMASMXAMASXMAXSMMSASXSMMMMSSMMAAAAMXSXMAASAMXMAMMSMMMMMMAAMSMMMAAAXMASXSAAAX
MMSSSMMSASAAMXSXSAMXMAMAXSSSMAAMMAMXMXAAMSSMMASAMXAXSMSMAMAMSAAMAXSAXMASAMASMMSAASASMMMSSMXAAXMASXMMMMSMMMXMXXXMXXAAAAASMSMAAASMMMSXAAAMXSMM
XXAXAAAMASMMMXMAMXMAXAXAMXAAMXSSMXSAMMMSMAAASMSMSMMMAAAMAMXMMMSSSMMAMMAMAMAMMAMMMSSMAAAAAASXMMSXMASXMAAAAXMMMMSMMMSSSMXAAAXXSMSXSXXMXMXMAMAM
SMMMMSMMMMXAXXMAMASXSSMSAMSMMMXXMASXXAAAMMSMMASXAAASMSMSSSXAXAXAMXSASMSXSMSSMSSSXSASXMMSSMMMAXMASAAAMSSXMXXSAMAAAXMAXXSMSAMXAMXMXMASMXMMASAM
MAASXMXSMMSSSSMXMXMXAXAXSMMXXAXMMASMSSSXSXAMAMXMXXXXAMAMASMSMXSAMXMASXMAAMAAMMAMXSMMMSAXMAXXMXMASMSXMAXAMSMSASMSMSMMXMXAXASAMXXXMAMAAASMASAM
MSMSASAXAMAAAMMXSMMMSMXMMSXMAMMXSAMXAAAASMSMSXAMMMSSSMMXAXSXAXMXMAMXMAMSMMSSMMAMMMXMAMMSMSSSXAMXXAAMMASXMAAXAMAXMSAASAMXMAXXXMXMXSMMSMSXMMXM
MAXXMMMSSMMMMMMAAAXSXSXMAAMMSMAXMASMMXMXMAMAXMXSAAAAXAXMSSSMMMMSMSSXMXMAMAMAXSMMAAAMAXMAMXAMSSMSMXMXSASMSXSMAMAMMSXMMXXMMMMSMMAXAAAAMMXAMSMS
SSSMAAXMASXXMAMSSMMMAMMMAMSAAMSSSMMAXSXMMAMMMMAMMMMMMMXXMAMXXAXAAXXMMAMASAAMMAMXXXXSASXMMMMMAMMXAAXMMMSAMAAXXMXSXMMMXMXMAMAAASAMSSMMSMSMMAAS
SAAMSMSMMMMMMAXMAMAMAMAAMSMMMSAMXASXMASAMASAAXAMXXXXSSMSMAMMSSSMSMAMXMXASASXSMMXSAMSAMXXAAAMMSMSAMSXSAMAMSMSSMASAAAXASMMAMSMXMSMMAAAXAAMMMMM
MXAMXXMAAAAAXMMMAMXSMSSSMAMXXMASXXMAMAASAMXXASXSSMXSAAASMXSMAAMXXXMMASMMSAMAAXAAMAMXAMASMSSSXAAXXAXAMASXXXASAXAMXMMMXMAXSMMXSAXSSSMMSSXSSSSS
MSXMAMXMSMSXSSSSMMXXXAXMMAMXMSXMMSAMXSAMXMASXSXMXAXAMMSMMMSMXXSAMAMSASAAMAMSMMMMSAMXSMAXXAXXMMMMASMXMMMMMMMMMMMSASXMSMSMXASAMXMAXAMMXMAAXXAA
AAASMMMAMAAAXAMMASMSMMSSSXMMASAMXASXMXASAMXSXMASMSMMXXAAAAXXMMMASAMMASMMSAMAXASXSASAXMASMXMMXSXAMAAXMASXAMAAXAXSAXAXAAMASAMXMAMMSMMXAMMMMMMM
MMXMAAXAXMMSMAMSAMAXAAAMXMASAMAXMAMSMSASXSAMMXXMAMAXSSSSMSSXAXMAMAXMXMAASMSMMXSAXAMASXXXMASXASAMXMSXSASXSSSSXSXMMSSSMMMAMXMASASXSXAMMMMAAMAM
SSMSSMMXSMAXXXMMASMSMMMSAAMMASXMMSMMMAAMAMXSMXSSXSSMMXAXAXAMSSMMSMMSASXMSAAXSSMMMXSXSXSASASMXSASAXXMMASAMAMXAMXAMAAXAXSMMASXSXSXMAMSMASMMSAS
MAAXXXMXXMAXMSSMMMAAXSASASMMMMMXXXAAMMMMXMAAMAAMAMXMXMMMSMMSAAXAAAXSAMXSMMMSXAAXMAMXMASAXSXMAMAMXMMXMAMXMAMMMMAMMMSMXMAASXMASAMMAMSAXXSAMSAM
SMMMMASMMXSSMAMMAMMMXMASAXMAMAMXXMSMSXMXAMMAMMMMSMSXXXSAMXXMMSMSSSMMAMXAMXXSMMMMMASAMXMMMMSMXMSXSSMSSSSSXSAMXASMSSMMMSXXMAMAMXMAAXMAMXMAMMAM
AAAASAAAMAMAMMXSAXAAAMXMXAMMXSSMMMXAMXXSXMXXXAAXMASAMXMASMMMXMAXXMASAMXMSSMMASXXSASXSAMXAAXMAAMMMAAXAAAAXXXXXXXSXAAAASXSMSAMXSSMXSMAMXSAMMAM
XMMMMAMXMXSAMMASMSMXMMASMSASAMAASASASAMXAMSMSSSSMAMMMXMAAAAXAMMMASASAMXXAAAAAMMXMASXXMSSMXSMMSMASMMMMMMMMSMMSSXAMSMMXSAMAAAXAMASAMMAMXMAMSMS
SASXSSMMXXMAMMAMAXMASMMSAXAMXXSMMAMXMMAMXMAAXAMXMXSXMASXSXMMMSMSXMASXMXMSSMMSSXXMMMMSXMXXASAMAMXMMAXXXAAAAASAMMSMXXXMMAMMMSAAXAMMXSMSSSSMSAA
XAMAAXMXASXMMMMSMMSASAAMXMSMSXMXMSMSMSMXMMSMMASXSAMXSMMAMXAAAAAMXMAMASXXAMXXMMMXXAAXMAXXMASASASXSSXSXSXSMSMMAMAAXXXMASAMAAXMXMXSXXMAMMAMAMXM
MSMSMMSAMXAXMXXAXXMMSMMSAMXAASMAMAAMASXAMAMASAMMMAXXAAMSMMXSXMXMAMASXMSMSSMXXAXMSMXSSMMMMASAMXMAASAMASAAAMMSSMSSSSXXAXAMMAMMXMSMMMMAMMMMSMXM
AXAXMAXMXSMMSSSSSSMXSXXMASMXMXMAXMXMAMXMMASXMAXASXMSMMMXAMAMAMMSASAMAAAMMAMSSMMXASXMAMXXSASASXMSMMSMAMMMMMAAAXMAXMMMMSSMMSXXAXXXAASAMSAAXMAM
SMAMMMSXAAASXAAMAMXAMAMSMMMMSASMXSAMXSSMAASXSXSXSSXXAAASXMASAMAAAMASMXMMMAMXASXSASMSAMMMMASXMAMMAXAMXXXXAMMMSSMSMMAMAAMSAAXSXMMMSMSAAXMAMSAX
MMXMAXXMMSMMMAMMXMMMXAAAMAMSMAAAASXSAAXAMXXAXAAMMMMSAMXSAMXMAMXSMSMMXSXSMMMSMMXMMMAMAMSAMAMASAMXXMMXSSMMSSMAAMXMASMSMMSMMSMSAAXAMXMMMXXXMSAS
SAXSMSMSAMAAXSXMXSXMSMXXSASXMSMMXMXMASMMMSMMMSMXXAAXAMXXMASXXMXMMXMAASXSASASASXSSSMSSMXMMMSMSXMXAMXAAAAMAXMMSSSMAMAAXMAXMMASMMMSSSXAMMMAMMSA
MAMAAAAMMSSMSASAMMAMAMSMXXAMXAMXSMXXAXAAAXAXAMXSXMMSAMMXMAXXMSMXXAMMMSASMMASAAAAAAAAXAAXMXSAMASXMSMMSSMMMSXMAAXMAXXMMSASMMAMXXAXMASMXAAAXMXM
MSSMMMSMAAAMSAMXAMAMASAASMSSSMSAAAAMXSSMMSXMXSASAMAMMAMXMMSMAMAXMSSMXMAMXMXMMMMMMMMMSXAXSAMAMAMAAAXAAMXAAMASASXSMSMSAXMSAMXMXMSSMMXXSMMXXSAM
XMAMXMXMMXMMMAMASXXXXXMXMAAXXAMXMXXSMMXXXMASAMASMMASXSSMXMXAAMMMXAAAAMAMXMASMXXXXXXXMXMSMMSAMXSMMMSMXSXMMSAMMMAAAXAASXASMMASXMMAMXAAASMSXSAS
MSAMSXSXSAMSSSMAMMMSSMSAMMMSSSMAXMXMASAXMSMMMMMMASAMXAAXMSMSXSMSMSSMSXSAMXXMMMSMAMSXSAXAAMMAXAXXXXAMMSAAAMMMAMSMSMSMMXAMXMAAAXSAMXSMAAXMASAM
AMAMXAMAMMSAAAMMSAAAAASMXSAMAAXAASMMMAMMMAXAMASXMMXMASMMAAAXXSAAAMAXAAXMASAAAAAAAMAAXMSSSMMASMXMASXMASAMMSXSXXMAXXXAMXXSAMXSAMMAMAXSSMAMXMMM
SMSSMAMAMMMMSMMXSMSSMMMMAMAMSMMSSMASAXMMSAMMSASMXMAMMAMMSMSMAMXMXSMMMSMXSAMXMMSSXSMSMXMXAAMAXAASAMXMXSAMXAMMMMMMMMSAMXXXXMXMASMSMMMMMXXSAMXM
MXMAMMXAMXAXAAMMMAXMXSAMXSXMASXXXXAMSSXAAMSMMMMAXMAXMAXMXMAMMMASMAXAMAMXAMXAMMMMXMAAAAMSXMMSXSMMSSXXXMASMMMAAAAAAXSASXSXSAMXMMAMXMAMAXMAMSAS
SMSAMSSXXSASMSSMMAMMSSXSAMXSASMXMMMSMXMSSMAXASXSMMAXSSMMASASXMASASMXSAMXXSXXMAASAMSMSXXAAMMMAAXAMXASXSMXAASXSXSMSMSAMAAAAMXSSMAMXXMMMAMXMSAS
SAMXSAMSAXASAAMXXAXXAXMMXSXMASAAAAMAMAAAAMXXXSAAXXAMXAMSASXSAMASXXAXSAMSMSAMSSSMMXAAXMASXSAMXMMXMMMMAAAXMMMAMAXXMMMMMMMMMMAMAMXSMXSASAXAMXXS
MMMXMAMMSMAMMMSMSMSMMSAMASMMMMXSMMSAMSXXAMXMSMMMMMSSSMMMASASXMASXMXMMAMMAMAMMMXAXXMSMMMAAXXXMXXAXAMMSMMSMAMAMAMAXXAXAAXXSMXSMMMSAASMSASMSMXS
MSXMMSMAAMXMAXAAASAAXXXMASXXSMAXAASAXXMAXAMXAAXMAXMAXXAMXMXMXMASMAMMMAMMSMMMXASMMXMAAXSMSMAMASXMSMSAMXAAXAMMMMSAMXMSXMMXASAAXAAMMMSXMAMXAMAM
XAASAMMSMSSMXSSSMSSSXMXMASXAXMAXMMMSAMXXXAXSMSMMMMMAMSXSAMXSXMASASAAASXMXAXSMMMXMASAMMXAXMMAXSAAMXSAMMSSSMSXAAMMMSAXXSSMMXMXSMSSSXXMMSMSMMAS
MMSMASMAXAAAAMMAAXXMASAMXSMMMMSXMMXMMSAMSSMAXMMAMSMMMSXXAMASASXSXMSSMMASMSMXAXAXMASASAMAMMXSASMMMASXMAMXMASMMMSAAMXMXXAAXMSAMXXMMMXXXAAAXSAX
MMMMMMAMMXSMMSXMMSAMXSASAXASXAMAMAMMAMAMAAMMMSMAMAAAAMMXSSMMAMAXXXXMXSAMAMXSMMSSMAXAAMMXXXAMAXASMXSXMXSMMAMAAMMMMSAMXSSMMAMASMSASAXXXMSMMMSS
XSXAAXXXMAXMMXASAMAMASAMMSAMMSSXMASMAMMMMSXSAMMMMXMMSXSAAAAMSMMMSSXMXMMMAMAAXAAMMAMSMMSXMMXSASXMXSMAMXAMMAMMXMASMSASAMAMMSSXMASAMSSXAXAXXAAM
XASXMSXAMAXAXSMMASXMAMXMMMAXXAAASAXXAXMAMMAMASASMSSMMAMMSMMMMAAAXMSMSMASASMSMMSSMMMXAAMASAXSAMMSMMSAMXMMSMXSAMXMASAMXSAMXXXMMMMMMAMXXMASMMMS
AMAMXMMMMMSMMSXSAMXMASAMXASMMMSMMASMMSMASMAMAMASAAXAMXMXXAMMSXMMMAAXAMASMSMAASXXXAAMXMSAMSMMXMAAAMMMMSAAXAASXMASMMSMASASMMASAXAAMXSAMMAMAXSX
SMASAXAXAAAAXXMMAXXMAXASAMAAXAXXMXMAAXMXMMXMAMXMMMMMMMSMMMXAAAMSMXMSMMASXXMXSMMXSMSXMMMAMXXXAMSSSMAAAMMMXMMSASXSXAXMXMAAAMASXSSXXAMXXMASMMSM
AMAMAMSMMSSSMSXSXMMMSXMMASXSMSSMMAXMMSAMXMXSSSSMMSAASAAXMAMMSSXAASXSAMXSAMXSAMMXMAXMAASMSMMMAMMAMXSMMSSMSAMXXMAXMASXASXSSMMSXMASMSMSASAMXAMX
MMSSMAMAMAMAXMASXMAAXASXMMMMAMAASMXSAAASASMMMAAAASXSMXMSMXMXAMMMMAASMMXSMAXSAMSXSASMSMSAAAMSSMMAMAMAXAAMSAMXMXXXAMMAMXAMAAASMMAXMASAAMMSMSSX
XXAAAAMAMMXSMMAMXMAXSAMAMXSMSMMMMAAMMSMSASXAMSMMMSXXMMSAMSSMMXXXAMXMXXAMXSXSAMXXMAMMAAMMMSXAAASMMXSAMMXAXAMXMXXSMMMAMMMMSMMXAMASAMXMXMXAXMAM
SMXXMMXSMSAXMSAASMMMXXXXSMMAXAMXMMMMAAMMMMMXMAAAAMASAMSASAAXSASMXSAXAMXMAMXSAMXSMXMSMMMXXXMMSMMSMASXSASMSSMAMSMMASXXSXXAXAMSSMMAMXXXMSMMXMAS
SAAMMMXAAMXSASMXSAMXMASMXAMAMSAMXAXMSXXAXMSMSSSMXSASAMSXMMSMSASAASMMSMSAASASAMXXAXXMMMXAMSMXAMXSMAMXAXXAAXMASAAAAMSAMXMASXMAMAMSAMXSAMASXMAS
XAXSAASMMMAMAMMASAMXAXXSSSMASXXMSXSMMMSMXSAAAAXAMMMMXMXMXAXAMAMMXMAXAAMSAMMSAMXMXMXAXMMSMMMAAMAXMASAMMMMMSSXXXSMAMXMMAXMMAMXSAMXAMASMSAMAMAM
MSMMXXXAAXMAMSMASXMMSMMAXASMXXAXSAMXAXAAMSMMMXMXSASMSMSMMSSSMAMXSSSMMXMMMAXSAMXAAAXMMSAMXAXMXMSSMASAAXAAAMAXMAMMMSASMSMSAAMMSMMSAMASXMASXMAS
AMAXSXSXMMAMXSMMMMSXAAMMSMMMMSMMXMASMSMSMMMMXXSMSAMAAASMMAAXMASMAAMXSAAMASXSXMAXXSMXAMSSMMMSAXAAMAMMXMMSXSASMAXAAMAMAAAASXSXXMASAMAMASAMASXS
SSMMAAASXMASXMASMAXMMXMXMXAAXAAAXMAMMAMXXXAMMMSAMXMXMMMASMMMSASXMXMASXSMAMMSASXSAAMMMSASAAASMMSXMSSMSMMXAMAMMSMMMSAMXMMMXXMXXMAXXMMSAMASAMMX
XAAXMMMAXSAMMSAMASXMSXSAMXMXSMSAMXSMSASMSMSSMASXSMSASASXMAAAMXSAMXMXSAMMMMAMMAAAMMAAXMASMMMMMAXXMAAAAAASAMXMXMAMAMMMMMMXSMSMSMXSAMXMASXMASAX
SSMMXAMXAMASXSASAXXMAAMAMMSMAXXXSMMASASAXXMAMMMAMAMASMAASMMMSASAMMXMMAMAXMSSSMMMXXMSMMXMXSSMXAXMMSSMSMMMAMXMASMMASAMAMXAXAAAAAASXMASAMXSMMMX
XAXXSXSMXMASMSAMXXAMMMMAXAAMMMXAAAMAMAMMSASAMAMAMXMASXSXXMAMMASMMSAMMXMXMMMMMASXMXXMASASXMAAMMSAXAMAMAMSMMMMAXMSMSMSASMXSSMSMSMSASXSAMXXASXM
SAMXXAXXXMAMXMXMMMMMASMMMSXXXMXSMXMXMXMAMMSMSXSASXMASAMAMXXXSAMAAXASMSMSAXMMSAMAMSXSAMAMXMAMMXAMMAXAMAXAMAXMXSMAAXXMAXXXMXMXMAMSAXAMAAAMXASA
MAAAMAMMMMSMSAMXSAAMAMAAXAMXSAAMXXMASXMASMXMXXMASXXAMAMASMSAMASMMSAMXAASAMXXMMSSMAXMAMSMSXSSXXSMMMMASXSMSSSMASMSSSMSMSMMMAAAXXAMXMMSSMSXSASM
SMSXSAAAAAAAXXMAMXSMMMSMMAXAAMMMMMSASASXMMAMMSMSMXMASXSXSAMXSMMMMAAAMMMMMMXSAAAXMMSSMMMAXAXMAMMAXAAMAMXXAMMMASAMXAXAAAAAXXSSXSMMMAXAAXAAMAMM
MAMMMXXMMSSXSAMXSMMMSAMMSSMMMXMAAMMMSMMMMSMSAAMAMAMXMAMAMXMXXXAXXMXMASXAXMAXMMSMXXXAXSMXMXMSAMSSMMSXAMSMSSMSMMXMMMMMSMSMSAXAASAMMSMSSMMMMMMA
MAMAMMSSMMAASXXAAXAAMMSMAXXXXMMMMSAMSMMSAAMMXXSMSMSSMMMAMAMXAMMMMXMMMMSXSMSMSXMASASMMMMMMXXSAXXXAXMMMMSMAAMSASMSMSAMXAAAMAMMMMAMXXAMXMASAMXX
SASXSAXAAMMMMXMSMMMXXAAMMSMMMSXXXSMSMAAMMXSASMSXSXAAAXSXSASXMASASASAAXAMSAXAMXXXXAXXAXAAAAASMMASMMSXSASMMXMSAMAAXMSXMMMMMSXMASAMXMMMASXSASXS
SMSAMXSSMMSMAXXAXAXSSSMSXAXAAMMMAXMXMMMSMMMASASAMMSMMMSASASMMSSMMAMSXMMXSAMXMMMMMMMSSSSMXXAXAMMMMAMAMASXAAMMXMMMSMMXMXSAMXXSMSXMASASMSASXMAS
SXMXSAXAXSAMXXMASMMXAAASMMSMMXAXXMMAMMMMMXMAMMMMMAAAXXMAMMMXMAXMMSMMMMSAMXMAAAAAAAXXMAXMSXMXMMMAMAMXMAMMSXMAMXAXAAMXAASMSMXXXMAXMAXSASAMSMAM
SAMSAXASMMSSMMSAXXAMMMMMAASXSMXSAAMMXAAAMXMASAAAMSSMMMMAMXSAMASMAMAXXMAMMASXSSSSSXXAMXMXAASASASXSMSMXSSXXXXSASMSMSMMMXSXAMAMMSXMMSMMMMAMAMAS
SAMXMSMMSXAXXAMXSAMMAAMMMMSASAASXMMXSMSSSXSAMXSXXAXMAXMAXXSAXAAXAMXSASXMSMSXMAMXAASXMSSMMMSAMASMSAAMSMMMSMMXAXAXXMAXSXMMMSMAMAMXAAMAMXSMMSAS
SSMAXXAAMMMSMMSAMMSSSSSSXXMAMMMMSXSASAAAAAAAMXXMXMASMSSSSMSAMXMSXSMAAAAASXMMMMMMMMMAAXMAXAMMMAMAMMMMAAAAAAAMMMMMSSSMMAMAMAXXAXSMSSSMSAXAXMAS
SASXMMMMMAAMAAXAAXMAAXMASAMMMXAAXXMAMMMMMMMXMSAMMSAMXAAXXAXXXAXMAMAMSMMMMAMSMAAMASMMSMSMMMAMMXMMMMXSSSMMXXMXSAMAMAMMSAMXSASXMXXAMAAAMAMMMMAM
SAMXSAXSSMMSMMSAMXMMMMSMAMMAXSMMSAMAMASAXAXMASAMASAMMMMMMSMMSAMMAMSXXASXSAMAMSXSAMMAMXXXAXSAMAXAASMMAAAMSSSMSXSAMAMXXMSMMASMXMASXSMMMXMSMMMS
MAMAMMMXASAAXASXSMXMMAMXASMSMSAAMXMASXSMXMXAXSAMXSAASASAXMAMXMASASXMXXMASASXXXMMXMMAMAASMMXAXMMSXSAMSMMMAAXAXMXXSMSAXSAMMAMAXMAMMMAXSAMAAAAX
SMMSSMXSAMSSMXSASXAXMASXMAAXAMMMSASXSAMAASXSXSMMXMMMSASASMMXAMMSMSAMSAMXMAMMAMXASXSASMMAMASXMMAMXSAMAXMMMMMMMSMAAAMMMSASMAMMXMASASXMMMSXMMMS
SXAXAAAMAMAXMMMAMSAMSMSXMMMMXMMXMASXMXMMMSAMAMXMAAAXMXMMXAMSXMASMSAMXXMAMXXAMAMXMASAXXSAMMMXXMAXASXMMMMAMAAXAAMXMSMMASAMXMASMSXSXSAMXAAMXSMM
XMAMMMMSSMXSMAMAMASXAXSXSASMSSMASMMMXMMXMMMMAMASXSMSMMXMSAMXMMMSXMMMSSSMSSMAMSSSMAMAMXSXSAMXXMMSXMMSXASAXSSMSSMSAAXMAMMMAAMAAMMMXSMMMMMXAAAM
SMSMXXMAMXMAXASASMASMXXASASMAASAMAAXMAXASAMSXSASAMASAAAMSASXMAAMMXMMAXAXAXMAMXAAMMMMSASMSAMMASAMXAXXMMMXMXAMXAAMSMSSXSASXSXSXMAMXMXAAXASXSMM
AAAMXXMASXSAMXSAMXMXXAMMMMMMSXMXSMMSAMSMMAMMAMXSMMAMMSXXSXMASMXSAASMMMSMMSMAXMSMMXAXMAXASAASAMASXXMMSSMMXMAMMMMMMXXAAMAMAAXAASXSASXSMSASXAXM
MSMMAXMAXMMSAMXMXAXAMMXSAAXXMMXAXXAMAMSXSSMMXSMMXMMMXXMASASAMAAMXMXAMAAAXAXASXXMXSXSMSMAMXMMASAMXMXMAAAMASMXSASAMMMMMMAMXMMMMMAMAMAAAMAMMMMM
MAMSMMMMSAAMSSMMMSXMXSASAMXAAMMAMMXSASXAXMASAXMAMMMAXXMAMMMMMMMMMSMSMSMSMMMSXXAMMMMXAAMXMAMXMMASXAAMMSMMAAXAXASASAAXASMXXXMXSMMMSMMMSMMMAAAX
XAMAAMAASMMXAXAMXMAAAMXMASMXAMMSMAXSMSXMASXMASMSXSAMSMMASXMSAXASASAMXMXAAMSXXSSMAAMMMMXAMMSAXSXMASXSAXAMXSMMXXSAMXSSMMSMMAMAMASAXXSMMAXSXMMM
SSSSSMSASMXMMSSMASMMMSASAMMXSMAAMSMXXSAMAXAXXXAXMAXSAAMAXAAMAMMMAMAMASXSMMAMXMASMSSMSSSSSMSXXXMXMAAMMMAAMMAMSMMAMAMAAAAMXAMAXMMAXXMAMXMAASMS
AAXAXAMMMMSAMAAMMMXMAMMMAXMAMMSMSXMSMSMSXSMMSMMMMSMSMSMSSMMMSMSXXMAMSMAXSMMMASMMSMAAAAXMAMXXSMMSSMMAXSSMMMAMAASXMMSSMMSMSASASMMSMSMAMAMXAXAA
MSMMMXMAAMSAMSSMSAAMSSXXMAMXSAMXMAAXAMASAMXASASAAAASAXAXXAMAAAXMASXSMMSMMXXXXSXXASMMMXMSAMXXAAAAAAXMAXMASMSSXXMSMXXAXXAASXMAXMAMAMMAMXMSMMSM
MXAXAMSMSMSXMXMAXSMXAAXMXMXXMAMMSMMSSMAMAMMASAMMSMSMAMXMSMMSMSAMXAXAMAAAMXSAASXSMSXMXSMXMSSXSMMSXMMMXXMAMSAMXMMXSASMMMMXMAMMSMAMAMXSAMXMXAMS
MSXMMMXAMAMSMMMMMAMXMSXSASMMMMMAXAAAMMSMMMSAMXXAXMXMAMMMAXAXXMASMSSMMSMSMAAMAMASXMASAXXAMAXAXAMXMMMMMSMXSMSMSSMASMMAAXSASAMAAXASXSAXMSXXMASA
XXSAXMMMMAMAXAXXXAXXXMASASAAAAMSMMMSXAXSAMXXMXMMMAMSASASXSMMXSAMXAAXAMXXMMMXAMAMASAMASXSMASXSASXSAAAAXMMXAMMMAMXXXSMMMXASMSXMSMSAMASXMAAMSMM
AASXMXAXSSSMSMSAMXSXAMXMXMMSXSXXMSXMMMSAMXSXAXMAMAMSXSMXAAASAMMSMSMMMSXMASASXXSSXMASMMMMXXSXMAMASMSMSXMAMSMASASMSMAASMMXMMSXXAAMAMAMAXMAXAAX
MMMAXSSMXMAXSASMAAMMSMMMXMAMXMASXSXMAMAAASXSMMMASAXXAMAMSMMMASAXXXXXMAMSAMXMXSAMMSAMASMMSMMMMAMXMAMXMMMSAASXSASAAAMMSAXXAAXSXMSMXMASXSMSSSSM
XAMSMMAAXSMMMAXXSXAAAAAMXMASAMAMAMAMSXXSMSAXSASASMMMSMAAXAXSXMMSMMAXXAAMASXSXAAMAMASAMAXXAAASXSXMXMMMAMXSXSMMAMMMXXXSAMSMMXXXSAMXSXMXXAAAXXX
SXMAAMAMMSXXMAMXMMMMSMMXMMASAMASXSAMASMAMMMMAXMXMAXAAMMMSSMXAAAAAAXMMMSSMMMAXSSMXSXMASMMMSMXMASXMASAMAMAMXXAMXMASXMAXAAXAMXSXSASASASXMSMSMSA
XMSMSMASAXMXMAXAAASXMMSAXMASXMMSXMAXAMXAMAASMSSMSSMSXXXAAXASMMSSSMSAXAAAASAMAMAMASASAAAXAAMAMAMAXAMMXXSMXASMMMSASAMAXSXSMMMMASXMASAMAAAAAAAM
SAAAAXMMMSAASMSMSMSXMAMASMMSAAMXASAMSSMSSSXSXMAXAAXAMSMMSMXAMXMAAAXXMMXXMMSXMSAMASAMMSAMXSMAMASMMSSMSXAXMAMAAAMSSXMMMMAAMXAMAMASXMAMXSMSMSMX
ASMSMSXMXMXAXAAAXAMAMAMSXXAXMASXMMXMAAMXAAXMASXMSSMMSMAAXMASMSMMMMMMSXMSSMXSASAMMMAXXMXMSAMMSMSAMXAASMSMSSSSMMXAMXMMAMXMMSMSXSAMXSXMAXAMMMMM
MXMMASMSXMXSMMMMMAMXMSSXAMMXXMAXXASMMSMMAMASXMMAMXMXAMMMMMMMXMASXAXMXAMAAMAXMSAMXXMASMAMXMAMAXXXMMMMXAMXAAXXAXSSSSSMMSAXASASMMASXMAAAMAMAAAX
XAMMAMAAAMAXASXSSXMXXXMAXMXAXXMXSASXAXXXAMXSMMMSMMMSMSXMAXAMSXMMSMSASMMSSMXSXMAMXMMAMMASAXMSXXMAAXXMMMMSMMSSSMAAAXXAASMMSMAMXSSMXSMMMAASXSSS
SXSMAMSSXMAMXMAXXMMSMMSSMMXMXASAMXMMXSXXMMXMASXMAXAXXAXSSSMXAXSXMMSAMSAMXMAMMSMMMXMAXSXSMSXSAMSAMMMSAAAMXXAAAMMMMMMMMMXMMMMMXSXAAXAAXSXSAXAX
MXMXMMAMAXSXMMMMMAXAAMAAAXASXAMXSMSSSMMSSMASXMAAMMSXMMMAXAMMSSMAXAMAMMMSMMMXAAAAAASMXXASAXXMAMXAXASASMMMMMMSMSXXAAXMXMAMSAXSXMMMMSSMXXMMXMAM
SSMMSMASMMMAAAAASMSSSMSSMMASMMMXXAXSAAAAASASXSSXMAXAMXSMMXMAXAMMMMSMMXMAXAAXSSSMSASXSMAMMMXMSMSSMSSMMMSAMXXXXMASMMSAAXMMSXMSAXSMXMAXMAMAMMMM
AAAAXXAMAAXMSSSMSAMAAAXAMXAMASAMMSMMXMMSMMMSAXMAMASAMMAMXSMMSAMXAXAMMSSMSXXMXAAAXAMAXMXMXXXMXAAAXAXXAASASXSMSXXMSASMMMSMMSASAMXMSSSMSAMASASA
SSMMSMMSMMSXMMAMXAMSMMSXXMXSAMASAXXMASXXASAMXMMAMXSMMSAMAXAAMAMMXXASAAAMMMMAMXMSMMMSMMMXXMAMAMMSMSAMAMMAMAXAXMXSMMSAXAAAASMMMMAXXAMASXXXSASX
XAASAMMAMSMXSSMMSAMXXMAMXMAMMSXMXSASXSASAMXSMASXSMSMMSAMASMMSXMASXMMMSSMAAXSXMXAXAAAAXASXMMASXMXXAAXAXXAMXMSMMMXAASXMMSMMSXMAMXSMAMAMMMMMMMM
SMMMASMMMAAAMAMAMASAMMAMXMASMSMSXSXMAMMMMMMAXXAASASAAMAMXMAAXAMXSASAMAXMMSXMASXMASMSXMMSAAMAMAMAMSMMMSSMMXMMAMAMSMSASAAAMXASXSSXXXMAMMASAMAM
AXAMAMAASMSMSAMXSAMAMXSXMXXMXMASAMAMSMAAAMSMSAMXMASMMSMMAMSMSSMAMAMAMXXSAMXSAMAXXMMMMSXSMMMASAMXAAXAAMMSSXMSAMSMXASAMSSMMSMMAXAMAXMMMSMSAXMS
XSXMMSXMMAAXSXSAMXSAMMXAMSMSMMAMAMAMAMSSXMAMMMMXMAMXMAMSXXMAAMMSSMXAMXMMASASXSXMASAAXAAMAMMXMAMMMASMXSAAMXMMASXAXAMAMAMAASAMXMMAXMASASXSXMXM
XMASXMMMMSMMXXMASXMASAMAMAAAXMAMSMSXSXMAMSMXAAXXMXSMSAMAMAMMMSAMAXMAXXMSMMASMSMSMSMMMMMMMXSASMMSXMXMMMMMSMXMMSMMMXMXMASMMSMMAMXSXMXAASAMMSAA
XSSMMMAAMAMMMMMAAASXMMMAMMMMXMAMMAMAMMMAMSXSXSSMMMMASMSAMXSMAMAMAMMMMMMAXMAMXSASXSAMXAAMXMXAAMAMAXAMAAAMAAXMAXAMXSXXMXSMASMXXSMMAXAMMMAMAMAS
XSAMASXSSMMAAAMMMMMAAXSSSSMXMMSAMAMMMSSSMSASAAXMAAMXMASXMAAMASMMSSSXAASASMXXMMAMAXAMSMMXXMMMMMMSMMMSSSSSSSMMXSXMASMMSAMXMMMSAAAXMXXXAMSMMSAX
XMAMXMXMXXSSSSMXAAMXMMMXAXAMXAAMMAMAAMMMAMAMMMMSSMMAMAMAMMMSASAMXAAMSXSXXMMMMMAMMMSMMAAAAMAMXMAAAAXMAAAAAXAXAMAMAXAAMASMSAAAASMAMMSXMMMAMMAM
SSMMASASAMMMAXASMXSAAAXMSMMXMASMSXSMXXAMXMXMAXMAMXSSXXSXMXXMASMMMMMAXMMMAXAAMMSXXXMAXMMMASASAMSXXMMMMMMMMSMMSSMMSSMMSAMXSMSSMAXXXAAMSAMAMMXM
MASXXSASAXAMXMMXAASMSMMAMAMAMAXAXASXXMAMXXXXXMMAXMMXAXSXSAXMASAMSSXMMAAASMSXSAMSAMSSMSSMXXASXMMMSSMXSXXAXAASAMXAAAAXMMSAXXAAMAMSMXSASASASMSM
SAMXMMXMASXSMSXMSMXXXMXXXAMAMMXSMMMAMSSMMSSXSXSMSAAMXMMASMMMMSMMAAAXSMMXXXAAMAMAXMAMMAAMMMMMXMAAAAMAXMXMSSSMXSMMSSSMMMMMSMSSMMXSAAMMSAMASAXS
MXMAASXMXMXAASMMMSSMMAASXSXMSAAXASMXMAAAAXXASAMAXMXMAAMAMMAAMMXMMMXMMSXMXMXSSMMMXMASMSXMAAAXASMMSSMSXSAMXXMAXSXMXAMAAXMAXAAMAMAMXMMAMAMAMAMX
SSSSMMAMXMMMMMASAAMAXMMSAMXAMMXSAMXXMXSMMSMMMMMSMMMSSSMAXSSXSAAMXXAMXAXXMMXXMAAXASXMXXXSXSXMXSXAAAXMASXSMSMXMXAXMAMSSSSSMMMMAMASMXXSSMMSSXMM
XMAMASXMMAAXXSAMMSSSMSXMAMMXMXMMMMXMSAMXAMXMAXXAAAAXMXXMMMAAAMMSSSMSMMXSASASMSMSMSAMXMASAMMSMXMMSSMMAMAMAAAASMMMSMMAAAAMMASXXSAXAMXAAAAAXASX
XMASMSAMXSSSXMASAAMAASAMSMAAAXAAAAAAMASMMSASASXSSMSSXAXMASMMMASXAMSAAAASXMASAAXAMXXMAMSMAMAAAAMMXMAMASAMSMSMSAMAAAMMMMMMSASXAMXMSAMSSMMSSXMM
MMXSXSXMAMAMXSSMMSSMMMXMAMSAMXSSSSXSAMXMSSXSXSAAXAXAMMMSASXXASAMXMXSSMXSSMXMMMSMSXASXSXMXMSMSASASXSSXSXMXAMMSXMSSSMSAMXXMASMSMXAXSAAAAXMMXMS";

    assert_eq!(2646, calculate_occurances(&convert_block_to_vec(source)));
    assert_eq!(2000, calculate_xmas(&convert_block_to_vec(source)));

}
