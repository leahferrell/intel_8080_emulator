pub struct ConditionCodes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8
}

impl Default for ConditionCodes {
    fn default() -> ConditionCodes {
        ConditionCodes {
            z: 1,
            s: 1,
            p: 1,
            cy: 1,
            ac: 1,
            pad: 3
        }
    }
}