//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/net/pcs-rzn1-miic.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2022 Schneider-Electric
//
// Clément Léger <clement.leger@bootlin.com>
//
// Reefer to the datasheet [1] section 8.2.1, Internal Connection of Ethernet
// Ports to check the available combination
//
// [1] REN_r01uh0750ej0140-rzn1-introduction_MAT_20210228.pdf
//
pub const MIIC_GMAC1_PORT: c_int = 0;
pub const MIIC_GMAC2_PORT: c_int = 1;
pub const MIIC_RTOS_PORT: c_int = 2;
pub const MIIC_SERCOS_PORTA: c_int = 3;
pub const MIIC_SERCOS_PORTB: c_int = 4;
pub const MIIC_ETHERCAT_PORTA: c_int = 5;
pub const MIIC_ETHERCAT_PORTB: c_int = 6;
pub const MIIC_ETHERCAT_PORTC: c_int = 7;
pub const MIIC_SWITCH_PORTA: c_int = 8;
pub const MIIC_SWITCH_PORTB: c_int = 9;
pub const MIIC_SWITCH_PORTC: c_int = 10;
pub const MIIC_SWITCH_PORTD: c_int = 11;
pub const MIIC_HSR_PORTA: c_int = 12;
pub const MIIC_HSR_PORTB: c_int = 13;
