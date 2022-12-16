use std::cmp::Ordering;
use Ordering::{Equal, Greater, Less};

pub fn part1(data: String) -> String {
    parse(&data)
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
        .sum::<usize>()
        .to_string()
}

pub fn part2(data: String) -> String {
    let packet_pairs = parse(&data);
    let mut packets: Vec<&Packet> = packet_pairs
        .iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .collect();

    let d1 = divider(2);
    let d2 = divider(6);
    packets.append(&mut vec![&d1, &d2]);
    packets.sort();

    let dividers = vec![divider(2), divider(6)];
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if dividers.contains(packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>()
        .to_string()
}

fn divider(n: u32) -> Packet {
    List(vec![List(vec![Integer(n)])])
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}
use Packet::*;

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // easy case - 2 ints
            (Integer(i1), Integer(i2)) => i1.cmp(i2),

            //  int and list - wrap the int, and recursively call cmp
            (Integer(i1), l2 @ List(_)) => List(vec![Integer(*i1)]).cmp(l2),
            (l1 @ List(_), Integer(i2)) => l1.cmp(&List(vec![Integer(*i2)])),

            // two lists, step through each
            (List(l1), List(l2)) => {
                let mut it1 = l1.iter();
                let mut it2 = l2.iter();
                // manual loop instead of zip so we can check what's left after we're done
                loop {
                    match (it1.next(), it2.next()) {
                        // both lists have items - call cmp recursively
                        // if result is less or greater, return that, otherwise carry on
                        (Some(p1), Some(p2)) => match p1.cmp(p2) {
                            ord @ Less => return ord,
                            ord @ Greater => return ord,
                            Ordering::Equal => (),
                        },

                        // 1st list is consumed, but stuff left in 2nd
                        (None, Some(_p2)) => return Less,

                        // 1st list has stuff left, but 2nd consumed
                        (Some(_p1), None) => return Greater,

                        // both lists ran out at the same time
                        (None, None) => return Equal,
                    }
                }
                // we're done comparing items. check what's left, and decide based on that
            }
        }
    }
}

type PacketPair = (Packet, Packet);

type PacketPairs = Vec<PacketPair>;

fn parse(s: &str) -> PacketPairs {
    let (_rest, packet_pairs) = packet_pairs(s).unwrap();
    packet_pairs
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::map,
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Clone)]
enum PacketToken {
    Start,
    End,
    Sep,
    EOL,
}

fn packet_pairs(s: &str) -> IResult<&str, PacketPairs> {
    separated_list0(tuple((eol, eol)), packet_pair)(s)
}
fn packet_pair(s: &str) -> IResult<&str, PacketPair> {
    separated_pair(packet, eol, packet)(s)
}

fn packet(s: &str) -> IResult<&str, Packet> {
    alt((integer, list))(s)
}

fn integer(s: &str) -> IResult<&str, Packet> {
    map(u32, |n| Packet::Integer(n))(s)
}

fn list(s: &str) -> IResult<&str, Packet> {
    let p = delimited(start, separated_list0(sep, packet), end);
    let mut p = map(p, |nodes| Packet::List(nodes));
    p(s)
}

fn start(s: &str) -> IResult<&str, PacketToken> {
    value(PacketToken::Start, tag("["))(s)
}

fn end(s: &str) -> IResult<&str, PacketToken> {
    value(PacketToken::End, tag("]"))(s)
}

fn sep(s: &str) -> IResult<&str, PacketToken> {
    value(PacketToken::Sep, tag(","))(s)
}

fn eol(s: &str) -> IResult<&str, PacketToken> {
    value(PacketToken::EOL, tag("\n"))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_packet() {
        use Packet::*;
        assert_eq!(
            packet("[1,1,3,1,1]").unwrap(),
            (
                "",
                List(vec![
                    Integer(1),
                    Integer(1),
                    Integer(3),
                    Integer(1),
                    Integer(1)
                ])
            )
        );

        assert_eq!(
            packet("[[1],[2,3,4]]").unwrap(),
            (
                "",
                List(vec![
                    List(vec![Integer(1)]),
                    List(vec![Integer(2), Integer(3), Integer(4)])
                ])
            )
        );

        assert_eq!(
            packet("[[[]]]").unwrap(),
            ("", List(vec![List(vec![List(vec![])])]))
        );
    }

    #[test]
    fn it_can_compare_packets() {
        let (_, p1) = packet("[1,1,3,1,1]").unwrap();
        let (_, p2) = packet("[1,1,5,1,1]").unwrap();
        assert_eq!(p1.cmp(&p2), Ordering::Less);

        let (_, p1) = packet("[[1],[2,3,4]]").unwrap();
        let (_, p2) = packet("[[1],4]").unwrap();
        assert_eq!(p1.cmp(&p2), Ordering::Less);

        let (_, p1) = packet("[9]").unwrap();
        let (_, p2) = packet("[[8,7,6]]").unwrap();
        assert_eq!(p1.cmp(&p2), Ordering::Greater);

        let (_, p1) = packet("[[4,4],4,4]").unwrap();
        let (_, p2) = packet("[[4,4],4,4,4]").unwrap();
        assert!(p1 < p2);
    }
}
