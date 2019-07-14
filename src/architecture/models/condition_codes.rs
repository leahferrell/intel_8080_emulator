
#[derive(Debug, Eq, PartialEq, Default)]
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool
}

impl ConditionCodes {
    pub fn as_binary(&self) -> u8 {
        let _7 = if self.s {0x80} else {0x00};
        let _6 = if self.z {0x40} else {0x00};
        let _5 = 0x00;
        let _4 = if self.ac {0x10} else {0x00};
        let _3 = 0x00;
        let _2 = if self.p {0x04} else {0x00};
        let _1 = 0x02;
        let _0 = if self.cy {0x01} else {0x00};

        let value = _7 | _6 | _5 | _4 | _3 | _2 | _1 | _0;

        value
    }

    pub fn update_from_binary(&mut self, bin: u8) {
        self.cy = bin & 1 == 1;
        self.p = (bin >> 2) & 1 == 1;
        self.ac = (bin >> 4) & 1 == 1;
        self.z = (bin >> 6) & 1 == 1;
        self.s = (bin >> 7) & 1 == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_conditions_false() {
        let cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        assert_eq!(cc.as_binary(), 0x02);
    }

    #[test]
    fn test_all_conditions_true() {
        let cc = ConditionCodes{z: true,s: true,p: true,cy: true,ac: true};
        assert_eq!(cc.as_binary(), 0xd7);
    }

    #[test]
    fn test_some_conditions_true() {
        let cc = ConditionCodes{z: false,s: false,p: true,cy: false,ac: true};
        assert_eq!(cc.as_binary(), 0x16);
    }

    #[test]
    fn test_update_all_conditions_false() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        cc.update_from_binary(0x02);
        assert_eq!(cc, ConditionCodes{z: false,s: false,p: false,cy: false,ac: false});
    }

    #[test]
    fn test_update_all_conditions_true() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        cc.update_from_binary(0xd7);
        assert_eq!(cc, ConditionCodes{z: true,s: true,p: true,cy: true,ac: true});
    }

    #[test]
    fn test_update_some_conditions_true() {
        let mut cc = ConditionCodes{z: false,s: false,p: false,cy: false,ac: false};
        cc.update_from_binary(0x16);
        assert_eq!(cc, ConditionCodes{z: false,s: false,p: true,cy: false,ac: true});
    }
}