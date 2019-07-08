use crate::architecture::condition_codes::ConditionCodes;

pub fn add(num1: u8, num2:u8) -> (u8, ConditionCodes) {
    let mut conditions = ConditionCodes{..Default::default()};

    let sum = (num1 as u16) + (num2 as u16);

    conditions.z = (sum & 0xff) == 0;
    conditions.s = (sum & 0x80) > 0;
    conditions.cy = sum > 0xff;
    conditions.p = compute_parity(sum as u8);
    conditions.ac = (num1 < 0x80 && num2 < 0x80) && sum >= 0x80;

    (sum as u8, conditions)
}

pub fn sub(num1: u8, num2: u8) -> (u8, ConditionCodes) {
    let mut result = add(num1, twos_complement(num2));
    result.1.cy = !result.1.cy;

    result
}

pub fn compare(num1: u8, num2: u8) -> (u8, ConditionCodes) {
    sub(num1, num2)
}

pub fn ones_complement(num: u8) -> u8 {
    num ^ 0xff
}

pub fn twos_complement(num: u8) -> u8 {
    add(ones_complement(num), 1).0
}

pub fn compute_parity(x: u8) -> bool {
    let mut y = x ^ (x >> 1);
    y = y ^ (y >> 2);
    y = y ^ (y >> 4);
    y % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_zero() {
        let result = add(0x00,0x00);
        assert_eq!(result.0, 0x00);
        assert_eq!(result.1, ConditionCodes{z: true,s: false,p: true,cy: false,ac: false,pad: 0});
    }

    #[test]
    fn test_add_non_zero() {
        let result = add(0x05,0x03);
        assert_eq!(result.0, 0x08);
        assert_eq!(result.1, ConditionCodes{z: false,s: false,p: false,cy: false,ac: false,pad: 0});
    }

    #[test]
    fn test_add_carry_max() {
        let result = add(0x7f,0x01);
        assert_eq!(result.0, 0x80);
        assert_eq!(result.1, ConditionCodes{z: false,s: true,p: false,cy: false,ac: true,pad: 0});
    }

    #[test]
    fn test_add_carry_min() {
        let result = add(0x80,0xff);
        assert_eq!(result.0, 0x7f);
        assert_eq!(result.1, ConditionCodes{z: false,s: false,p: false,cy: true,ac: false,pad: 0});
    }

    #[test]
    fn test_negatives() {
        let result = add(0xf6,0xff);
        assert_eq!(result.0, 0xf5);
        assert_eq!(result.1, ConditionCodes{z: false,s: true,p: true,cy: true,ac: false,pad: 0});
    }

    #[test]
    fn test_with_example_in_manual() {
        let result = add(0x2e,0x6c);
        assert_eq!(result.0, 0x9a);
        assert_eq!(result.1, ConditionCodes{z: false,s: true,p: true,cy: false,ac: true,pad: 0});
    }

    #[test]
    fn test_parity_odd() {
        let result = compute_parity(0x1);
        assert_eq!(result, false);
    }

    #[test]
    fn test_parity_even() {
        let result = compute_parity(0x3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_ones_complement() {
        let result = ones_complement(0x3e);
        assert_eq!(result, 0xc1);
    }

    #[test]
    fn test_twos_complement() {
        let result = twos_complement(0x3e);
        assert_eq!(result, 0xc2);
    }

    #[test]
    fn test_sub_both_positive() {
        let result = sub(0x03, 0x01);
        assert_eq!(result.0, 0x02);
        assert_eq!(result.1, ConditionCodes{z: false,s: false,p: false,cy: false,ac: false,pad: 0});
    }

    #[test]
    fn test_sub_neg() {
        let result = sub(0x03, 0xf6);
        assert_eq!(result.0, 0x0d);
        assert_eq!(result.1, ConditionCodes{z: false,s: false,p: false,cy: true,ac: false,pad: 0});
    }

    #[test]
    fn test_sub_from_neg() {
        let result = sub(0xf6, 0x01);
        assert_eq!(result.0, 0xf5);
        assert_eq!(result.1, ConditionCodes{z: false,s: true,p: true,cy: false,ac: false,pad: 0});
    }

    #[test]
    fn test_sub_with_example_in_manual() {
        let result = sub(0x3e, 0x3e);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, ConditionCodes{z: true,s: false,p: true,cy: false,ac: false,pad: 0});
    }

    #[test]
    fn test_compare_with_example_in_manual() {
        let result = compare(0x4a, 0x40);
        assert_eq!(result.0, 0x0a);
        assert_eq!(result.1, ConditionCodes{z: false,s: false,p: true,cy: false,ac: false,pad: 0});
    }
}