#[doc = "Register `SINGLECTRL` reader"]
pub struct R(crate::R<SINGLECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLECTRL` writer"]
pub struct W(crate::W<SINGLECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SINGLECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLESEL` reader - Single Channel Input Select"]
pub type SINGLESEL_R = crate::FieldReader<u8, SINGLESEL_A>;
#[doc = "Single Channel Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SINGLESEL_A {
    #[doc = "32: `100000`"]
    APORT1XCH0 = 32,
    #[doc = "33: `100001`"]
    APORT1YCH1 = 33,
    #[doc = "34: `100010`"]
    APORT1XCH2 = 34,
    #[doc = "35: `100011`"]
    APORT1YCH3 = 35,
    #[doc = "36: `100100`"]
    APORT1XCH4 = 36,
    #[doc = "37: `100101`"]
    APORT1YCH5 = 37,
    #[doc = "38: `100110`"]
    APORT1XCH6 = 38,
    #[doc = "39: `100111`"]
    APORT1YCH7 = 39,
    #[doc = "40: `101000`"]
    APORT1XCH8 = 40,
    #[doc = "41: `101001`"]
    APORT1YCH9 = 41,
    #[doc = "42: `101010`"]
    APORT1XCH10 = 42,
    #[doc = "43: `101011`"]
    APORT1YCH11 = 43,
    #[doc = "44: `101100`"]
    APORT1XCH12 = 44,
    #[doc = "45: `101101`"]
    APORT1YCH13 = 45,
    #[doc = "46: `101110`"]
    APORT1XCH14 = 46,
    #[doc = "47: `101111`"]
    APORT1YCH15 = 47,
    #[doc = "48: `110000`"]
    APORT1XCH16 = 48,
    #[doc = "49: `110001`"]
    APORT1YCH17 = 49,
    #[doc = "50: `110010`"]
    APORT1XCH18 = 50,
    #[doc = "51: `110011`"]
    APORT1YCH19 = 51,
    #[doc = "52: `110100`"]
    APORT1XCH20 = 52,
    #[doc = "53: `110101`"]
    APORT1YCH21 = 53,
    #[doc = "54: `110110`"]
    APORT1XCH22 = 54,
    #[doc = "55: `110111`"]
    APORT1YCH23 = 55,
    #[doc = "56: `111000`"]
    APORT1XCH24 = 56,
    #[doc = "57: `111001`"]
    APORT1YCH25 = 57,
    #[doc = "58: `111010`"]
    APORT1XCH26 = 58,
    #[doc = "59: `111011`"]
    APORT1YCH27 = 59,
    #[doc = "60: `111100`"]
    APORT1XCH28 = 60,
    #[doc = "61: `111101`"]
    APORT1YCH29 = 61,
    #[doc = "62: `111110`"]
    APORT1XCH30 = 62,
    #[doc = "63: `111111`"]
    APORT1YCH31 = 63,
    #[doc = "96: `1100000`"]
    APORT3XCH0 = 96,
    #[doc = "97: `1100001`"]
    APORT3YCH1 = 97,
    #[doc = "98: `1100010`"]
    APORT3XCH2 = 98,
    #[doc = "99: `1100011`"]
    APORT3YCH3 = 99,
    #[doc = "100: `1100100`"]
    APORT3XCH4 = 100,
    #[doc = "101: `1100101`"]
    APORT3YCH5 = 101,
    #[doc = "102: `1100110`"]
    APORT3XCH6 = 102,
    #[doc = "103: `1100111`"]
    APORT3YCH7 = 103,
    #[doc = "104: `1101000`"]
    APORT3XCH8 = 104,
    #[doc = "105: `1101001`"]
    APORT3YCH9 = 105,
    #[doc = "106: `1101010`"]
    APORT3XCH10 = 106,
    #[doc = "107: `1101011`"]
    APORT3YCH11 = 107,
    #[doc = "108: `1101100`"]
    APORT3XCH12 = 108,
    #[doc = "109: `1101101`"]
    APORT3YCH13 = 109,
    #[doc = "110: `1101110`"]
    APORT3XCH14 = 110,
    #[doc = "111: `1101111`"]
    APORT3YCH15 = 111,
    #[doc = "112: `1110000`"]
    APORT3XCH16 = 112,
    #[doc = "113: `1110001`"]
    APORT3YCH17 = 113,
    #[doc = "114: `1110010`"]
    APORT3XCH18 = 114,
    #[doc = "115: `1110011`"]
    APORT3YCH19 = 115,
    #[doc = "116: `1110100`"]
    APORT3XCH20 = 116,
    #[doc = "117: `1110101`"]
    APORT3YCH21 = 117,
    #[doc = "118: `1110110`"]
    APORT3XCH22 = 118,
    #[doc = "119: `1110111`"]
    APORT3YCH23 = 119,
    #[doc = "120: `1111000`"]
    APORT3XCH24 = 120,
    #[doc = "121: `1111001`"]
    APORT3YCH25 = 121,
    #[doc = "122: `1111010`"]
    APORT3XCH26 = 122,
    #[doc = "123: `1111011`"]
    APORT3YCH27 = 123,
    #[doc = "124: `1111100`"]
    APORT3XCH28 = 124,
    #[doc = "125: `1111101`"]
    APORT3YCH29 = 125,
    #[doc = "126: `1111110`"]
    APORT3XCH30 = 126,
    #[doc = "127: `1111111`"]
    APORT3YCH31 = 127,
}
impl From<SINGLESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SINGLESEL_A) -> Self {
        variant as _
    }
}
impl SINGLESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SINGLESEL_A> {
        match self.bits {
            32 => Some(SINGLESEL_A::APORT1XCH0),
            33 => Some(SINGLESEL_A::APORT1YCH1),
            34 => Some(SINGLESEL_A::APORT1XCH2),
            35 => Some(SINGLESEL_A::APORT1YCH3),
            36 => Some(SINGLESEL_A::APORT1XCH4),
            37 => Some(SINGLESEL_A::APORT1YCH5),
            38 => Some(SINGLESEL_A::APORT1XCH6),
            39 => Some(SINGLESEL_A::APORT1YCH7),
            40 => Some(SINGLESEL_A::APORT1XCH8),
            41 => Some(SINGLESEL_A::APORT1YCH9),
            42 => Some(SINGLESEL_A::APORT1XCH10),
            43 => Some(SINGLESEL_A::APORT1YCH11),
            44 => Some(SINGLESEL_A::APORT1XCH12),
            45 => Some(SINGLESEL_A::APORT1YCH13),
            46 => Some(SINGLESEL_A::APORT1XCH14),
            47 => Some(SINGLESEL_A::APORT1YCH15),
            48 => Some(SINGLESEL_A::APORT1XCH16),
            49 => Some(SINGLESEL_A::APORT1YCH17),
            50 => Some(SINGLESEL_A::APORT1XCH18),
            51 => Some(SINGLESEL_A::APORT1YCH19),
            52 => Some(SINGLESEL_A::APORT1XCH20),
            53 => Some(SINGLESEL_A::APORT1YCH21),
            54 => Some(SINGLESEL_A::APORT1XCH22),
            55 => Some(SINGLESEL_A::APORT1YCH23),
            56 => Some(SINGLESEL_A::APORT1XCH24),
            57 => Some(SINGLESEL_A::APORT1YCH25),
            58 => Some(SINGLESEL_A::APORT1XCH26),
            59 => Some(SINGLESEL_A::APORT1YCH27),
            60 => Some(SINGLESEL_A::APORT1XCH28),
            61 => Some(SINGLESEL_A::APORT1YCH29),
            62 => Some(SINGLESEL_A::APORT1XCH30),
            63 => Some(SINGLESEL_A::APORT1YCH31),
            96 => Some(SINGLESEL_A::APORT3XCH0),
            97 => Some(SINGLESEL_A::APORT3YCH1),
            98 => Some(SINGLESEL_A::APORT3XCH2),
            99 => Some(SINGLESEL_A::APORT3YCH3),
            100 => Some(SINGLESEL_A::APORT3XCH4),
            101 => Some(SINGLESEL_A::APORT3YCH5),
            102 => Some(SINGLESEL_A::APORT3XCH6),
            103 => Some(SINGLESEL_A::APORT3YCH7),
            104 => Some(SINGLESEL_A::APORT3XCH8),
            105 => Some(SINGLESEL_A::APORT3YCH9),
            106 => Some(SINGLESEL_A::APORT3XCH10),
            107 => Some(SINGLESEL_A::APORT3YCH11),
            108 => Some(SINGLESEL_A::APORT3XCH12),
            109 => Some(SINGLESEL_A::APORT3YCH13),
            110 => Some(SINGLESEL_A::APORT3XCH14),
            111 => Some(SINGLESEL_A::APORT3YCH15),
            112 => Some(SINGLESEL_A::APORT3XCH16),
            113 => Some(SINGLESEL_A::APORT3YCH17),
            114 => Some(SINGLESEL_A::APORT3XCH18),
            115 => Some(SINGLESEL_A::APORT3YCH19),
            116 => Some(SINGLESEL_A::APORT3XCH20),
            117 => Some(SINGLESEL_A::APORT3YCH21),
            118 => Some(SINGLESEL_A::APORT3XCH22),
            119 => Some(SINGLESEL_A::APORT3YCH23),
            120 => Some(SINGLESEL_A::APORT3XCH24),
            121 => Some(SINGLESEL_A::APORT3YCH25),
            122 => Some(SINGLESEL_A::APORT3XCH26),
            123 => Some(SINGLESEL_A::APORT3YCH27),
            124 => Some(SINGLESEL_A::APORT3XCH28),
            125 => Some(SINGLESEL_A::APORT3YCH29),
            126 => Some(SINGLESEL_A::APORT3XCH30),
            127 => Some(SINGLESEL_A::APORT3YCH31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == SINGLESEL_A::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == SINGLESEL_A::APORT1YCH31
    }
    #[doc = "Checks if the value of the field is `APORT3XCH0`"]
    #[inline(always)]
    pub fn is_aport3xch0(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH0
    }
    #[doc = "Checks if the value of the field is `APORT3YCH1`"]
    #[inline(always)]
    pub fn is_aport3ych1(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH1
    }
    #[doc = "Checks if the value of the field is `APORT3XCH2`"]
    #[inline(always)]
    pub fn is_aport3xch2(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH2
    }
    #[doc = "Checks if the value of the field is `APORT3YCH3`"]
    #[inline(always)]
    pub fn is_aport3ych3(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH3
    }
    #[doc = "Checks if the value of the field is `APORT3XCH4`"]
    #[inline(always)]
    pub fn is_aport3xch4(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH4
    }
    #[doc = "Checks if the value of the field is `APORT3YCH5`"]
    #[inline(always)]
    pub fn is_aport3ych5(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH5
    }
    #[doc = "Checks if the value of the field is `APORT3XCH6`"]
    #[inline(always)]
    pub fn is_aport3xch6(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH6
    }
    #[doc = "Checks if the value of the field is `APORT3YCH7`"]
    #[inline(always)]
    pub fn is_aport3ych7(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH7
    }
    #[doc = "Checks if the value of the field is `APORT3XCH8`"]
    #[inline(always)]
    pub fn is_aport3xch8(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH8
    }
    #[doc = "Checks if the value of the field is `APORT3YCH9`"]
    #[inline(always)]
    pub fn is_aport3ych9(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH9
    }
    #[doc = "Checks if the value of the field is `APORT3XCH10`"]
    #[inline(always)]
    pub fn is_aport3xch10(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH10
    }
    #[doc = "Checks if the value of the field is `APORT3YCH11`"]
    #[inline(always)]
    pub fn is_aport3ych11(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH11
    }
    #[doc = "Checks if the value of the field is `APORT3XCH12`"]
    #[inline(always)]
    pub fn is_aport3xch12(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH12
    }
    #[doc = "Checks if the value of the field is `APORT3YCH13`"]
    #[inline(always)]
    pub fn is_aport3ych13(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH13
    }
    #[doc = "Checks if the value of the field is `APORT3XCH14`"]
    #[inline(always)]
    pub fn is_aport3xch14(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH14
    }
    #[doc = "Checks if the value of the field is `APORT3YCH15`"]
    #[inline(always)]
    pub fn is_aport3ych15(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH15
    }
    #[doc = "Checks if the value of the field is `APORT3XCH16`"]
    #[inline(always)]
    pub fn is_aport3xch16(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH16
    }
    #[doc = "Checks if the value of the field is `APORT3YCH17`"]
    #[inline(always)]
    pub fn is_aport3ych17(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH17
    }
    #[doc = "Checks if the value of the field is `APORT3XCH18`"]
    #[inline(always)]
    pub fn is_aport3xch18(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH18
    }
    #[doc = "Checks if the value of the field is `APORT3YCH19`"]
    #[inline(always)]
    pub fn is_aport3ych19(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH19
    }
    #[doc = "Checks if the value of the field is `APORT3XCH20`"]
    #[inline(always)]
    pub fn is_aport3xch20(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH20
    }
    #[doc = "Checks if the value of the field is `APORT3YCH21`"]
    #[inline(always)]
    pub fn is_aport3ych21(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH21
    }
    #[doc = "Checks if the value of the field is `APORT3XCH22`"]
    #[inline(always)]
    pub fn is_aport3xch22(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH22
    }
    #[doc = "Checks if the value of the field is `APORT3YCH23`"]
    #[inline(always)]
    pub fn is_aport3ych23(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH23
    }
    #[doc = "Checks if the value of the field is `APORT3XCH24`"]
    #[inline(always)]
    pub fn is_aport3xch24(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH24
    }
    #[doc = "Checks if the value of the field is `APORT3YCH25`"]
    #[inline(always)]
    pub fn is_aport3ych25(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH25
    }
    #[doc = "Checks if the value of the field is `APORT3XCH26`"]
    #[inline(always)]
    pub fn is_aport3xch26(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH26
    }
    #[doc = "Checks if the value of the field is `APORT3YCH27`"]
    #[inline(always)]
    pub fn is_aport3ych27(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH27
    }
    #[doc = "Checks if the value of the field is `APORT3XCH28`"]
    #[inline(always)]
    pub fn is_aport3xch28(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH28
    }
    #[doc = "Checks if the value of the field is `APORT3YCH29`"]
    #[inline(always)]
    pub fn is_aport3ych29(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH29
    }
    #[doc = "Checks if the value of the field is `APORT3XCH30`"]
    #[inline(always)]
    pub fn is_aport3xch30(&self) -> bool {
        *self == SINGLESEL_A::APORT3XCH30
    }
    #[doc = "Checks if the value of the field is `APORT3YCH31`"]
    #[inline(always)]
    pub fn is_aport3ych31(&self) -> bool {
        *self == SINGLESEL_A::APORT3YCH31
    }
}
#[doc = "Field `SINGLESEL` writer - Single Channel Input Select"]
pub type SINGLESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, SINGLESEL_A, 7, O>;
impl<'a, const O: u8> SINGLESEL_W<'a, O> {
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH0)
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH1)
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH2)
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH3)
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH4)
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH5)
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH6)
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH7)
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH8)
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH9)
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH10)
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH11)
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH12)
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH13)
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH14)
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH15)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH16)
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH17)
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH18)
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH19)
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH20)
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH21)
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH22)
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH23)
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH24)
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH25)
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH26)
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH27)
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH28)
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH29)
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1XCH30)
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT1YCH31)
    }
    #[doc = "`1100000`"]
    #[inline(always)]
    pub fn aport3xch0(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH0)
    }
    #[doc = "`1100001`"]
    #[inline(always)]
    pub fn aport3ych1(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH1)
    }
    #[doc = "`1100010`"]
    #[inline(always)]
    pub fn aport3xch2(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH2)
    }
    #[doc = "`1100011`"]
    #[inline(always)]
    pub fn aport3ych3(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH3)
    }
    #[doc = "`1100100`"]
    #[inline(always)]
    pub fn aport3xch4(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH4)
    }
    #[doc = "`1100101`"]
    #[inline(always)]
    pub fn aport3ych5(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH5)
    }
    #[doc = "`1100110`"]
    #[inline(always)]
    pub fn aport3xch6(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH6)
    }
    #[doc = "`1100111`"]
    #[inline(always)]
    pub fn aport3ych7(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH7)
    }
    #[doc = "`1101000`"]
    #[inline(always)]
    pub fn aport3xch8(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH8)
    }
    #[doc = "`1101001`"]
    #[inline(always)]
    pub fn aport3ych9(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH9)
    }
    #[doc = "`1101010`"]
    #[inline(always)]
    pub fn aport3xch10(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH10)
    }
    #[doc = "`1101011`"]
    #[inline(always)]
    pub fn aport3ych11(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH11)
    }
    #[doc = "`1101100`"]
    #[inline(always)]
    pub fn aport3xch12(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH12)
    }
    #[doc = "`1101101`"]
    #[inline(always)]
    pub fn aport3ych13(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH13)
    }
    #[doc = "`1101110`"]
    #[inline(always)]
    pub fn aport3xch14(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH14)
    }
    #[doc = "`1101111`"]
    #[inline(always)]
    pub fn aport3ych15(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH15)
    }
    #[doc = "`1110000`"]
    #[inline(always)]
    pub fn aport3xch16(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH16)
    }
    #[doc = "`1110001`"]
    #[inline(always)]
    pub fn aport3ych17(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH17)
    }
    #[doc = "`1110010`"]
    #[inline(always)]
    pub fn aport3xch18(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH18)
    }
    #[doc = "`1110011`"]
    #[inline(always)]
    pub fn aport3ych19(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH19)
    }
    #[doc = "`1110100`"]
    #[inline(always)]
    pub fn aport3xch20(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH20)
    }
    #[doc = "`1110101`"]
    #[inline(always)]
    pub fn aport3ych21(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH21)
    }
    #[doc = "`1110110`"]
    #[inline(always)]
    pub fn aport3xch22(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH22)
    }
    #[doc = "`1110111`"]
    #[inline(always)]
    pub fn aport3ych23(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH23)
    }
    #[doc = "`1111000`"]
    #[inline(always)]
    pub fn aport3xch24(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH24)
    }
    #[doc = "`1111001`"]
    #[inline(always)]
    pub fn aport3ych25(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH25)
    }
    #[doc = "`1111010`"]
    #[inline(always)]
    pub fn aport3xch26(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH26)
    }
    #[doc = "`1111011`"]
    #[inline(always)]
    pub fn aport3ych27(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH27)
    }
    #[doc = "`1111100`"]
    #[inline(always)]
    pub fn aport3xch28(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH28)
    }
    #[doc = "`1111101`"]
    #[inline(always)]
    pub fn aport3ych29(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH29)
    }
    #[doc = "`1111110`"]
    #[inline(always)]
    pub fn aport3xch30(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3XCH30)
    }
    #[doc = "`1111111`"]
    #[inline(always)]
    pub fn aport3ych31(self) -> &'a mut W {
        self.variant(SINGLESEL_A::APORT3YCH31)
    }
}
impl R {
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline(always)]
    pub fn singlesel(&self) -> SINGLESEL_R {
        SINGLESEL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn singlesel(&mut self) -> SINGLESEL_W<4> {
        SINGLESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Conversion Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrl](index.html) module"]
pub struct SINGLECTRL_SPEC;
impl crate::RegisterSpec for SINGLECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlectrl::R](R) reader structure"]
impl crate::Readable for SINGLECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](W) writer structure"]
impl crate::Writable for SINGLECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0"]
impl crate::Resettable for SINGLECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
