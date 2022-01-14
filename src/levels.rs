use std::collections::HashMap;
use std::slice::Iter;
use bevy::log::Level;
use crate::prelude::*;
use crate::level_data::LevelData;

pub const MAX_LEVELS: u8 = 10;

pub struct Levels {
    level_data: HashMap<u8, LevelData>,
    level_colors: Vec<LevelColor>
}

impl Default for Levels {
    fn default() -> Self {
        // Levels
        let mut level_data = HashMap::new();
        level_data.insert(1, LevelData::new(1, LEVEL1.iter()));
        level_data.insert(2, LevelData::new(2, LEVEL2.iter()));
        level_data.insert(3, LevelData::new(2, LEVEL3.iter()));
        level_data.insert(4, LevelData::new(2, LEVEL4.iter()));
        level_data.insert(5, LevelData::new(1, LEVEL5.iter()));
        level_data.insert(6, LevelData::new(3, LEVEL6.iter()));
        level_data.insert(7, LevelData::new(3, LEVEL7.iter()));
        level_data.insert(8, LevelData::new(1, LEVEL8.iter()));
        level_data.insert(9, LevelData::new(4, LEVEL9.iter()));
        level_data.insert(10, LevelData::new(5, LEVEL10.iter()));

        // Colors
        ///////////

        let mut level_colors = Vec::new();
        // pastel colors
        level_colors.push(LevelColor::new(1, 'y', [255, 247, 165]));
        level_colors.push(LevelColor::new(1, 'p', [255, 165, 224]));
        level_colors.push(LevelColor::new(1, 'b', [165, 179, 255]));
        level_colors.push(LevelColor::new(1, 'g', [191, 255, 165]));
        level_colors.push(LevelColor::new(1, 'o', [255, 203, 165]));
        level_colors.push(LevelColor::new(1, 'x', [56, 56, 56]));
        // arkanoid colors
        level_colors.push(LevelColor::new(2, 'w', [252, 252, 252]));
        level_colors.push(LevelColor::new(2, 'o', [252, 116, 96]));
        level_colors.push(LevelColor::new(2, 'l', [60, 188, 252]));
        level_colors.push(LevelColor::new(2, 'g', [128, 208, 16]));
        level_colors.push(LevelColor::new(2, 'r', [216, 40, 0]));
        level_colors.push(LevelColor::new(2, 'b', [0, 112, 236]));
        level_colors.push(LevelColor::new(2, 'p', [252, 116, 180]));
        level_colors.push(LevelColor::new(2, 'y', [252, 152, 56]));
        level_colors.push(LevelColor::new(2, 's', [188, 188, 188]));
        level_colors.push(LevelColor::new(2, 'd', [240, 188, 60]));
        // vintage colors
        level_colors.push(LevelColor::new(3, 'a', [239, 210, 121]));
        level_colors.push(LevelColor::new(3, 'b', [149, 203, 233]));
        level_colors.push(LevelColor::new(3, 'c', [2, 71, 105]));
        level_colors.push(LevelColor::new(3, 'd', [175, 215, 117]));
        level_colors.push(LevelColor::new(3, 'e', [44, 87, 0]));
        level_colors.push(LevelColor::new(3, 'f', [222, 157, 127]));
        level_colors.push(LevelColor::new(3, 'g', [127, 157, 222]));
        level_colors.push(LevelColor::new(3, 'h', [0, 87, 44]));
        level_colors.push(LevelColor::new(3, 'i', [117, 215, 175]));
        level_colors.push(LevelColor::new(3, 'j', [105, 71, 2]));
        level_colors.push(LevelColor::new(3, 'k', [233, 203, 149]));
        level_colors.push(LevelColor::new(3, 'l', [121, 210, 239]));
        // level9 colors
        level_colors.push(LevelColor::new(4, 'b', [17, 17, 17]));
        level_colors.push(LevelColor::new(4, 'w', [238, 238, 238]));
        level_colors.push(LevelColor::new(4, 'c', [236, 113, 80]));
        level_colors.push(LevelColor::new(4, 's', [179, 58, 47]));
        // level10 colors
        level_colors.push(LevelColor::new(5, 'r', [216, 0, 0]));
        level_colors.push(LevelColor::new(5, 'b', [112, 104, 0]));
        level_colors.push(LevelColor::new(5, 'o', [248, 171, 0]));
        level_colors.push(LevelColor::new(5, 'f', [248, 56, 0]));
        level_colors.push(LevelColor::new(5, 'w', [255, 255, 255]));
        level_colors.push(LevelColor::new(5, 'e', [255, 224, 168]));

        Self {
            level_data,
            level_colors
        }
    }
}

impl Levels {
    pub fn get_level_data(&self, level_number: u8) -> &Vec<String> {
        &self.level_data.get(&level_number).unwrap().level_data
    }

    pub fn get_brick_color(&self, level_number: u8, symbol: char) -> Color {
        let symbol = make_lowercase(symbol);
        let level_data = self.level_data.get(&level_number).unwrap();
        for level_color in self.level_colors.iter() {
            if level_color.color_palette == level_data.color_palette && level_color.symbol == symbol {
                return level_color.color;
            }
        }
        panic!("No color brick in level {}, color-palette: {} and symbol '{}'",
               level_number, level_data.color_palette, symbol);
    }
}

// Discussion about to_lowercase:
// https://stackoverflow.com/questions/35716159/what-is-the-motivation-of-rusts-tolowercase
fn make_lowercase(chr: char) -> char {
    let chr_lowercase : String = chr.to_lowercase().collect();
    chr_lowercase.chars().next().unwrap()
}

/// LevelData
/// ------------------------------------------------------------------------------------------------
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
