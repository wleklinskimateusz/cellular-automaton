pub struct Automaton {
    pub fields: u128,
    pub rule: u8,
}

fn detect_pattern(fields: u128, center_index: u8) -> u8 {
    let left = if center_index == 127 {
        0
    } else {
        (fields >> (center_index + 1)) & 1
    };

    let center = (fields >> center_index) & 1;

    let right = if center_index == 0 {
        0
    } else {
        (fields >> (center_index - 1)) & 1
    };

    ((left << 2) | (center << 1) | right) as u8
}

fn apply_rule(pattern: u8, rule: u8) -> u8 {
    (rule >> pattern) & 1
}

fn find_nth_bit(number: u128, n: usize) -> u8 {
    (number >> n) as u8 & 1
}

impl Automaton {
    pub fn new(rule: u8, initial: u128) -> Self {
        Automaton {
            fields: initial,
            rule,
        }
    }

    pub fn update(&mut self) {
        let mut new_fields: u128 = 0b0;
        for i in 0..128 {
            let pattern = detect_pattern(self.fields, i);
            let new_bit = apply_rule(pattern, self.rule);
            new_fields |= (new_bit as u128) << i;
        }
        self.fields = new_fields;
    }

    pub fn to_vector(&self) -> Vec<u8> {
        let mut vector = vec![0; 128];
        for i in 0..128 {
            vector[127 - i] = find_nth_bit(self.fields, i);
        }
        vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_detect_pattern() {
        assert_eq!(detect_pattern(0b101, 0), 0b010);
        assert_eq!(detect_pattern(0b101, 1), 0b101);
        assert_eq!(detect_pattern(0b101, 2), 0b010);
        assert_eq!(detect_pattern(0b101, 3), 0b001);
        assert_eq!(detect_pattern(0b101, 4), 0b000);
        assert_eq!(detect_pattern(0b101, 127), 0b000);
    }

    #[test]
    fn detect_more_patterns() {
        assert_eq!(detect_pattern(0b1101, 2), 0b110);
        assert_eq!(detect_pattern(0b1101, 3), 0b011);
        assert_eq!(detect_pattern(0b1101, 4), 0b001);
        assert_eq!(detect_pattern(0b1101, 5), 0b000);
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
        let automaton = Automaton::new(30, 0b101);
        assert_eq!(automaton.fields, 5);
        assert_eq!(automaton.rule, 30);
    }

    #[test]
    fn test_empty_automaton_rule_0() {
        let mut automaton = Automaton::new(0, 0b0);
        assert_eq!(automaton.fields, 0);
        automaton.update();

        assert_eq!(automaton.fields, 0)
    }

    #[test]
    fn test_empty_automaton_rule_1() {
        let mut automaton = Automaton::new(1, 0b0);
        automaton.update();

        assert_eq!(automaton.fields, u128::MAX);

        automaton.update();

        assert_eq!(automaton.fields, 0)
    }

    #[test]
    fn test_rule_30() {
        let mut automaton = Automaton::new(30, 0b101);
        automaton.update();

        assert_eq!(automaton.fields, 0b1101);
        automaton.update();

        assert_eq!(automaton.fields, 0b11001);
        automaton.update();

        assert_eq!(automaton.fields, 0b110111);
    }

    #[test]
    fn test_rule_100() {
        let mut automaton = Automaton::new(100, 0b100);
        automaton.update();

        assert_eq!(automaton.fields, 0b100);

        let mut automaton = Automaton::new(100, 0b101);
        automaton.update();

        assert_eq!(automaton.fields, 0b111);

        automaton.update();
        assert_eq!(automaton.fields, 0b001);

        automaton.update();
        assert_eq!(automaton.fields, 0b001);
    }

    #[test]
    fn test_to_vector() {
        let automaton = Automaton::new(30, 0b101);
        let vector = automaton.to_vector();
        let mut expected = vec![0; 128];
        expected[125] = 1;
        expected[126] = 0;
        expected[127] = 1;

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_to_vector_2() {
        let automaton = Automaton::new(30, 0b1101);
        let vector = automaton.to_vector();
        let mut expected = vec![0; 128];
        expected[124] = 1;
        expected[125] = 1;
        expected[126] = 0;
        expected[127] = 1;

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_max_vector() {
        let automaton = Automaton::new(30, u128::MAX);
        let vector = automaton.to_vector();
        let expected = vec![1; 128];

        assert_eq!(vector, expected);
    }

    #[test]
    fn test_1_with_0s() {
        let automaton = Automaton::new(30, 0x80000000000000000000000000000000);
        let vector = automaton.to_vector();
        let mut expected = vec![0; 128];
        expected[0] = 1;

        assert_eq!(vector, expected);
    }
}
