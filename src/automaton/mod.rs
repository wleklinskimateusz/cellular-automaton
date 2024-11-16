use std::array;

pub struct Automaton {
    pub fields: u128,
    pub rule: u8,
    pub periodic_boundary: bool,
}

fn apply_rule(pattern: u8, rule: u8) -> u8 {
    (rule >> pattern) & 1
}

fn find_nth_bit(number: u128, n: usize) -> u8 {
    (number >> n) as u8 & 1
}

impl Automaton {
    pub fn new(rule: u8, initial: u128, periodic_boundary: bool) -> Self {
        Automaton {
            fields: initial,
            rule,
            periodic_boundary,
        }
    }

    pub fn update(&mut self) {
        let mut new_fields: u128 = 0b0;
        for i in 0..128 {
            let pattern = self.detect_pattern(i);
            let new_bit = apply_rule(pattern, self.rule);
            new_fields |= (new_bit as u128) << i;
        }
        self.fields = new_fields;
    }

    fn detect_pattern(&self, center_index: u8) -> u8 {
        let left = if center_index == 127 {
            if self.periodic_boundary {
                (self.fields >> 0) & 1
            } else {
                0
            }
        } else {
            (self.fields >> (center_index + 1)) & 1
        };

        let center = (self.fields >> center_index) & 1;

        let right = if center_index == 0 {
            if self.periodic_boundary {
                (self.fields >> 127) & 1
            } else {
                0
            }
        } else {
            (self.fields >> (center_index - 1)) & 1
        };

        ((left << 2) | (center << 1) | right) as u8
    }

    pub fn to_list(&self) -> [u8; 128] {
        array::from_fn(|i| find_nth_bit(self.fields, 127 - i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs::OpenOptions, io::Write, time::Instant};

    #[test]
    fn bench_update() {
        let mut automaton = Automaton::new(30, 0b101, true);
        let start = Instant::now();
        {
            for _ in 0..10000 {
                automaton.update();
            }
        }
        let duration = start.elapsed();
        // append to file if it exists or create it
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("bench_update.txt")
            .unwrap();
        writeln!(file, "{:?}", duration).unwrap();
    }

    #[test]
    fn correctly_detect_pattern() {
        let automaton = Automaton::new(30, 0b101, false);
        assert_eq!(automaton.detect_pattern(0), 0b010);
        assert_eq!(automaton.detect_pattern(1), 0b101);
        assert_eq!(automaton.detect_pattern(2), 0b010);
        assert_eq!(automaton.detect_pattern(3), 0b001);
        assert_eq!(automaton.detect_pattern(4), 0b000);
        assert_eq!(automaton.detect_pattern(127), 0b000);
    }

    #[test]
    fn detect_more_patterns() {
        let automaton = Automaton::new(30, 0b1101, false);
        assert_eq!(automaton.detect_pattern(2), 0b110);
        assert_eq!(automaton.detect_pattern(3), 0b011);
        assert_eq!(automaton.detect_pattern(4), 0b001);
        assert_eq!(automaton.detect_pattern(5), 0b000);
    }

    #[test]
    fn detect_periodic_boundary() {
        let automaton = Automaton::new(30, 0b1, true);
        assert_eq!(automaton.detect_pattern(0), 0b010);
        assert_eq!(automaton.detect_pattern(1), 0b001);
        assert_eq!(automaton.detect_pattern(127), 0b100);

        let automaton = Automaton::new(30, u128::MAX, true);
        assert_eq!(automaton.detect_pattern(0), 0b111);
        assert_eq!(automaton.detect_pattern(1), 0b111);
        assert_eq!(automaton.detect_pattern(127), 0b111);
    }

    #[test]
    fn test_apply_rule0() {
        assert_eq!(apply_rule(0b000, 0), 0);
        assert_eq!(apply_rule(0b111, 0), 0);
        assert_eq!(apply_rule(0b101, 0), 0);
    }

    #[test]
    fn test_apply_rule1() {
        assert_eq!(apply_rule(0b000, 1), 1);
        assert_eq!(apply_rule(0b001, 1), 0);
        assert_eq!(apply_rule(0b111, 1), 0);
    }

    #[test]
    fn test_new() {
        let automaton = Automaton::new(30, 0b101, false);
        assert_eq!(automaton.fields, 5);
        assert_eq!(automaton.rule, 30);
    }

    #[test]
    fn test_empty_automaton_rule_0() {
        let mut automaton = Automaton::new(0, 0b0, false);
        assert_eq!(automaton.fields, 0);
        automaton.update();

        assert_eq!(automaton.fields, 0)
    }

    #[test]
    fn test_empty_automaton_rule_1() {
        let mut automaton = Automaton::new(1, 0b0, false);
        automaton.update();

        assert_eq!(automaton.fields, u128::MAX);

        automaton.update();

        assert_eq!(automaton.fields, 0)
    }

    #[test]
    fn test_rule_30() {
        let mut automaton = Automaton::new(30, 0b101, false);
        automaton.update();

        assert_eq!(automaton.fields, 0b1101);
        automaton.update();

        assert_eq!(automaton.fields, 0b11001);
        automaton.update();

        assert_eq!(automaton.fields, 0b110111);
    }

    #[test]
    fn test_rule_100() {
        let mut automaton = Automaton::new(100, 0b100, false);
        automaton.update();

        assert_eq!(automaton.fields, 0b100);

        let mut automaton = Automaton::new(100, 0b101, false);
        automaton.update();

        assert_eq!(automaton.fields, 0b111);

        automaton.update();
        assert_eq!(automaton.fields, 0b001);

        automaton.update();
        assert_eq!(automaton.fields, 0b001);
    }

    #[test]
    fn test_to_vector() {
        let automaton = Automaton::new(30, 0b101, false);
        let vector = automaton.to_list();
        let mut expected = [0; 128];
        expected[125] = 1;
        expected[126] = 0;
        expected[127] = 1;

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_to_vector_2() {
        let automaton = Automaton::new(30, 0b1101, false);
        let vector = automaton.to_list();
        let mut expected = [0; 128];
        expected[124] = 1;
        expected[125] = 1;
        expected[126] = 0;
        expected[127] = 1;

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_max_vector() {
        let automaton = Automaton::new(30, u128::MAX, false);
        let vector = automaton.to_list();
        let expected = [1; 128];

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_1_with_0s() {
        let automaton = Automaton::new(30, 0x80000000000000000000000000000000, false);
        let vector = automaton.to_list();
        let mut expected = [0; 128];
        expected[0] = 1;

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_periodic_boundary() {
        let mut automaton = Automaton::new(0b10000, 0b1, true);
        automaton.update();

        assert_eq!(automaton.fields, 0b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000);
    }
}
