use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day16() {
    let f: File = File::open("data/day16.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let diagnostics: String = reader.lines().next().unwrap().unwrap();

    let mut bit_string: String = "".to_string();
    for c in diagnostics.chars() {
        match c {
            '0' => {
                bit_string.push_str("0000");
            }
            '1' => {
                bit_string.push_str("0001");
            }
            '2' => {
                bit_string.push_str("0010");
            }
            '3' => {
                bit_string.push_str("0011");
            }
            '4' => {
                bit_string.push_str("0100");
            }
            '5' => {
                bit_string.push_str("0101");
            }
            '6' => {
                bit_string.push_str("0110");
            }
            '7' => {
                bit_string.push_str("0111");
            }
            '8' => {
                bit_string.push_str("1000");
            }
            '9' => {
                bit_string.push_str("1001");
            }
            'A' => {
                bit_string.push_str("1010");
            }
            'B' => {
                bit_string.push_str("1011");
            }
            'C' => {
                bit_string.push_str("1100");
            }
            'D' => {
                bit_string.push_str("1101");
            }
            'E' => {
                bit_string.push_str("1110");
            }
            'F' => {
                bit_string.push_str("1111");
            }
            _ => {
                panic!("Unexpected character");
            }
        }
    }

    let mut packet_version_sum: u128 = 0;
    // This day needs some tidy-up in general - but nothing embodies that more
    // than the absolutely absurd signature of this decode_packet function.
    let result = decode_packet(&bit_string, &mut packet_version_sum);
    println!("Packet version sum is {}", packet_version_sum);
    println!("Packet value is {}", result.0);
}

fn calculate_packet_value(packet_type_id: u128, subpacket_values: &[u128]) -> u128 {
    let packet_value: u128;
    match packet_type_id {
        0 => {
            packet_value = subpacket_values.iter().sum();
        }
        1 => {
            packet_value = subpacket_values.iter().product();
        }
        2 => {
            packet_value = *subpacket_values.iter().min().unwrap();
        }
        3 => {
            packet_value = *subpacket_values.iter().max().unwrap();
        }
        5 => {
            if subpacket_values.get(0) > subpacket_values.get(1) {
                packet_value = 1;
            } else {
                packet_value = 0;
            };
        }
        6 => {
            if subpacket_values.get(0) < subpacket_values.get(1) {
                packet_value = 1;
            } else {
                packet_value = 0;
            };
        }
        7 => {
            if subpacket_values.get(0) == subpacket_values.get(1) {
                packet_value = 1;
            } else {
                packet_value = 0;
            };
        }
        _ => {
            panic!("Unexpected packet type ID");
        }
    }
    packet_value
}

fn decode_packet(packet: &str, packet_version_sum: &mut u128) -> (u128, usize) {
    let packet_value: u128;
    let mut cursor: usize = 0;
    let packet_version = decode_bits(packet.get(cursor..cursor + 3).unwrap());
    cursor += 3;
    *packet_version_sum += packet_version;
    let packet_type_id = decode_bits(packet.get(cursor..cursor + 3).unwrap());
    cursor += 3;
    if packet_type_id != 4 {
        let mut subpacket_values: Vec<u128> = Vec::new();
        // Operator packet
        cursor += 1;
        if packet.chars().nth(cursor - 1).unwrap() == '0' {
            // The next 15 bits are a number that represents the total length
            // in bits of the sub-packets contained by this packet.
            let sub_packets_length = decode_bits(packet.get(cursor..cursor + 15).unwrap());
            cursor += 15;
            let mut sub_packets_length_so_far: usize = 0;
            while sub_packets_length_so_far < sub_packets_length as usize {
                let l = decode_packet(
                    packet.get(cursor..packet.len()).unwrap(),
                    packet_version_sum,
                );
                cursor += l.1;
                sub_packets_length_so_far += l.1;
                subpacket_values.push(l.0);
            }
        } else {
            // The next 11 bits are a number that represents the number of
            // sub-packets immediately contained by this packet.
            let number_of_sub_packets = decode_bits(packet.get(cursor..cursor + 11).unwrap());
            cursor += 11;
            for _ in 0..number_of_sub_packets {
                let l = decode_packet(
                    packet.get(cursor..packet.len()).unwrap(),
                    packet_version_sum,
                );
                cursor += l.1;
                subpacket_values.push(l.0);
            }
        }
        packet_value = calculate_packet_value(packet_type_id, &subpacket_values);
    } else {
        let mut literal_bits: String = "".to_string();
        loop {
            literal_bits.push_str(packet.get(cursor + 1..cursor + 5).unwrap());
            cursor += 5;
            if packet.chars().nth(cursor - 5).unwrap() == '0' {
                break;
            }
        }
        let literal_value = decode_bits(&literal_bits);
        packet_value = literal_value;
    }

    (packet_value, cursor)
}

fn decode_bits(bits: &str) -> u128 {
    isize::from_str_radix(bits, 2).unwrap() as u128
}
