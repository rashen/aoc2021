use std::{fs, io::BufRead, io::BufReader};

// This solution is left MSB
// const POSITIONS_BIT_MASK: [u16; 12] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
const POSITIONS_BIT_MASK: [u16; 12] = [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];

fn read_input() -> Vec<u16> {
    let filename = "src/day3/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);
    contents
        .lines()
        .filter_map(|r| match r {
            Ok(s) => u16::from_str_radix(&s, 2).ok(),
            Err(_) => None,
        })
        .collect::<Vec<_>>()
}

fn get_sequence_of_most_common_bits(bits: &[u16]) -> u16 {
    let mut n_bits_per_position = Vec::new();
    for pos in POSITIONS_BIT_MASK {
        n_bits_per_position.push(bits.iter().fold(0_u16, |acc, b| match b & pos > 0 {
            true => acc + 1,
            _ => acc,
        }));
    }

    let half_number_of_rows = (bits.len() / 2) as u16;
    n_bits_per_position
        .iter()
        .zip(POSITIONS_BIT_MASK.iter())
        .fold(0_u16, |acc, (n_bits, pos)| {
            match n_bits > &half_number_of_rows {
                true => acc + pos,
                _ => acc,
            }
        })
}

fn get_full_bit_mask(size: u16) -> u16 {
    let mut bit_mask = 0;
    for i in 0..size {
        bit_mask += 2_u16.pow(i.into());
    }
    bit_mask
}

fn get_sequence_of_least_common_bits(bits: &[u16], size: u16) -> u16 {
    let most_common = get_sequence_of_most_common_bits(bits);
    let bit_mask = get_full_bit_mask(size);

    !most_common & bit_mask
}

fn get_ones_at_position(bits: &[u16], position: u16) -> u16 {
    let bit_mask = POSITIONS_BIT_MASK[POSITIONS_BIT_MASK.len() - position as usize - 1];
    bits.iter().fold(0, |acc, b| match b & bit_mask > 0 {
        true => acc + 1,
        false => acc,
    })
}

fn get_oxygen_rating(bits: &[u16], size: u16) -> u16 {
    let mut remaining_sequences = bits;
    for i in size..0 {
        get_ones_at_position(remaining_sequences, i);

        // for j in 0..remaining_sequences {}
    }

    0
}

pub fn run() {
    let bits = read_input();
    assert_eq!(bits.len(), 1000);
    let gamma_rate = get_sequence_of_most_common_bits(&bits);
    let epsilon_rate = get_sequence_of_least_common_bits(&bits, 12);

    let power_consumption: u32 = gamma_rate as u32 * epsilon_rate as u32;

    println!(
        "Task1: Gamme rate {}, Epsilon rate {}, Total {}",
        gamma_rate, epsilon_rate, power_consumption
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<u16> {
        vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ]
    }

    #[test]
    fn test_get_most_common_bit() {
        let input = get_input();
        assert_eq!(get_sequence_of_most_common_bits(&input), 22);
    }

    #[test]
    fn test_get_bit_mask() {
        let input = get_input();
        assert_eq!(get_full_bit_mask(5), 31);
    }

    #[test]
    fn test_get_least_common_bit() {
        let input = get_input();
        assert_eq!(get_sequence_of_least_common_bits(&input, 5), 9);
    }

    #[test]
    fn test_task1() {
        let input = get_input();

        let gamma = get_sequence_of_most_common_bits(&input);
        let epsilon = get_sequence_of_least_common_bits(&input, 5);
        let fuel_consumption: u32 = gamma as u32 * epsilon as u32;
        assert_eq!(fuel_consumption, 198);
    }

    #[test]
    fn test_get_ones_at_position() {
        let input = get_input();
        assert_eq!(get_ones_at_position(&input, 4), 7);
    }

    #[test]
    fn test_get_oxygen_rating() {
        let input = get_input();
        assert_eq!(get_oxygen_rating(&input, 5), 23);
    }
}
