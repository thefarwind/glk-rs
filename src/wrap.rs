//! This module contains safe rust functions for glk functions

use ::gen::*;

pub fn main() { unsafe { glk_main() } }
pub fn exit() { unsafe { glk_exit() } }
// TODO: Interrupt Handler
pub fn tick() { unsafe { glk_tick() } }
pub fn gestalt(sel: u32, val: u32) -> u32 { unsafe { glk_gestalt(sel, val) } }
// TODO: gestalt_ext
pub fn char_to_lower(input: u8) -> u8 { unsafe { glk_char_to_lower(input) } }
pub fn char_to_upper(input: u8) -> u8 { unsafe { glk_char_to_upper(input) } }
// TODO: lots
