#[allow(dead_code)]
pub fn run() {
    let input = include_str!("../inputs/16.txt");
    let hex = input.lines().next().unwrap();
    let binary = hex
        .chars()
        .map(|c| c.to_digit(16).map(|n| format!("{:04b}", n)).unwrap())
        .collect::<String>();
    let input = binary
        .chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect::<Vec<u8>>();

    let (packet, _) = parsing::parse_packet(&input);

    eprintln!("value: {}", packet.eval());
}

mod parsing {
    #[derive(Debug, Clone)]
    pub enum PacketContent {
        Literal(usize),
        Sum(Vec<Packet>),
        Product(Vec<Packet>),
        Minimum(Vec<Packet>),
        Maxmium(Vec<Packet>),
        GreaterThan(Box<Packet>, Box<Packet>),
        LessThan(Box<Packet>, Box<Packet>),
        EqualTo(Box<Packet>, Box<Packet>),
    }

    #[derive(Debug, Clone)]
    pub struct Packet {
        pub header: Header,
        pub content: PacketContent,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Header {
        pub version: u8,
        pub type_id: u8,
    }

    #[derive(Debug, Clone, Copy)]
    enum OpLen {
        Total(usize),
        NumPackets(usize),
    }

    impl Packet {
        pub fn eval(&self) -> usize {
            match &self.content {
                PacketContent::Literal(v) => *v,
                PacketContent::Sum(p) => p.iter().map(|p| p.eval()).sum(),
                PacketContent::Product(p) => p.iter().map(|p| p.eval()).fold(1, |acc, v| acc * v),
                PacketContent::Minimum(p) => p.iter().map(|p| p.eval()).min().unwrap(),
                PacketContent::Maxmium(p) => p.iter().map(|p| p.eval()).max().unwrap(),
                PacketContent::GreaterThan(a, b) => {
                    if a.eval() > b.eval() {
                        1
                    } else {
                        0
                    }
                }
                PacketContent::LessThan(a, b) => {
                    if a.eval() < b.eval() {
                        1
                    } else {
                        0
                    }
                }
                PacketContent::EqualTo(a, b) => {
                    if a.eval() == b.eval() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }

    pub fn parse_packet(input: &[u8]) -> (Packet, &[u8]) {
        let (header, input) = parse_header(&input);
        let (packet, input) = if header.type_id == 4 {
            let mut groups = Vec::new();
            let mut input = input;
            loop {
                let last_block = input[0] == 0;
                groups.push(&input[1..5]);
                input = &input[5..];
                if last_block {
                    break;
                }
            }
            let num = groups.into_iter().flatten().copied().collect::<Vec<_>>();
            let num = to_number(&num[..]);
            (
                Packet {
                    header,
                    content: PacketContent::Literal(num),
                },
                input,
            )
        } else {
            let (len, input) = {
                let remaining;
                let len_type = input[0];
                let len = if len_type == 0 {
                    let len = OpLen::Total(to_number(&input[1..16]));
                    remaining = &input[16..];
                    len
                } else {
                    let len = OpLen::NumPackets(to_number(&input[1..12]));
                    remaining = &input[12..];
                    len
                };
                (len, remaining)
            };

            let mut packets = Vec::new();
            let mut input = input;
            match len {
                OpLen::NumPackets(n) => {
                    for _ in 0..n {
                        let (packet, remaining) = parse_packet(input);
                        packets.push(packet);
                        input = remaining;
                    }
                }
                OpLen::Total(n) => {
                    let mut inner_input = &input[0..n];
                    input = &input[n..];

                    while inner_input.len() > 0 {
                        let (packet, remaining) = parse_packet(inner_input);
                        packets.push(packet);
                        inner_input = remaining;
                    }
                }
            };
            let content = match header.type_id {
                0 => PacketContent::Sum(packets),
                1 => PacketContent::Product(packets),
                2 => PacketContent::Minimum(packets),
                3 => PacketContent::Maxmium(packets),
                v => {
                    let b = Box::new(packets.pop().unwrap());
                    let a = Box::new(packets.pop().unwrap());
                    match v {
                        5 => PacketContent::GreaterThan(a, b),
                        6 => PacketContent::LessThan(a, b),
                        7 => PacketContent::EqualTo(a, b),
                        v => panic!("type_id out of range: {}", v),
                    }
                }
            };
            (Packet { header, content }, input)
        };
        (packet, input)
    }

    fn parse_header(input: &[u8]) -> (Header, &[u8]) {
        let version = to_number(&input[0..3]) as u8;
        let type_id = to_number(&input[3..6]) as u8;
        (Header { version, type_id }, &input[6..])
    }

    fn to_number(binary: &[u8]) -> usize {
        let binary_str = binary
            .iter()
            .filter_map(|n| char::from_digit((*n).into(), 10))
            .collect::<String>();
        usize::from_str_radix(binary_str.as_str(), 2).unwrap()
    }
}
