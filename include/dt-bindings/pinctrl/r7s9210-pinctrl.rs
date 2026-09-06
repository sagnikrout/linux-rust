//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/r7s9210-pinctrl.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// Defines macros and constants for Renesas RZ/A2 pin controller pin
// muxing functions.
//
pub const RZA2_PINS_PER_PORT: c_int = 8;
// Port names as labeled in the Hardware Manual
pub const PORT0: c_int = 0;
pub const PORT1: c_int = 1;
pub const PORT2: c_int = 2;
pub const PORT3: c_int = 3;
pub const PORT4: c_int = 4;
pub const PORT5: c_int = 5;
pub const PORT6: c_int = 6;
pub const PORT7: c_int = 7;
pub const PORT8: c_int = 8;
pub const PORT9: c_int = 9;
pub const PORTA: c_int = 10;
pub const PORTB: c_int = 11;
pub const PORTC: c_int = 12;
pub const PORTD: c_int = 13;
pub const PORTE: c_int = 14;
pub const PORTF: c_int = 15;
pub const PORTG: c_int = 16;
pub const PORTH: c_int = 17;
// No I
pub const PORTJ: c_int = 18;
pub const PORTK: c_int = 19;
pub const PORTL: c_int = 20;

//
// Create the pin index from its bank and position numbers and store in
// the upper 16 bits the alternate function identifier
//

//
// Convert a port and pin label to its global pin index
//

