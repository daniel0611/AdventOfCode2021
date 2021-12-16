use aoc_utils::PuzzleInput;
const DAY: u8 = 16;

struct Packet {
    version: u8,
    type_id: u8,
    value: Option<u64>,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn parse_from_input(input: &PuzzleInput) -> Packet {
        let mut bit_string = String::default();

        for c in input.raw_input.chars() {
            // TODO: could be made cleaner by using a function to parse the hex number and convert it to binary
            let binary = match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => panic!("Invalid character: {}", c),
            };

            bit_string.push_str(binary);
        }

        Self::parse_bit_string(bit_string).0
    }

    fn parse_bit_string(s: String) -> (Packet, usize) {
        if s.len() < 7 {
            panic!("Invalid bit string: {}", s);
        }

        let version_bits = &s[0..=2];
        let version = u8::from_str_radix(version_bits, 2).unwrap();
        let type_id_bits = &s[3..=5];
        let type_id = u8::from_str_radix(type_id_bits, 2).unwrap();

        let mut value = None;
        let mut sub_packets = Vec::new();
        let mut current_index = 5;

        if type_id == 4 {
            // Literal
            let mut number_bits = String::default();
            loop {
                let group = s
                    .chars()
                    .skip(current_index + 1)
                    .take(5)
                    .collect::<String>();
                current_index += 5;
                number_bits.push_str(&group[1..=4]);

                // Check if this was the last group
                if group.starts_with('0') {
                    break;
                }
            }

            value = Some(u64::from_str_radix(&number_bits, 2).unwrap());
        } else {
            // Operator
            let length_type_bit = &s.chars().nth(6).unwrap();
            current_index += 1;
            let mut packet_count = -1;
            let mut length_in_bits = s.len() - 7 - 11;
            let mut bits_consumed = 0;

            if *length_type_bit == '1' {
                let length_bits = &s
                    .chars()
                    .skip(current_index + 1)
                    .take(11)
                    .collect::<String>();
                packet_count = usize::from_str_radix(length_bits, 2).unwrap() as isize;
                current_index += 11;
            } else {
                let length_bits = &s
                    .chars()
                    .skip(current_index + 1)
                    .take(15)
                    .collect::<String>();
                length_in_bits = usize::from_str_radix(length_bits, 2).unwrap();
                current_index += 15;
            }

            loop {
                let (packet, length) =
                    Self::parse_bit_string(s.chars().skip(current_index + 1).collect::<String>());
                sub_packets.push(packet);
                current_index += length;
                bits_consumed += length;
                packet_count -= 1;

                if packet_count == 0
                    || bits_consumed >= length_in_bits
                    || s.chars().skip(current_index + 1).all(|c| c == '0')
                {
                    break;
                }
            }
        }

        (
            Packet {
                version,
                type_id,
                value,
                sub_packets,
            },
            current_index + 1,
        )
    }

    fn sum_versions(&self) -> usize {
        self.version as usize
            + self
                .sub_packets
                .iter()
                .map(|p| p.sum_versions())
                .sum::<usize>()
    }

    fn print(&self, indentation: u8) {
        let whitespaces = " ".repeat(indentation as usize);
        println!("{}Version: {}", whitespaces, self.version);
        println!("{}Type ID: {}", whitespaces, self.type_id);
        println!("{}Value: {:?}", whitespaces, self.value);
        for packet in &self.sub_packets {
            packet.print(indentation + 2);
        }
    }

    fn evalulate(&self) -> u64 {
        let sub_values = self.sub_packets.iter().map(|p| p.evalulate());

        match self.type_id {
            0 => sub_values.sum(),
            1 => sub_values.product(),
            2 => sub_values.min().unwrap(),
            3 => sub_values.max().unwrap(),
            4 => self.value.unwrap(),
            t => {
                let values = sub_values.collect::<Vec<u64>>();
                let first = values[0];
                let second = values[1];

                let bool_val = match t {
                    5 => first > second,
                    6 => first < second,
                    7 => first == second,
                    _ => panic!("Invalid type ID: {}", t),
                };

                if bool_val {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    let packet = Packet::parse_from_input(input);
    packet.print(0);
    packet.sum_versions()
}

fn solve_b(input: &PuzzleInput) -> u64 {
    let packet = Packet::parse_from_input(input);
    packet.evalulate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_parse_literal() {
        let packet = Packet::parse_from_input(&PuzzleInput::new("D2FE28"));
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.value, Some(2021));
    }

    #[test]
    fn test_parse_operator() {
        let packet = Packet::parse_from_input(&PuzzleInput::new("38006F45291200"));
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.value, None);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets[0].value, Some(10));
        assert_eq!(packet.sub_packets[1].value, Some(20));
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new("8A004A801A8002F478")), 16);
        assert_eq!(solve_a(&PuzzleInput::new("620080001611562C8802118E34")), 12);
        assert_eq!(
            solve_a(&PuzzleInput::new("C0015000016115A2E0802F182340")),
            23
        );
        assert_eq!(
            solve_a(&PuzzleInput::new("A0016C880162017C3686B18A3D4780")),
            31
        );
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new("C200B40A82")), 3);
        assert_eq!(solve_b(&PuzzleInput::new("04005AC33890")), 54);
        assert_eq!(solve_b(&PuzzleInput::new("880086C3E88112")), 7);
        assert_eq!(solve_b(&PuzzleInput::new("CE00C43D881120")), 9);
        assert_eq!(solve_b(&PuzzleInput::new("D8005AC2A8F0")), 1);
        assert_eq!(solve_b(&PuzzleInput::new("F600BC2D8F")), 0);
        assert_eq!(solve_b(&PuzzleInput::new("9C005AC2F8F0")), 0);
        assert_eq!(solve_b(&PuzzleInput::new("9C0141080250320F1802104A08")), 1);
    }
}
