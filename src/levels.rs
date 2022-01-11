use std::collections::HashMap;
use std::slice::Iter;

pub const MAX_LEVELS : u8 = 10;

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

const LEVEL6: [&str; 15] = [
    "", "", "",
    "   AAaaAAaaAAaaAAaaAAaaAAaa   ",
    "    BBbbBBbbBBbbBBbbBBbbBB    ",
    "     CCccCCccCCccCCccCCcc     ",
    "      DDddDDddDDddDDddDD      ",
    "       EEeeEEeeEEeeEEee       ",
    "        FFffFFffFFffFF        ",
    "         GGggGGggGGgg         ",
    "          HHhhHHhhHH          ",
    "           IIiiIIii           ",
    "            JJjjJJ            ",
    "             KKkk             ",
    "              LL              "
];

const LEVEL7: [&str; 17] = [
    "", "",
    "  aabbccddeeffggFFEEDDCCBBAA  ",
    "   aabbccddeeffFFEEDDCCBBAA   ",
    "    aabbccddeeffEEDDCCBBAA    ",
    "     aabbccddeeEEDDCCBBAA     ",
    "      aabbccddeeDDCCBBAA      ",
    "       aabbccddDDCCBBAA       ",
    "        aabbccddCCBBAA        ",
    "         aabbccCCBBAA         ",
    "          aabbccBBAA          ",
    "      hh   aabbCCAA   hh      ",
    "     hhHH   aabbAA   hhHH     ",
    "    hhiiHH   aaAA   hhiiHH    ",
    "   hhiiIIHH   aa   hhiiIIHH   ",
    "  hhiijjIIHH      hhiijjIIHH  ",
    " hhiijjJJIIHH    hhiijjJJIIHH "
];

const LEVEL8: [&str; 18] = [
    "                              ",
    "                              ",
    "  bbBBbbBBbbBBbbBBbbBBbbBBbb  ",
    "  ooggGGggGGggGGggGGggGGggoo  ",
    "  ooggGGggGGggGGggGGggGGggoo  ",
    "  ooppPPppPPppPPppPPppPPppoo  ",
    "  ooppPPppPPppBBppPPppPPppoo  ",
    "  ooppPPppPPbbBBbbPPppPPppoo  ",
    "  ooppPPppBBbbOObbBBppPPppoo  ",
    "  ooppPPbbBBooOOooBBbbPPppoo  ",
    "  ooppBBbbOOooYYooOObbBBppoo  ",
    "  oobbBBOOooyyYYyyooOOBBbboo  ",
    "  oobbooOOYYyyYYyyYYOOoobboo  ",
    "  ooOOooyyYYyyYYyyYYyyooOOoo  ",
    "  ooOOYYyyYYyyYYyyYYyyYYOOoo  ",
    "  ooyyYYyyYYyyYYyyYYyyYYyyoo  ",
    "  ooyyYYyyYYyyYYyyYYyyYYyyoo  ",
    "  bbBBbbBBbbBBbbBBbbBBbbBBbb  "
];

const LEVEL9: [&str; 22] = [
    "",
    "       bBb                    ",
    "      BcCcB                   ",
    "     bCwCcsb  b               ",
    "     bCcCcsb b                ",
    "      BcCsB B                 ",
    "    BbBsSsBbB       bBb       ",
    "   bcCcbBbcCcb     BcCcB      ",
    "  bcwcCsbcwcCsb   bCwCcsb  b  ",
    "  bcCcCsbcCcCsb   bCcCcsb b   ",
    "  bcCcsSbcCcsSb    BcCsB B    ",
    "   bsSsb bsSsb   BbBsSsBbB    ",
    "    bBb   bBb   bcCcbBbcCcb   ",
    "               bcwcCsbcwcCsb  ",
    "               bcCcCsbcCcCsb  ",
    "               bcCcsSbcCcsSb  ",
    "                bsSsb bsSsb   ",
    "                 bBb   bBb    ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
];

const LEVEL10: [&str; 19] = [
    "",
    "    rRrRr                     ",
    "   RrRrRrRrR                  ",
    "   BbBoObo                    ",
    "  boboOoboOo       F    f   f ",
    "  bobBoOoboOo     f e         ",
    "  bBoOoObBbB       F  f     e ",
    "    oOoOoOo        Ff      E  ",
    "   bBrbBb        E  f fF F  f ",
    "  bBbrbBrbBb       FfFfFf  F  ",
    " bBbBrRrRbBbB     fFeFeFfFf   ",
    " oObrorRorboO    FfEeEeEfF    ",
    " oOorRrRrRoOo    FeEeWwEeFf   ",
    " oOrRrRrRrRoO   fFeFwWfEeFf   ",
    "   rRr  RrR     fFeFwWfEeFf   ",
    "  bBb    bBb    fFeEwWeEeFf   ",
    " bBbB    bBbB   fFfEeEeEfF    ",
    "                 FfFfFfFfF    ",
    "                   FfFfF      "
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
        6 => Option::Some(convert_level_data(LEVEL6.iter())),
        7 => Option::Some(convert_level_data(LEVEL7.iter())),
        8 => Option::Some(convert_level_data(LEVEL8.iter())),
        9 => Option::Some(convert_level_data(LEVEL9.iter())),
        10 => Option::Some(convert_level_data(LEVEL10.iter())),
        _ => None
    }
}
