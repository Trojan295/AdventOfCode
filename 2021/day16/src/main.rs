type BitVec = Vec<bool>;

fn bitvec_from_hex(x: &str) -> BitVec {
    let intvec = x
        .chars()
        .map(|x| match x {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => vec![],
        })
        .flatten()
        .collect();

    bitvec_from_intvec(&intvec)
}

fn bitvec_from_intvec(x: &Vec<i32>) -> BitVec {
    x.into_iter().map(|x| *x == 1).collect()
}

fn bitvec_to_u64(bv: &BitVec) -> u64 {
    let mut val = 0;

    for (pos, bit) in bv.iter().rev().enumerate() {
        let b = match bit {
            true => 1,
            false => 0,
        };

        val += b << pos;
    }

    val
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u64,
    content: PacketContent,
}

#[derive(PartialEq, Debug)]
enum PacketContent {
    Value(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

fn parse_packet<'a, I>(bits: &mut I) -> (Packet, usize)
where
    I: Iterator<Item = &'a bool>,
{
    let mut consumed_bits = 0;
    let packet_iter = bits.by_ref();

    let version_bits: BitVec = packet_iter.take(3).map(|x| *x).collect();
    let type_id_bits: BitVec = packet_iter.take(3).take(3).map(|x| *x).collect();
    consumed_bits += 6;

    let type_id = bitvec_to_u64(&type_id_bits);

    let content = match type_id {
        4 => {
            let mut value_bits = vec![];

            let mut cont = true;
            while cont {
                let mut bits = packet_iter.take(5);
                consumed_bits += 5;

                cont = *bits.next().unwrap_or(&false);
                let mut bits: Vec<bool> = bits.map(|x| *x).collect();
                value_bits.append(&mut bits);
            }

            PacketContent::Value(bitvec_to_u64(&value_bits))
        }
        _ => {
            let (packets, consumed) = parse_content_packets(packet_iter);
            consumed_bits += consumed;

            match type_id {
                1 => PacketContent::Product(packets),
                2 => PacketContent::Minimum(packets),
                3 => PacketContent::Maximum(packets),
                5 => PacketContent::GreaterThan(packets),
                6 => PacketContent::LessThan(packets),
                7 => PacketContent::EqualTo(packets),
                _ => PacketContent::Sum(packets),
            }
        }
    };

    (
        Packet {
            version: bitvec_to_u64(&version_bits),
            content,
        },
        consumed_bits,
    )
}

fn parse_content_packets<'a, I>(bits: &mut I) -> (Vec<Packet>, usize)
where
    I: Iterator<Item = &'a bool>,
{
    let mut packets = vec![];
    let mut consumed_bits = 0;

    let length_type_id = *bits.take(1).next().unwrap_or(&false);
    consumed_bits += 1;

    if length_type_id {
        let length_bits = bits.take(11).map(|x| *x).collect();
        consumed_bits += 11;

        let packet_number = bitvec_to_u64(&length_bits);

        for _ in 0..packet_number {
            let (packet, consumed) = parse_packet(bits);
            consumed_bits += consumed;

            packets.push(packet);
        }
    } else {
        let length_bits = bits.take(15).map(|x| *x).collect();
        consumed_bits += 15;

        let mut remaining_bits = bitvec_to_u64(&length_bits) as i64;

        while remaining_bits > 0 {
            let (packet, consumed) = parse_packet(bits);
            consumed_bits += consumed;
            remaining_bits -= consumed as i64;

            packets.push(packet);
        }
    }

    (packets, consumed_bits)
}

#[test]
fn test_parse_packet() {
    let input = bitvec_from_hex("D2FE28");

    println!("{:?}", input);

    let mut bit_input = input.iter();

    let (packet, _) = parse_packet(&mut bit_input);
    assert_eq!(
        packet,
        Packet {
            version: 6,
            content: PacketContent::Value(2021),
        }
    );

    let input = bitvec_from_hex("38006F45291200");
    let mut bit_input = input.iter();

    let (packet, _) = parse_packet(&mut bit_input);
    assert_eq!(
        packet,
        Packet {
            version: 1,
            content: PacketContent::Sum(vec![
                Packet {
                    version: 6,
                    content: PacketContent::Value(10)
                },
                Packet {
                    version: 2,
                    content: PacketContent::Value(20)
                },
            ]),
        }
    );

    let input = bitvec_from_hex("EE00D40C823060");
    let mut bit_input = input.iter();

    let (packet, _) = parse_packet(&mut bit_input);
    assert_eq!(
        packet,
        Packet {
            version: 7,
            content: PacketContent::Sum(vec![
                Packet {
                    version: 2,
                    content: PacketContent::Value(1)
                },
                Packet {
                    version: 4,
                    content: PacketContent::Value(2)
                },
                Packet {
                    version: 1,
                    content: PacketContent::Value(3)
                }
            ]),
        }
    )
}

fn main() {
    let input = "E20D4100AA9C0199CA6A3D9D6352294D47B3AC6A4335FBE3FDD251003873657600B46F8DC600AE80273CCD2D5028B6600AF802B2959524B727D8A8CC3CCEEF3497188C017A005466DAA6FDB3A96D5944C014C006865D5A7255D79926F5E69200A164C1A65E26C867DDE7D7E4794FE72F3100C0159A42952A7008A6A5C189BCD456442E4A0A46008580273ADB3AD1224E600ACD37E802200084C1083F1540010E8D105A371802D3B845A0090E4BD59DE0E52FFC659A5EBE99AC2B7004A3ECC7E58814492C4E2918023379DA96006EC0008545B84B1B00010F8E915E1E20087D3D0E577B1C9A4C93DD233E2ECF65265D800031D97C8ACCCDDE74A64BD4CC284E401444B05F802B3711695C65BCC010A004067D2E7C4208A803F23B139B9470D7333B71240050A20042236C6A834600C4568F5048801098B90B626B00155271573008A4C7A71662848821001093CB4A009C77874200FCE6E7391049EB509FE3E910421924D3006C40198BB11E2A8803B1AE2A4431007A15C6E8F26009E002A725A5292D294FED5500C7170038C00E602A8CC00D60259D008B140201DC00C401B05400E201608804D45003C00393600B94400970020C00F6002127128C0129CDC7B4F46C91A0084E7C6648DC000DC89D341B23B8D95C802D09453A0069263D8219DF680E339003032A6F30F126780002CC333005E8035400042635C578A8200DC198890AA46F394B29C4016A4960C70017D99D7E8AF309CC014FCFDFB0FE0DA490A6F9D490010567A3780549539ED49167BA47338FAAC1F3005255AEC01200043A3E46C84E200CC4E895114C011C0054A522592912C9C8FDE10005D8164026C70066C200C4618BD074401E8C90E23ACDFE5642700A6672D73F285644B237E8CCCCB77738A0801A3CFED364B823334C46303496C940";
    let bits = bitvec_from_hex(input);

    let (packet, _) = parse_packet(&mut bits.iter());
    println!("Part1: {}", sum_versions(&packet));
    println!("Part2: {}", evaluate(&packet));
}

fn sum_versions(p: &Packet) -> u64 {
    p.version
        + match &p.content {
            PacketContent::Value(_) => 0,
            PacketContent::Sum(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::EqualTo(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::GreaterThan(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::LessThan(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::Product(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::Minimum(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
            PacketContent::Maximum(packets) => packets.iter().map(|p| sum_versions(p)).sum(),
        }
}

fn evaluate(p: &Packet) -> u64 {
    match &p.content {
        PacketContent::Value(v) => *v,
        PacketContent::EqualTo(packets) => {
            if evaluate(&packets[0]) == evaluate(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketContent::LessThan(packets) => {
            if evaluate(&packets[0]) < evaluate(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketContent::GreaterThan(packets) => {
            if evaluate(&packets[0]) > evaluate(&packets[1]) {
                1
            } else {
                0
            }
        }
        PacketContent::Maximum(packets) => packets.iter().map(|p| evaluate(p)).max().unwrap_or(0),
        PacketContent::Minimum(packets) => packets.iter().map(|p| evaluate(p)).min().unwrap_or(0),
        PacketContent::Product(packets) => packets
            .iter()
            .map(|p| evaluate(p))
            .fold(1, |acc, v| acc * v),
        PacketContent::Sum(packets) => packets
            .iter()
            .map(|p| evaluate(p))
            .fold(0, |acc, v| acc + v),
    }
}
