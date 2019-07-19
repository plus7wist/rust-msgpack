/*
https://github.com/msgpack/msgpack
*/

pub type Code = u8;

/*
format name: nil
first byte (in binary): 11000000
first byte (in hex): 0xc0
*/
pub const NIL: Code = 0xc0;

/*
format name: false
first byte (in binary): 11000010
first byte (in hex): 0xc2

format name: true
first byte (in binary): 11000011
first byte (in hex): 0xc3
*/
pub const FALSE: Code = 0xc2;
pub const TRUE: Code = 0xc3;

/*
format name: bin 8
first byte (in binary): 11000100
first byte (in hex): 0xc4

format name: bin 16
first byte (in binary): 11000101
first byte (in hex): 0xc5

format name: bin 32
first byte (in binary): 11000110
first byte (in hex): 0xc6
*/
pub const BIN_8: Code = 0xc4;
pub const BIN_16: Code = 0xc5;
pub const BIN_32: Code = 0xc6;

/*
format name: ext 8
first byte (in binary): 11000111
first byte (in hex): 0xc7

format name: ext 16
first byte (in binary): 11001000
first byte (in hex): 0xc8

format name: ext 32
first byte (in binary): 11001001
first byte (in hex): 0xc9
*/
pub const EXT_8: Code = 0xc7;
pub const EXT_16: Code = 0xc8;
pub const EXT_32: Code = 0xc9;

/*
format name: float 32
first byte (in binary): 11001010
first byte (in hex): 0xca

format name: float 64
first byte (in binary): 11001011
first byte (in hex): 0xcb
*/
pub const FLOAT_32: Code = 0xca;
pub const FLOAT_64: Code = 0xcb;

/*
format name: uint 8
first byte (in binary): 11001100
first byte (in hex): 0xcc

format name: uint 16
first byte (in binary): 11001101
first byte (in hex): 0xcd

format name: uint 32
first byte (in binary): 11001110
first byte (in hex): 0xce

format name: uint 64
first byte (in binary): 11001111
first byte (in hex): 0xcf
*/
pub const UINT_8: Code = 0xcc;
pub const UINT_16: Code = 0xcd;
pub const UINT_32: Code = 0xce;
pub const UINT_64: Code = 0xcf;

/*
format name: int 8
first byte (in binary): 11010000
first byte (in hex): 0xd0

format name: int 16
first byte (in binary): 11010001
first byte (in hex): 0xd1

format name: int 32
first byte (in binary): 11010010
first byte (in hex): 0xd2

format name: int 64
first byte (in binary): 11010011
first byte (in hex): 0xd3
*/
pub const INT_8: Code = 0xd0;
pub const INT_16: Code = 0xd1;
pub const INT_32: Code = 0xd2;
pub const INT_64: Code = 0xd3;

/*
format name: fixext 1
first byte (in binary): 11010100
first byte (in hex): 0xd4

format name: fixext 2
first byte (in binary): 11010101
first byte (in hex): 0xd5

format name: fixext 4
first byte (in binary): 11010110
first byte (in hex): 0xd6

format name: fixext 8
first byte (in binary): 11010111
first byte (in hex): 0xd7

format name: fixext 16
first byte (in binary): 11011000
first byte (in hex): 0xd8
*/
pub const FIX_EXT_1: Code = 0xd4;
pub const FIX_EXT_2: Code = 0xd5;
pub const FIX_EXT_4: Code = 0xd6;
pub const FIX_EXT_8: Code = 0xd7;
pub const FIX_EXT_16: Code = 0xd8;

/*
format name: str 8
first byte (in binary): 11011001
first byte (in hex): 0xd9

format name: str 16
first byte (in binary): 11011010
first byte (in hex): 0xda

format name: str 32
first byte (in binary): 11011011
first byte (in hex): 0xdb
*/
pub const STR_8: Code = 0xd9;
pub const STR_16: Code = 0xda;
pub const STR_32: Code = 0xdb;

/*
format name: array 16
first byte (in binary): 11011100
first byte (in hex): 0xdc

format name: array 32
first byte (in binary): 11011101
first byte (in hex): 0xdd
*/
pub const ARRAY_16: Code = 0xdc;
pub const ARRAY_32: Code = 0xdd;

/*
format name: map 16
first byte (in binary): 11011110
first byte (in hex): 0xde

format name: map 32
first byte (in binary): 11011111
first byte (in hex): 0xdf
*/
pub const MAP_16: Code = 0xde;
pub const MAP_32: Code = 0xdf;

/*
format name: positive fixint
first byte (in binary): 0xxxxxxx
first byte (in hex): 0x00 - 0x7f
*/
pub const POS_FIXED_NUM_HIGH: Code = 0x7f;

/*
format name: fixmap
first byte (in binary): 1000xxxx
first byte (in hex): 0x80 - 0x8f
*/
pub const FIXED_MAP_LOW: Code = 0x80;
pub const FIXED_MAP_HIGH: Code = 0x8f;
pub const FIXED_MAP_MASK: Code = 0xf;

/*
format name: fixarray
first byte (in binary): 1001xxxx
first byte (in hex): 0x90 - 0x9f
*/
pub const FIXED_ARRAY_LOW: Code = 0x90;
pub const FIXED_ARRAY_HIGH: Code = 0x9f;
pub const FIXED_ARRAY_MASK: Code = 0xf;

/*
format name: fixstr
first byte (in binary): 101xxxxx
first byte (in hex): 0xa0 - 0xbf
*/
pub const FIXED_STR_LOW: Code = 0xa0;
pub const FIXED_STR_HIGH: Code = 0xbf;
pub const FIXED_STR_MASK: Code = 0x1f;

/*
format name: negative fixint
first byte (in binary): 111xxxxx
first byte (in hex): 0xe0 - 0xff
*/
pub const NEG_FIXED_NUM_LOW: Code = 0xe0;

pub fn is_fixed_num(c: Code) -> bool {
    c <= POS_FIXED_NUM_HIGH || c >= NEG_FIXED_NUM_LOW
}

pub fn is_fixed_map(c: Code) -> bool {
    c >= FIXED_MAP_LOW && c <= FIXED_MAP_HIGH
}

pub fn is_fixed_array(c: Code) -> bool {
    c >= FIXED_ARRAY_LOW && c <= FIXED_ARRAY_HIGH
}

pub fn is_fixed_string(c: Code) -> bool {
    c >= FIXED_STR_LOW && c <= FIXED_STR_HIGH
}

pub fn is_ext(c: Code) -> bool {
    (c >= FIX_EXT_1 && c <= FIX_EXT_16) || (c >= EXT_8 && c <= EXT_32)
}

pub fn is_nil(c: Code) -> bool {
    c == NIL
}

pub fn is_bool(c: Code) -> bool {
    c == FALSE || c == TRUE
}

pub fn is_number(c: Code) -> bool {
    if is_fixed_num(c) {
        return true;
    }
    c == FLOAT_32
        || c == FLOAT_64
        || c == UINT_8
        || c == UINT_16
        || c == UINT_32
        || c == UINT_64
        || c == INT_8
        || c == INT_16
        || c == INT_32
        || c == INT_64
}

pub fn is_string(c: Code) -> bool {
    if is_fixed_string(c) {
        return true;
    }
    c == STR_8 || c == STR_16 || c == STR_32
}

pub fn is_bin_array(c: Code) -> bool {
    c == BIN_8 || c == BIN_16 || c == BIN_32
}

pub fn is_array(c: Code) -> bool {
    if c >= FIXED_ARRAY_LOW && c <= FIXED_ARRAY_HIGH {
        return true;
    }
    c == ARRAY_16 || c == ARRAY_32
}

pub fn is_hashmap(c: Code) -> bool {
    if c >= FIXED_MAP_LOW && c <= FIXED_MAP_HIGH {
        return true;
    }
    c == MAP_16 || c == MAP_32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg_fixed_num_low_change_type() {
        assert_eq!(NEG_FIXED_NUM_LOW, 0xe0);
        assert_eq!(NEG_FIXED_NUM_LOW as i8, -32);
        assert_eq!((NEG_FIXED_NUM_LOW as i8) as i64, -32);
    }
}
