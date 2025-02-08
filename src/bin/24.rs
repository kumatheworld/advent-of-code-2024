use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(24);

fn parse_wires_and_gates(
    input: &str,
) -> (HashMap<&str, Option<bool>>, Vec<(&str, &str, &str, &str)>) {
    let (xys, gates) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::<&str, Option<bool>>::new();
    for xy in xys.lines() {
        let (key, value) = xy.split_once(": ").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        };
        wires.insert(key, Some(value));
    }

    let gates = gates
        .lines()
        .map(|line| {
            let (w0, op, w1, _, w2) = line.split(' ').collect_tuple().unwrap();
            wires.entry(w0).or_insert(None);
            wires.entry(w1).or_insert(None);
            wires.entry(w2).or_insert(None);
            (w0, op, w1, w2)
        })
        .collect_vec();

    (wires, gates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wires, mut gates) = parse_wires_and_gates(input);

    while !gates.is_empty() {
        gates.retain(|&(w0, op, w1, w2)| {
            if let (Some(b0), Some(b1)) = (*wires.get(&w0).unwrap(), *wires.get(&w1).unwrap()) {
                *wires.get_mut(&w2).unwrap() = Some(match op {
                    "AND" => b0 && b1,
                    "XOR" => b0 != b1,
                    "OR" => b0 || b1,
                    _ => unreachable!(),
                });
                false
            } else {
                true
            }
        })
    }

    Some(
        wires
            .iter()
            .filter(|(w, _)| w.as_bytes()[0] == b'z')
            .sorted_unstable()
            .enumerate()
            .fold(
                0,
                |acc, (i, (_, &b))| if b.unwrap() { (1 << i) | acc } else { acc },
            ),
    )
}

pub fn part_two(_: &str) -> Option<String> {
    // Partially solved with regex searches over the input text.

    // It appears that the whole circuit is made of one half adder + 44 full adders (2 XORs, 2 ANDs and 1 OR).
    // That is a standard way to make a 45-bit adder (e.g. https://www.build-electronic-circuits.com/full-adder/).
    // The following pattern matched all the inputs, meaning that carries aren't mixed up:
    //   [xy].. XOR [xy]..

    // z should come from an XOR except for the last z45.
    // Searching for regex " OR ... -> z", the following turns out wrong:
    //   psw OR nng -> z12
    // Searching for regex "AND ... -> z", the following also turn out wrong:
    //   x26 AND y26 -> z26
    //   vtg AND bkh -> z32

    // XOR should either have x and y as input or have z as output
    //   XOR [^xy].. -> [^z]
    // So there's something wrong with the following that don't match the above regex:
    //   vtg XOR bkh -> tbt
    //   dfp XOR mbg -> gsd
    //   nhb XOR cdq -> kth

    // The following output pairs are swapped
    //   nhb XOR cdq -> kth, psw OR nng -> z12
    //   dfp XOR mbg -> gsd, x26 AND y26 -> z26
    //   vtg XOR bkh -> tbt, vtg AND bkh -> z32

    // The above are all "type errors", meaning that there will still be some wrong wires
    // within the same type (e.g. z01 and z02, conceivably)

    // // Now we chase the circuit from the bottom
    // let (_, gates) = parse_wires_and_gates(input);
    // let mut gates: HashMap<(&str, &str, &str), &str> = gates
    //     .into_iter()
    //     .map(|(w0, op, w1, w2)| ((w0.min(w1), op, w0.max(w1)), w2))
    //     .collect();

    // *gates.get_mut(&("cdq", "XOR", "nhb")).unwrap() = "z12";
    // *gates.get_mut(&("nng", "OR", "psw")).unwrap() = "kth";
    // *gates.get_mut(&("dfp", "XOR", "mbg")).unwrap() = "z26";
    // *gates.get_mut(&("x26", "AND", "y26")).unwrap() = "gsd";
    // *gates.get_mut(&("bkh", "XOR", "vtg")).unwrap() = "z32";
    // *gates.get_mut(&("bkh", "AND", "vtg")).unwrap() = "tbt";

    // // The following shows that the first carry bit is bdj:
    // //   x00 AND y00 -> bdj
    // let mut carry = "bdj";

    // // The following erred when n = 36, carry = "htb", and xn_xor_yn = "vpm"
    // for n in 1..45 {
    //     let xn_xor_yn = *gates
    //         .get(&(&format!("x{:02}", n), "XOR", &format!("y{:02}", n)))
    //         .unwrap();
    //     assert_eq!(
    //         gates
    //             .get(&(carry.min(xn_xor_yn), "XOR", carry.max(xn_xor_yn)))
    //             .unwrap(),
    //         &format!("z{:02}", n)
    //     );
    //     let carry_and_xy = *gates
    //         .get(&(carry.min(xn_xor_yn), "AND", carry.max(xn_xor_yn)))
    //         .unwrap();
    //     let xn_and_yn = *gates
    //         .get(&(&format!("x{:02}", n), "AND", &format!("y{:02}", n)))
    //         .unwrap();
    //     carry = *gates
    //         .get(&(
    //             carry_and_xy.min(xn_and_yn),
    //             "OR",
    //             carry_and_xy.max(xn_and_yn),
    //         ))
    //         .unwrap();
    // }

    // The following is found in the input:
    //   y36 XOR x36 -> vpm
    //   thg OR vpm -> wkk
    // So this was also a type error.
    // From the follwoing, vpm should be swapped with either htb or qnf:
    //   htb XOR qnf -> z36
    // We have the following for htb and qnf:
    //   bbb OR cnp -> htb
    //   htb AND qnf -> thg
    //   y36 AND x36 -> qnf
    // Since vpm should be an output of an AND, it's swapped with qnf.

    Some("gsd,kth,qnf,tbt,vpm,z12,z26,z32".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }
}
