use alloy::primitives::U256;
use std::collections::HashMap;

use super::tick_math::MAX_TICK;

const SECOND_LAYER_OFFSET: i16 = 3466; // ceil(-MIN_TICK / 256)

// 0x5555555555555555555555555555555555555555555555555555555555555555
pub const LEVEL_0: U256 = U256::from_limbs([
    6148914691236517205,
    6148914691236517205,
    6148914691236517205,
    6148914691236517205,
]);
// 0x00000000000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
pub const LEVEL_1: U256 = U256::from_limbs([18446744073709551615, 18446744073709551615, 0, 0]);
// 0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF
pub const LEVEL_2: U256 = U256::from_limbs([18446744073709551615, 0, 18446744073709551615, 0]);
// 0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF
pub const LEVEL_3: U256 = U256::from_limbs([4294967295, 4294967295, 4294967295, 4294967295]);
// 0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF
pub const LEVEL_4: U256 = U256::from_limbs([
    281470681808895,
    281470681808895,
    281470681808895,
    281470681808895,
]);
// 0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF
pub const LEVEL_5: U256 = U256::from_limbs([
    71777214294589695,
    71777214294589695,
    71777214294589695,
    71777214294589695,
]);
// 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F
pub const LEVEL_6: U256 = U256::from_limbs([
    1085102592571150095,
    1085102592571150095,
    1085102592571150095,
    1085102592571150095,
]);
// 0x3333333333333333333333333333333333333333333333333333333333333333
pub const LEVEL_7: U256 = U256::from_limbs([
    3689348814741910323,
    3689348814741910323,
    3689348814741910323,
    3689348814741910323,
]);

pub struct TickTree {
    pub tree_root: u32,
    pub leaves: HashMap<i16, U256>,
    pub second_layer: HashMap<i16, U256>,
}

pub fn toggle_tick(
    leaves: &mut HashMap<i16, U256>,
    second_layer: &mut HashMap<i16, U256>,
    tree_root: u32,
    tick: i32,
) -> u32 {
    let mut new_tree_root = tree_root;
    let (toggled_node, node_index) = toggle_bit_in_node(leaves, tick);
    if toggled_node {
        let (toggled_node, node_index) =
            toggle_bit_in_node(second_layer, (node_index + SECOND_LAYER_OFFSET) as i32);
        if toggled_node {
            new_tree_root = (1 << node_index) ^ new_tree_root;
        }
    }
    return new_tree_root;
}

pub fn toggle_bit_in_node(tree_level: &mut HashMap<i16, U256>, bit_index: i32) -> (bool, i16) {
    let node_index = (bit_index >> 8) as i16;
    let node = *tree_level.get(&node_index).unwrap_or(&U256::ZERO);
    let node_is_zero = U256::from(node.is_zero());
    let mask = U256::from(1 << (bit_index & 0xFF)) ^ node;
    let toggled_node = node_is_zero ^ mask;

    tree_level.insert(node_index, mask);
    (toggled_node.is_zero(), node_index)
}

pub fn get_next_tick(
    leafs: &HashMap<i16, U256>,
    second_layer: &HashMap<i16, U256>,
    tree_root: u32,
    tick: i32,
) -> i32 {
    let mut initialized: bool = false;
    let mut next_tick: i32 = 0;
    let mut node_index: i16;

    let tick = tick + 1;
    node_index = (((tick >> 8) as i16 + SECOND_LAYER_OFFSET) as u16 >> 8) as i16;

    if tree_root & (1 << node_index as u16) != 0 {
        (node_index, next_tick, initialized) = next_active_bit_in_same_node(leafs, tick);
        if initialized {
            return next_tick;
        }
        (node_index, next_tick, initialized) = next_active_bit_in_same_node(
            second_layer,
            (node_index + SECOND_LAYER_OFFSET + 1) as i32,
        );
    }
    if !initialized {
        (next_tick, initialized) =
            next_active_bit_in_word(U256::from(tree_root), (node_index + 1) as i32);
        if !initialized {
            return MAX_TICK;
        }
        next_tick = first_active_bit_in_node(second_layer, next_tick);
    }
    next_tick = first_active_bit_in_node(leafs, next_tick - SECOND_LAYER_OFFSET as i32);
    next_tick
}

pub fn next_active_bit_in_same_node(
    tree_level: &HashMap<i16, U256>,
    bit_index: i32,
) -> (i16, i32, bool) {
    let node_index = (bit_index >> 8) as i16;
    let node = *tree_level.get(&node_index).unwrap_or(&U256::ZERO);
    let (next_bit_index, initialized) = next_active_bit_in_word(node, bit_index);
    (node_index, next_bit_index, initialized)
}

pub fn first_active_bit_in_node(tree_level: &HashMap<i16, U256>, node_index: i32) -> i32 {
    let bit_index_in_node = node_index >> 8;
    let node = *tree_level.get(&(node_index as i16)).unwrap_or(&U256::ZERO);
    let (bit_index, _) = next_active_bit_in_word(node, bit_index_in_node);
    bit_index
}

pub fn next_active_bit_in_word(word: U256, bit_index: i32) -> (i32, bool) {
    let next_bit_index: i32;
    let initialized: bool;
    let bit_index_in_word = bit_index & 0xFF;
    let row = word >> bit_index_in_word;
    if row == U256::ZERO {
        next_bit_index = bit_index | 255;
        initialized = false;
    } else {
        next_bit_index = bit_index + get_single_significant_bit((U256::ZERO - row) & row) as i32;
        initialized = true;
    }
    (next_bit_index, initialized)
}

// It is assumed that word contains exactly one 1-bit,
// otherwise the result will be incorrect.
pub fn get_single_significant_bit(word: U256) -> u8 {
    let mut single_bit_pos = (word & LEVEL_0).is_zero() as u8;
    single_bit_pos |= ((word & LEVEL_1).is_zero() as u8) << 7;
    single_bit_pos |= ((word & LEVEL_2).is_zero() as u8) << 6;
    single_bit_pos |= ((word & LEVEL_3).is_zero() as u8) << 5;
    single_bit_pos |= ((word & LEVEL_4).is_zero() as u8) << 4;
    single_bit_pos |= ((word & LEVEL_5).is_zero() as u8) << 3;
    single_bit_pos |= ((word & LEVEL_6).is_zero() as u8) << 2;
    single_bit_pos |= ((word & LEVEL_7).is_zero() as u8) << 1;
    single_bit_pos
}

#[cfg(test)]
mod test {}
