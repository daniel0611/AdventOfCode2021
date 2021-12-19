use aoc_utils::PuzzleInput;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;

const DAY: u8 = 19;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    let scanners = compute_absolute_beacon_positions(&input);
    println!("A: {}", solve_a(&scanners));
    println!("B: {}", solve_b(&scanners));
}

struct Orientation {
    direction: (i32, i32, i32),     // 1 for as is, -1 for mirrored
    mapping: (usize, usize, usize), // 0 is x, 1 is y, 2 is z
}

fn get_all_orientations() -> Vec<Orientation> {
    let dir_vec = vec![1, -1];
    let directions = dir_vec
        .iter()
        .flat_map(|dir_x| {
            dir_vec
                .iter()
                .flat_map(|dir_y| dir_vec.iter().map(|dir_z| (*dir_x, *dir_y, *dir_z)))
        })
        .collect::<Vec<_>>();

    vec![0usize, 1usize, 2usize]
        .iter()
        .permutations(3)
        .flat_map(|mapping| {
            directions
                .iter()
                .map(|d| Orientation {
                    direction: *d,
                    mapping: (*mapping[0], *mapping[1], *mapping[2]),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn apply_orientation(&self, orientation: &Orientation) -> Position {
        let v = vec![self.x, self.y, self.z];
        let m = orientation.mapping;
        let d = orientation.direction;

        Position {
            x: v[m.0] * d.0,
            y: v[m.1] * d.1,
            z: v[m.2] * d.2,
        }
    }

    fn relative_position(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn manhatten_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().trim().parse::<i32>().unwrap();
        let y = iter.next().unwrap().trim().parse::<i32>().unwrap();
        let z = iter.next().unwrap().trim().parse::<i32>().unwrap();
        Position { x, y, z }
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct Scanner {
    id: usize,
    beacons: Vec<Position>,
    position: Option<Position>,
}

impl Scanner {
    fn parse_all(input: &PuzzleInput) -> Vec<Scanner> {
        input
            .raw_input
            .split("\n\n")
            .map(|line| Scanner::parse(line))
            .collect()
    }

    fn parse(s: &str) -> Scanner {
        let lines = s.split('\n').collect::<Vec<&str>>();

        let id = lines[0]
            .split(' ')
            .nth(2)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let beacons = lines
            .iter()
            .skip(1)
            .map(|line| (*line).into())
            .collect::<Vec<_>>();

        Scanner { id, beacons, position: None }
    }
}

fn compute_absolute_beacon_positions(
    input: &PuzzleInput,
) -> HashMap<Scanner, Vec<Position>> {
    let possible_orientations = get_all_orientations();
    let all_scanners = Scanner::parse_all(input);
    let mut known_scanners = HashMap::new();

    let mut first_scanner = all_scanners[0].clone();
    first_scanner.position = Some(Position { x: 0, y: 0, z: 0 });
    known_scanners.insert(first_scanner, all_scanners[0].beacons.clone());

    while known_scanners.len() < all_scanners.len() {
        for (_, base_positions) in known_scanners.clone().iter() {
            for scanner in all_scanners.iter() {
                if known_scanners.contains_key(scanner) {
                    continue;
                }

                for orientation in possible_orientations.iter() {
                    let positions = scanner
                        .beacons
                        .iter()
                        .map(|pos| pos.apply_orientation(orientation))
                        .collect::<Vec<_>>();

                    let mut relative_positions = HashMap::new();
                    for a in base_positions.iter() {
                        for b in positions.iter() {
                            let rel = a.relative_position(b);
                            *relative_positions.entry(rel).or_insert(0) += 1;
                        }
                    }

                    if let Some((absolute_scanner_pos, _)) =
                        relative_positions.iter().find(|(_, v)| **v >= 12)
                    {
                        let mut s = scanner.clone();
                        s.position = Some(absolute_scanner_pos.clone());

                        let adjusted_beacon_positions = positions
                            .iter()
                            .map(|pos| pos.add(absolute_scanner_pos))
                            .collect::<Vec<_>>();
                        known_scanners.insert(s, adjusted_beacon_positions);
                    }
                }
            }
        }
    }

    known_scanners
}

fn solve_a(scanners: &HashMap<Scanner, Vec<Position>>) -> usize {
    let unique_beacons = scanners
        .values()
        .flat_map(|v| v.iter().cloned())
        .collect::<HashSet<_>>();

    unique_beacons.len()
}

fn solve_b(scanners: &HashMap<Scanner, Vec<Position>>) -> i32 {
    scanners
        .keys()
        .map(|s| s.position.as_ref().unwrap())
        .permutations(2)
        .map(|pos| pos[0].manhatten_distance(pos[1]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "--- scanner 0 ---\n\
                              404,-588,-901\n\
                              528,-643,409\n\
                              -838,591,734\n\
                              390,-675,-793\n\
                              -537,-823,-458\n\
                              -485,-357,347\n\
                              -345,-311,381\n\
                              -661,-816,-575\n\
                              -876,649,763\n\
                              -618,-824,-621\n\
                              553,345,-567\n\
                              474,580,667\n\
                              -447,-329,318\n\
                              -584,868,-557\n\
                              544,-627,-890\n\
                              564,392,-477\n\
                              455,729,728\n\
                              -892,524,684\n\
                              -689,845,-530\n\
                              423,-701,434\n\
                              7,-33,-71\n\
                              630,319,-379\n\
                              443,580,662\n\
                              -789,900,-551\n\
                              459,-707,401\n\
                              \n\
                              --- scanner 1 ---\n\
                              686,422,578\n\
                              605,423,415\n\
                              515,917,-361\n\
                              -336,658,858\n\
                              95,138,22\n\
                              -476,619,847\n\
                              -340,-569,-846\n\
                              567,-361,727\n\
                              -460,603,-452\n\
                              669,-402,600\n\
                              729,430,532\n\
                              -500,-761,534\n\
                              -322,571,750\n\
                              -466,-666,-811\n\
                              -429,-592,574\n\
                              -355,545,-477\n\
                              703,-491,-529\n\
                              -328,-685,520\n\
                              413,935,-424\n\
                              -391,539,-444\n\
                              586,-435,557\n\
                              -364,-763,-893\n\
                              807,-499,-711\n\
                              755,-354,-619\n\
                              553,889,-390\n\
                              \n\
                              --- scanner 2 ---\n\
                              649,640,665\n\
                              682,-795,504\n\
                              -784,533,-524\n\
                              -644,584,-595\n\
                              -588,-843,648\n\
                              -30,6,44\n\
                              -674,560,763\n\
                              500,723,-460\n\
                              609,671,-379\n\
                              -555,-800,653\n\
                              -675,-892,-343\n\
                              697,-426,-610\n\
                              578,704,681\n\
                              493,664,-388\n\
                              -671,-858,530\n\
                              -667,343,800\n\
                              571,-461,-707\n\
                              -138,-166,112\n\
                              -889,563,-600\n\
                              646,-828,498\n\
                              640,759,510\n\
                              -630,509,768\n\
                              -681,-892,-333\n\
                              673,-379,-804\n\
                              -742,-814,-386\n\
                              577,-820,562\n\
                              \n\
                              --- scanner 3 ---\n\
                              -589,542,597\n\
                              605,-692,669\n\
                              -500,565,-823\n\
                              -660,373,557\n\
                              -458,-679,-417\n\
                              -488,449,543\n\
                              -626,468,-788\n\
                              338,-750,-386\n\
                              528,-832,-391\n\
                              562,-778,733\n\
                              -938,-730,414\n\
                              543,643,-506\n\
                              -524,371,-870\n\
                              407,773,750\n\
                              -104,29,83\n\
                              378,-903,-323\n\
                              -778,-728,485\n\
                              426,699,580\n\
                              -438,-605,-362\n\
                              -469,-447,-387\n\
                              509,732,623\n\
                              647,635,-688\n\
                              -868,-804,481\n\
                              614,-800,639\n\
                              595,780,-596\n\
                              \n\
                              --- scanner 4 ---\n\
                              727,592,562\n\
                              -293,-554,779\n\
                              441,611,-461\n\
                              -714,465,-776\n\
                              -743,427,-804\n\
                              -660,-479,-426\n\
                              832,-632,460\n\
                              927,-485,-438\n\
                              408,393,-506\n\
                              466,436,-512\n\
                              110,16,151\n\
                              -258,-428,682\n\
                              -393,719,612\n\
                              -211,-452,876\n\
                              808,-476,-593\n\
                              -575,615,604\n\
                              -485,667,467\n\
                              -680,325,-822\n\
                              -627,-443,-432\n\
                              872,-547,-609\n\
                              833,512,582\n\
                              807,604,487\n\
                              839,-516,451\n\
                              891,-625,532\n\
                              -652,-548,-490\n\
                              30,-46,-14";

    #[test]
    fn test_solve_a() {
        let scanners = compute_absolute_beacon_positions(&PuzzleInput::new(TEST_INPUT));
        assert_eq!(solve_a(&scanners), 79);
    }

    #[test]
    fn test_solve_b() {
        let scanners = compute_absolute_beacon_positions(&PuzzleInput::new(TEST_INPUT));
        assert_eq!(solve_b(&scanners), 3621);
    }
}
