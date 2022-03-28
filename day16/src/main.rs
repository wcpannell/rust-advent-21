#[derive(Debug)]
/// Packet data type
struct Packet {
    version: u8,
    op: Content,
}

#[derive(Debug)]
/// Union-y use of struct-like-enum to hold the content of the [Packet].
enum Content {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

impl Content {
    /// Construct Content from string slice
    ///
    /// Reads Packet type ID and processes accordingly
    ///
    /// Returns tuple of the constructed enum, and the string index at which
    /// processing was completed.
    pub fn process(payload: &str) -> (Self, usize) {
        #[cfg(test)]
        println!("Processing Content: {}", &payload);
        match u8::from_str_radix(&payload[0..3], 2).unwrap() {
            4 => {
                #[cfg(test)]
                println!("found literal {}", &payload[3..]);
                let (litval, end) = Content::process_literal(&payload[3..]);

                // add start offset back in
                (Content::Literal(litval), end + 3)
            }
            x => {
                #[cfg(test)]
                println!("found operator {}", &payload[3..]);
                let (packets, end) = Content::process_operator(&payload[3..]);

                // add start offset back in
                (Content::Operator(x, packets), end + 3)
            }
        }
    }

    fn process_literal(payload: &str) -> (u64, usize) {
        #[cfg(test)]
        println!("Processing Literal: {}", payload);

        let mut value_bin: String = String::new();
        let mut i: usize = 0;
        let mut done: bool = false;
        while done == false {
            #[cfg(test)]
            println!("Processing 'byte': {}", &payload[i..i + 5]);
            value_bin.push_str(match &payload.get(i + 1..i + 5) {
                Some(x) => x,
                None => &payload.get(i + 1..).unwrap(),
            });

            // Last number is when "header" bit is low
            done = &payload[i..i + 1] == "0";
            i += 5;
        }

        #[cfg(test)]
        println!("literal last value {i}");

        return (u64::from_str_radix(value_bin.as_str(), 2).unwrap(), i);
    }

    fn process_operator(payload: &str) -> (Vec<Packet>, usize) {
        #[cfg(test)]
        println!("Processing Operator: {}", payload);
        let mut packets: Vec<Packet> = Vec::new();
        let mut i;
        match payload.chars().nth(0) {
            Some('0') => {
                i = 16; // bit after 15 bit sub-packet length identifier
                let sub_packet_end: usize = usize::from_str_radix(&payload[1..i], 2).unwrap() + i;
                #[cfg(test)]
                println!("Length of sub-packets: {}", sub_packet_end - 16);

                while (i < payload.len()) && (i < sub_packet_end) {
                    let subpacket = &payload[i..sub_packet_end];
                    #[cfg(test)]
                    println!("Proccessing subpacket {}", subpacket);
                    let (inner_packets, end) = Packet::from_bin(subpacket);
                    packets.push(inner_packets);
                    i += end;
                    #[cfg(test)]
                    println!("Start of next packet = {}", i);
                }
            }
            Some('1') => {
                // next 11 bits are number of subpackets
                i = 12; // bit after 11 bit sub-packet count identifier
                let packet_count = u32::from_str_radix(&payload[1..i], 2).unwrap();
                #[cfg(test)]
                println!("Number of subpackets expected {}", packet_count);

                for _ in 0..packet_count {
                    let subpacket = &payload[i..];
                    #[cfg(test)]
                    println!("Proccessing subpacket {}", subpacket);
                    let (packet, packet_end) = Packet::from_bin(subpacket);
                    packets.push(packet);
                    i += packet_end;
                    #[cfg(test)]
                    println!("Start of next packet = {}", i);
                }
            }
            Some(_) => panic!("Bad string value, not binary"),
            None => panic!("empty payload!"),
        }

        return (packets, i);
    }

    pub fn subpackets_version_sum(&self) -> u64 {
        let mut sum: u64 = 0;
        if let Content::Operator(_, packets) = self {
            for packet in packets {
                sum += packet.version_sum();
            }
        }
        return sum;
    }
}

impl Packet {
    fn from_bin(bin: &str) -> (Self, usize) {
        #[cfg(test)]
        println!("Procesing Packet {}", bin);

        let (op, end) = Content::process(&bin[3..]);
        (
            Packet {
                version: u8::from_str_radix(&bin[..3], 2).unwrap(),
                op,
            },
            end + 3, // add start offset back in
        )
    }

    /// Returns the sum of this packet's version and the sum of the version of
    /// all contained packets, if any, contained within.
    pub fn version_sum(&self) -> u64 {
        (self.version as u64)
            + match self.op {
                Content::Literal(_) => 0 as u64,
                Content::Operator(_, _) => self.op.subpackets_version_sum(),
            }
    }

    /// Used with the comparison operators in execute. This probably could
    /// be avoided by implementing Ord, but this was a bit more
    /// straightforward
    fn comparison_helper(packets: &Vec<Packet>) -> (u64, u64) {
        assert!(
            packets.len() == 2,
            "Bad packets for greater than operator, got {packets:#?}"
        );

        let lhs: u64 = match packets[0].op {
            Content::Literal(x) => x,
            Content::Operator(_, _) => packets[0].execute(),
        };

        let rhs: u64 = match packets[1].op {
            Content::Literal(x) => x,
            Content::Operator(_, _) => packets[1].execute(),
        };

        (lhs, rhs)
    }

    /// perform the computation denote by the packet's operator
    /// panics when called on packet containing Content::Literal
    pub fn execute(&self) -> u64 {
        match &self.op {
            // sum
            Content::Operator(0, packets) => {
                packets.iter().fold(0, |acc, packet| match packet.op {
                    Content::Literal(x) => acc + x,
                    Content::Operator(_, _) => acc + packet.execute(),
                })
            }

            // product
            Content::Operator(1, packets) => {
                // initialize accumulator as 1 for multiplication
                packets.iter().fold(1, |acc, packet| match packet.op {
                    Content::Literal(x) => acc * x,
                    Content::Operator(_, _) => acc * packet.execute(),
                })
            }

            // minimum
            Content::Operator(2, packets) => {
                let results: Vec<u64> = packets
                    .iter()
                    .map(|packet| match packet.op {
                        Content::Literal(x) => x,
                        Content::Operator(_, _) => packet.execute(),
                    })
                    .collect();
                results.iter().min().unwrap().to_owned()
            }

            // maximum
            Content::Operator(3, packets) => {
                let results: Vec<u64> = packets
                    .iter()
                    .map(|packet| match packet.op {
                        Content::Literal(x) => x,
                        Content::Operator(_, _) => packet.execute(),
                    })
                    .collect();
                results.iter().max().unwrap().to_owned()
            }

            // greater than
            Content::Operator(5, packets) => {
                let (lhs, rhs) = Packet::comparison_helper(packets);
                if lhs > rhs {
                    1u64
                } else {
                    0u64
                }
            }

            // less than
            Content::Operator(6, packets) => {
                let (lhs, rhs) = Packet::comparison_helper(packets);
                if lhs < rhs {
                    1u64
                } else {
                    0u64
                }
            }

            // equal to
            Content::Operator(7, packets) => {
                let (lhs, rhs) = Packet::comparison_helper(packets);
                if lhs == rhs {
                    1u64
                } else {
                    0u64
                }
            }
            _ => panic!("Invalid Content to execute! got {:#?}", self),
        }
    }
}

/// Turn text input file into Binary strings
fn parse_input(input: &String) -> String {
    // read text representation of hex values as numbers
    let hex: Vec<u32> = input.chars().map(|x| x.to_digit(16).unwrap()).collect();

    // translate numbers to binary
    hex.iter()
        .flat_map(|x| {
            std::fmt::format(format_args!("{:04b}", x))
                .chars()
                .collect::<Vec<char>>()
        })
        .collect()
}

fn main() {
    // Get input
    let args: Vec<String> = std::env::args().collect();
    let inputdata = match common::read_input(&args[1]) {
        Ok(val) => val,
        Err(e) => panic!("Error reading input file! {}", e),
    };
    let bin = parse_input(&inputdata[0]);
    let (outer_packet, _end) = Packet::from_bin(&bin);

    println!("Part 1: {}", outer_packet.version_sum());
    println!("Part 2: {}", outer_packet.execute());
}

#[cfg(test)]
/// Tests come from the problem statement examples
mod tests {
    use super::*;

    #[test]
    fn parseinput_literal() {
        assert_eq!(
            parse_input(&String::from("D2FE28")),
            String::from("110100101111111000101000")
        );
        assert_eq!(
            parse_input(&String::from("38006F45291200")),
            String::from("00111000000000000110111101000101001010010001001000000000")
        );
        assert_eq!(
            parse_input(&String::from("EE00D40C823060")),
            String::from("11101110000000001101010000001100100000100011000001100000")
        );
    }

    #[test]
    fn packet_literal() {
        let (packet, end) = Packet::from_bin(&parse_input(&String::from("D2FE28")));
        assert_eq!(packet.version, 6);
        assert!(matches!(packet.op, Content::Literal(x) if x == 2021));
        assert!(
            end == 21,
            "Bad end return value, expected 21, got {end}\nfrom {:#?}",
            packet
        );

        assert!(
            packet.version_sum() == 6,
            "Bad sum of packet versions, for {:#?}",
            packet
        )
    }

    #[test]
    fn packet_operator_length_type_0() {
        let (packet, _) = Packet::from_bin(&parse_input(&String::from("38006F45291200")));
        assert_eq!(packet.version, 1);
        println!("{:#?}", packet);

        // let _expected_sub_packets: Vec<Packet> = vec![
        //     Packet {
        //         version: 5,
        //         op: Content::Literal(10),
        //     },
        //     Packet {
        //         version: 2,
        //         op: Content::Literal(20),
        //     },
        // ];

        // Check packet operator
        if let Content::Operator(operation, ref sub_packets) = packet.op {
            assert_eq!(operation, 6); // correct operation
            assert_eq!(sub_packets.len(), 2); // correct number of subpackets

            // first sub_packet
            assert_eq!(sub_packets[0].version, 6); // correct version

            // correct literal value
            assert!(matches!(sub_packets[0].op, Content::Literal(x) if x == 10));

            // second sub_packet
            assert_eq!(sub_packets[1].version, 2);
            assert!(matches!(sub_packets[1].op, Content::Literal(x) if x == 20));
        } else {
            // can't get here, but in case magic happens, fail the test!
            panic!("Got bad op {:?}, failing!", packet.op);
        }

        assert!(
            packet.version_sum() == 9,
            "Bad sum of packet versions, for {:#?}",
            packet
        )
    }

    #[test]
    fn packet_operator_length_type_1() {
        // Expected Result:
        // Packet {
        //     version: 7,
        //     op: Content::Operator {
        //         2,
        //         vec![
        //             Packet {
        //                 version: 2,
        //                 op: Content::Literal(1),
        //             },
        //             Packet {
        //                 version: 4,
        //                 op: Content::Literal(2),
        //             },
        //             Packet {
        //                 version: 1,
        //                 op: Content::Literal(3),
        //             }
        //          ],
        //      }
        // ];

        let (packet, _) = Packet::from_bin(&parse_input(&String::from("EE00D40C823060")));

        assert_eq!(packet.version, 7);
        // Check packet operator
        if let Content::Operator(operation, ref sub_packets) = packet.op {
            assert!(
                operation == 3,
                "Incorrect operation value, got {}",
                operation
            );

            assert!(
                sub_packets.len() == 3,
                "Incorrect number of sub_packets, got {:#?}",
                sub_packets
            );

            // first sub_packet
            assert!(
                sub_packets[0].version == 2,
                "Incorrect version in first subpacket, got {:#?}",
                sub_packets
            );
            assert!(
                matches!(sub_packets[0].op, Content::Literal(x) if x == 1),
                "Incorrect literal value in first subpacket, got {:#?}",
                sub_packets
            );

            // second sub_packet
            assert!(
                sub_packets[1].version == 4,
                "Incorrect version in second subpacket, got {:#?}",
                sub_packets
            );
            assert!(
                matches!(sub_packets[1].op, Content::Literal(x) if x == 2),
                "Incorrect Content in second subpacket, got {:#?}",
                sub_packets
            );

            // third sub_packet
            assert!(
                sub_packets[2].version == 1,
                "Incorrect version in second subpacket, got {:#?}",
                sub_packets
            );
            assert!(
                matches!(sub_packets[2].op, Content::Literal(x) if x == 3),
                "Incorrect Content in third subpacket, got {:#?}",
                sub_packets
            );
        } else {
            panic!("Got bad op {:#?}, failing!", packet.op);
        }

        assert!(
            packet.version_sum() == 14,
            "Bad sum of packet versions, for {:#?}",
            packet
        )
    }

    #[test]
    fn nested_operator_3_deep() {
        let (packet, _) = Packet::from_bin(&parse_input(&String::from("8A004A801A8002F478")));

        assert!(
            packet.version == 4,
            "bad version, got {}\n for{:#?}",
            packet.version,
            packet,
        );

        // this functionality should be tested above? consider deletion
        if let Content::Operator(_operation, ref sub_packets) = packet.op {
            assert!(
                sub_packets.len() == 1,
                "Bad subpacket length, got {}\nfor{:#?}",
                sub_packets.len(),
                sub_packets,
            );
            // verify subpacket contains a subpacket of type Literal
            if let Content::Operator(_operation2, ref sub_packets2) = sub_packets[0].op {
                assert_eq!(sub_packets2.len(), 1);
                assert_eq!(sub_packets2[0].version, 5);

                if let Content::Operator(_operation3, ref sub_packets3) = sub_packets2[0].op {
                    assert_eq!(sub_packets3[0].version, 6);
                    assert!(
                        std::mem::discriminant(&sub_packets3[0].op)
                            == std::mem::discriminant(&Content::Literal(0)),
                        "Unexpected innermost packet type, Expected Content::Literal, got\n{:#?}",
                        sub_packets3[0]
                    ); // type, value not checked
                } else {
                    panic!(
                        "subpacket did not have a subpacket as expected, got {:#?}",
                        sub_packets2[0]
                    );
                }
            } else {
                panic!(
                    "subpacket did not have a subpacket as expected, got {:#?}",
                    sub_packets[0]
                );
            }
        } else {
            panic!(
                "Outer Packet expected to contain operator! got {:#?}",
                packet
            );
        }

        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn nested_operators_3deep_2wide() {
        let (packet, _) =
            Packet::from_bin(&parse_input(&String::from("620080001611562C8802118E34")));
        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn nested_operators_3deep_2pair_2wide_differing_type_id() {
        let (packet, end) =
            Packet::from_bin(&parse_input(&String::from("C0015000016115A2E0802F182340")));
        assert_eq!(packet.version_sum(), 23);
        assert!(
            end == 106,
            "End of Package mismatch. expected 106, got {end}"
        );
    }

    #[test]
    fn nested_operators_3deep_5wide() {
        let (packet, end) = Packet::from_bin(&parse_input(&String::from(
            "A0016C880162017C3686B18A3D4780",
        )));
        assert_eq!(packet.version_sum(), 31);
        assert!(
            end == 113,
            "End of Package mismatch. expected 106, got {end}"
        );
    }

    #[test]
    fn packet_sum() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("C200B40A82")));
        assert_eq!(packet.execute(), 3u64);
    }

    #[test]
    fn packet_product() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("04005AC33890")));
        assert_eq!(packet.execute(), 54u64);
    }

    #[test]
    fn packet_min() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("880086C3E88112")));
        assert_eq!(packet.execute(), 7u64);
    }

    #[test]
    fn packet_max() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("CE00C43D881120")));
        assert_eq!(packet.execute(), 9u64);
    }

    #[test]
    fn packet_lessthan() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("D8005AC2A8F0")));
        assert_eq!(packet.execute(), 1u64);
    }

    #[test]
    fn packet_greaterthan() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("F600BC2D8F")));
        assert_eq!(packet.execute(), 0u64);
    }

    #[test]
    fn packet_equalto() {
        let (packet, _end) = Packet::from_bin(&parse_input(&String::from("9C005AC2F8F0")));
        assert_eq!(packet.execute(), 0u64);
    }

    #[test]
    fn packet_equalto_2deep_2pair() {
        let (packet, _end) =
            Packet::from_bin(&parse_input(&String::from("9C0141080250320F1802104A08")));
        assert_eq!(packet.execute(), 1u64);
    }
}
