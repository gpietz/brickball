use std::collections::HashMap;
use std::slice::Iter;

const LEVEL1: [&str; 6] = [
    "yyyyyYYYYYyyyyyYYYYYyyyyyYYYYY",
    "pppppPPPPPpppppPPPPPpppppPPPPP",
    "bbbbbBBBBBbbbbbBBBBBbbbbbBBBBB",
    "gggggGGGGGgggggGGGGGgggggGGGGG",
    "oooooOOOOOoooooOOOOOoooooOOOOO",
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
];

const LEVEL2: [&str; 16] = [
    "", "",
    "          yy      yy          ",
    "            yy  yy            ",
    "            yy  yy            ",
    "          ssSSssSSss          ",
    "          ssSSssSSss          ",
    "        SSsswwsswwssSS        ",
    "        SSsswwsswwssSS        ",
    "      ssSSssSSssSSssSSss      ",
    "      ssSSssSSssSSssSSss      ",
    "      ss  ssSSssSSss  ss      ",
    "      ss  ss      ss  ss      ",
    "      ss  ss      ss  ss      ",
    "            ss  ss            ",
    "            ss  ss            ",
];

const LEVEL3: [&str; 16] = [
    "",
    "oo",
    "ooll",
    "oollgg",
    "oollggbb",
    "oollggbbrr",
    "oollggbbrroo",
    "oollggbbrrooll",
    "oollggbbrroollgg",
    "oollggbbrroollggbb",
    "oollggbbrroollggbbrr",
    "oollggbbrroollggbbrroo",
    "oollggbbrroollggbbrrooll",
    "oollggbbrroollggbbrroollgg",
    "oollggbbrroollggbbrroollggbb",
    "ssSSssSSssSSssSSssSSssSSssSSrr"
];

const LEVEL4: [&str; 14] = [
    "", "",
    "              ss              ",
    "          bbBBssggGG          ",
    "        BBbbWWwwWWGGgg        ",
    "      bbBBwwWWwwWWwwggGG      ",
    "      bbBBwwWWwwWWwwggGG      ",
    "      bbBBwwWWwwWWwwggGG      ",
    "      ss  ss  ss  ss  ss      ",
    "              ss              ",
    "              ss              ",
    "          oo  oo              ",
    "          ooOOoo              ",
    "            OO                "
];

const LEVEL5: [&str; 18] = [
    "", "",
    "  yyYYyyYYyyYY  YYyyYYyyYYyy  ",
    "  bbBBbbBBbbBB  BBbbBBbbBBbb  ",
    "  ggGGggGGggGG  GGggGGggGGgg  ",
    "  ooOOooOOooOO  OOooOOooOOoo  ",
    "", "",
    "  yyYYyyYYyyYY  YYyyYYyyYYyy  ",
    "  bbBBbbBBbbBB  BBbbBBbbBBbb  ",
    "  ggGGggGGggGG  GGggGGggGGgg  ",
    "  ooOOooOOooOO  OOooOOooOOoo  ",
    "", "",
    "  yyYYyyYYyyYY  YYyyYYyyYYyy  ",
    "  bbBBbbBBbbBB  BBbbBBbbBBbb  ",
    "  ggGGggGGggGG  GGggGGggGGgg  ",
    "  ooOOooOOooOO  OOooOOooOOoo  "
];


fn convert_level_data(level_data: Iter<'_, &str>) -> Vec<String> {
    let mut vec = Vec::new();
    for line in level_data {
        vec.push(line.to_string());
    }
    vec
}

pub fn get_level_data(level_number: u8) -> Option<Vec<String>> {
    match level_number {
        1 => Option::Some(convert_level_data(LEVEL1.iter())),
        2 => Option::Some(convert_level_data(LEVEL2.iter())),
        3 => Option::Some(convert_level_data(LEVEL3.iter())),
        4 => Option::Some(convert_level_data(LEVEL4.iter())),
        5 => Option::Some(convert_level_data(LEVEL5.iter())),
        _ => None
    }
}
