use std::{cmp::Ordering, fs, time::Instant};

fn part_one(input: &[String]) -> String {
    let pairs = PacketPair::parse_input(input);

    let result: usize = pairs
        .iter()
        .map(|pair| pair.order())
        .enumerate()
        .map(|(i, order)| (i + 1, order))
        .filter(|(_, order)| matches!(order, Order::Right))
        .map(|(i, _)| i)
        .sum();

    result.to_string()
}

fn part_two(input: &[String]) -> String {
    let mut packets: Vec<_> = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line.trim().as_bytes()).unwrap())
        .collect();

    let distress_signal_1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let distress_signal_2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    packets.push(distress_signal_1.clone());
    packets.push(distress_signal_2.clone());

    packets.sort();

    let result: usize = packets
        .iter()
        .enumerate()
        .map(|(i, packet)| (i + 1, packet))
        .filter(|(_, packet)| {
            matches!((*packet).cmp(&distress_signal_1), Ordering::Equal)
                || matches!((*packet).cmp(&distress_signal_2), Ordering::Equal)
        })
        .map(|(i, _)| i)
        .product();

    result.to_string()
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

#[derive(Debug)]
enum Order {
    Right,
    Wrong,
    Equal,
}

impl PacketPair {
    fn parse_input(input: &[String]) -> Vec<Self> {
        input
            .chunks(3)
            .map(|chunk| {
                Self::parse(chunk.first().unwrap(), chunk.get(1).unwrap())
                    .unwrap_or_else(|| panic!("Error parsing chunk: {chunk:?}"))
            })
            .collect()
    }

    fn parse(first: &str, second: &str) -> Option<Self> {
        Some(Self {
            left: Packet::parse(first.trim().as_bytes())?,
            right: Packet::parse(second.trim().as_bytes())?,
        })
    }

    fn order(&self) -> Order {
        match self.left.cmp(&self.right) {
            Ordering::Equal => Order::Equal,
            Ordering::Less => Order::Right,
            Ordering::Greater => Order::Wrong,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => Some(l.cmp(r)),
            (Self::List(l), Self::List(r)) => {
                for (l_pack, r_pack) in l.iter().zip(r) {
                    match l_pack.partial_cmp(r_pack) {
                        Some(Ordering::Equal) => (),
                        other => return other,
                    }
                }

                Some(l.len().cmp(&r.len()))
            }
            (list, Self::Number(r)) => list.partial_cmp(&Self::List(vec![Self::Number(*r)])),
            (Self::Number(l), list) => Self::List(vec![Self::Number(*l)]).partial_cmp(list),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Packet {
    fn parse(mut line: &[u8]) -> Option<Self> {
        let mut list = vec![];

        while let Some(&char) = line.first() {
            if char == b'[' {
                let end = line.iter().position(|c| *c == b']').unwrap_or(line.len());
                list.push(Self::parse(&line[1..end])?);

                line = if end >= line.len() {
                    &line[0..0]
                } else {
                    &line[(end + 1)..]
                };
            } else if char.is_ascii_digit() {
                let end = line
                    .iter()
                    .position(|c| *c == b',' || *c == b']')
                    .unwrap_or(line.len());
                let number = String::from_utf8_lossy(&line[0..end]);
                let number = number.parse().unwrap_or_else(|_| {
                    panic!("Invalid number: {number}, end: {end}, line: {line:?}")
                });

                list.push(Self::Number(number));

                line = if end >= line.len() {
                    &line[0..0]
                } else {
                    &line[(end + 1)..]
                };
            } else {
                line = &line[1..]
            }
        }

        Some(Self::List(list))
    }
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "140");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
