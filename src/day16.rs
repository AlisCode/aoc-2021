//! Handwritten parser for BITS protocol that could have been written with `nom`.
//! Code is clear enough so I'm fine with that :)

#[aoc_generator(day16)]
fn parse(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8, // Actually a u3
    kind: PacketKind,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketKind {
    // I don't think we're going to need more than 64 bytes for the literals, sadly, this is unspecified by the problem
    Literal(u64),
    Operator(OperatorPacket),
}

#[derive(Debug, PartialEq, Eq)]
// Weakyly-typed Operator Packet representation
// `packets` is an unsafe collection of all sub-packets.
// Effort could be made to represent this better by strong-typing using the OperatorKind enum.
// Sadly, Aint nobody's got time for that.
struct OperatorPacket {
    kind: OperatorKind,
    packets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq)]
enum OperatorKind {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

// Parsing methods : returns data + length of parsed element in bits
impl Packet {
    fn parse(input: &str) -> (Self, usize) {
        let version: u8 = u8::from_str_radix(&input[0..3], 2).unwrap();
        let packet_type: u8 = u8::from_str_radix(&input[3..6], 2).unwrap();
        let (kind, size) = match packet_type {
            4 => {
                let (x, size) = Packet::parse_literal(&input[6..]);
                (PacketKind::Literal(x), size)
            }
            x => {
                let (packets, size) = Packet::parse_operator(&input[6..]);
                let kind = match x {
                    0 => OperatorKind::Sum,
                    1 => OperatorKind::Product,
                    2 => OperatorKind::Min,
                    3 => OperatorKind::Max,
                    5 => OperatorKind::Greater,
                    6 => OperatorKind::Less,
                    7 => OperatorKind::Equal,
                    _ => panic!("Unsupported OperatorKind"), // Yeah yeah, this could use bettor error handling
                };
                (PacketKind::Operator(OperatorPacket { kind, packets }), size)
            }
        };
        (Packet { version, kind }, size + 6)
    }

    // Takes 5 bits, checks first bit to know if we should keep reading, bitshift and then adds 4 bits
    // Conveniently, MSB comes first so we don't need no black magic :)
    // See test should_parse_literal
    fn parse_literal(input: &str) -> (u64, usize) {
        let mut total: u64 = 0;
        for x in 0.. {
            let idx = x * 5;
            total = total << 4;
            let parsed = u64::from_str_radix(&input[idx + 1..idx + 5], 2).unwrap();
            if &input[idx..=idx] == "0" {
                total += parsed;
                return (total, (x + 1) * 5);
            } else {
                total += parsed;
            }
        }
        unreachable!() // Actually unreachable because the for loop on top never ends
    }

    // Depending on the first bit, we want to read either 15 or 11 bits.
    // The parsing method of an operator ticket depends on that "mode".
    fn parse_operator(input: &str) -> (Vec<Packet>, usize) {
        let mut packets = Vec::new();
        // See test should_parse_operator_0
        if &input[0..=0] == "0" {
            let total_length = usize::from_str_radix(&input[1..16], 2).unwrap();
            let mut idx = 0;
            while idx < total_length {
                let (packet, size) = Packet::parse(&input[16 + idx..]);
                packets.push(packet);
                idx += size;
            }
            (packets, 16 + idx)
        } else {
            // See test should_parse_operator_0
            let nb_packets = usize::from_str_radix(&input[1..12], 2).unwrap();
            let mut idx = 0;
            for _ in 0..nb_packets {
                let (packet, size) = Packet::parse(&input[12 + idx..]);
                packets.push(packet);
                idx += size;
            }
            (packets, 12 + idx)
        }
    }

    // Trivial version calculation
    fn version_sum(&self) -> u64 {
        match &self.kind {
            PacketKind::Literal(_) => self.version as u64,
            PacketKind::Operator(OperatorPacket { packets, .. }) => {
                packets.iter().map(Packet::version_sum).sum::<u64>() + self.version as u64
            }
        }
    }

    // Evaluates the packet
    fn value(&self) -> u64 {
        match &self.kind {
            PacketKind::Literal(x) => *x,
            PacketKind::Operator(OperatorPacket { packets, kind }) => {
                let values: Vec<u64> = packets.iter().map(Packet::value).collect();
                match kind {
                    OperatorKind::Sum => values.iter().sum(),
                    OperatorKind::Product => values.iter().product(),
                    OperatorKind::Min => *values.iter().min().unwrap(),
                    OperatorKind::Max => *values.iter().max().unwrap(),
                    OperatorKind::Greater => {
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorKind::Less => {
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorKind::Equal => {
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let (packet, _size) = Packet::parse(input);
    packet.version_sum()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let (packet, _size) = Packet::parse(input);
    packet.value()
}

#[cfg(test)]
/// Fairly exhaustive testing of the BITS protocol!
pub mod tests {
    use super::*;

    const INPUT_LITERAL: &'static str = "D2FE28";
    const INPUT_OPERATOR_0: &'static str = "38006F45291200";
    const INPUT_OPERATOR_1: &'static str = "EE00D40C823060";

    #[test]
    fn should_parse_literal() {
        let (packet, _size) = Packet::parse(&parse(INPUT_LITERAL));
        assert_eq!(
            packet,
            Packet {
                version: 6,
                kind: PacketKind::Literal(2021)
            }
        );
    }

    #[test]
    fn should_parse_operator_0() {
        let (packet, _size) = Packet::parse(&parse(INPUT_OPERATOR_0));
        assert_eq!(
            packet,
            Packet {
                version: 1,
                kind: PacketKind::Operator(OperatorPacket {
                    kind: OperatorKind::Less,
                    packets: vec![
                        Packet {
                            version: 6,
                            kind: PacketKind::Literal(10),
                        },
                        Packet {
                            version: 2,
                            kind: PacketKind::Literal(20),
                        },
                    ],
                })
            }
        );
    }

    #[test]
    fn should_parse_operator_1() {
        let (packet, _size) = Packet::parse(&parse(INPUT_OPERATOR_1));
        assert_eq!(
            packet,
            Packet {
                version: 7,
                kind: PacketKind::Operator(OperatorPacket {
                    kind: OperatorKind::Max,
                    packets: vec![
                        Packet {
                            version: 2,
                            kind: PacketKind::Literal(1),
                        },
                        Packet {
                            version: 4,
                            kind: PacketKind::Literal(2),
                        },
                        Packet {
                            version: 1,
                            kind: PacketKind::Literal(3),
                        },
                    ],
                })
            }
        );
    }

    #[test]
    fn part1_should_work() {
        assert_eq!(part1(&parse("8A004A801A8002F478")), 16);
        assert_eq!(part1(&parse("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&parse("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn part2_should_work() {
        assert_eq!(part2(&parse("C200B40A82")), 3);
        assert_eq!(part2(&parse("04005AC33890")), 54);
        assert_eq!(part2(&parse("880086C3E88112")), 7);
        assert_eq!(part2(&parse("CE00C43D881120")), 9);
        assert_eq!(part2(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part2(&parse("F600BC2D8F")), 0);
        assert_eq!(part2(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part2(&parse("9C0141080250320F1802104A08")), 1);
    }
}
