//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/cirrus/pinctrl-cs47l85.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Pinctrl for Cirrus Logic CS47L85
//
// Copyright (C) 2016-2017 Cirrus Logic
//

//
// The alt func groups are the most commonly used functions we place these at
// the lower function indexes for convenience, and the less commonly used gpio
// functions at higher indexes.
//
// To stay consistent with the datasheet the function names are the same as
// the group names for that function's pins
//
// Note - all 1 less than in datasheet because these are zero-indexed
//
    static const unsigned int cs47l85_mif1_pins[] = { 8, 9 };
    static const unsigned int cs47l85_mif2_pins[] = { 10, 11 };
    static const unsigned int cs47l85_mif3_pins[] = { 12, 13 };
    static const unsigned int cs47l85_aif1_pins[] = { 14, 15, 16, 17 };
    static const unsigned int cs47l85_aif2_pins[] = { 18, 19, 20, 21 };
    static const unsigned int cs47l85_aif3_pins[] = { 22, 23, 24, 25 };
    static const unsigned int cs47l85_aif4_pins[] = { 26, 27, 28, 29 };
    static const unsigned int cs47l85_dmic4_pins[] = { 30, 31 };
    static const unsigned int cs47l85_dmic5_pins[] = { 32, 33 };
    static const unsigned int cs47l85_dmic6_pins[] = { 34, 35 };
    static const unsigned int cs47l85_spk1_pins[] = { 36, 38 };
    static const unsigned int cs47l85_spk2_pins[] = { 37, 39 };
    static const struct madera_pin_groups cs47l85_pin_groups[] = {
    { "aif1", cs47l85_aif1_pins, ARRAY_SIZE(cs47l85_aif1_pins) },
    { "aif2", cs47l85_aif2_pins, ARRAY_SIZE(cs47l85_aif2_pins) },
    { "aif3", cs47l85_aif3_pins, ARRAY_SIZE(cs47l85_aif3_pins) },
    { "aif4", cs47l85_aif4_pins, ARRAY_SIZE(cs47l85_aif4_pins) },
    { "mif1", cs47l85_mif1_pins, ARRAY_SIZE(cs47l85_mif1_pins) },
    { "mif2", cs47l85_mif2_pins, ARRAY_SIZE(cs47l85_mif2_pins) },
    { "mif3", cs47l85_mif3_pins, ARRAY_SIZE(cs47l85_mif3_pins) },
    { "dmic4", cs47l85_dmic4_pins, ARRAY_SIZE(cs47l85_dmic4_pins) },
    { "dmic5", cs47l85_dmic5_pins, ARRAY_SIZE(cs47l85_dmic5_pins) },
    { "dmic6", cs47l85_dmic6_pins, ARRAY_SIZE(cs47l85_dmic6_pins) },
    { "pdmspk1", cs47l85_spk1_pins, ARRAY_SIZE(cs47l85_spk1_pins) },
    { "pdmspk2", cs47l85_spk2_pins, ARRAY_SIZE(cs47l85_spk2_pins) },
    };
    const struct madera_pin_chip cs47l85_pin_chip = {
    .n_pins = CS47L85_NUM_GPIOS,
    .pin_groups = cs47l85_pin_groups,
    .n_pin_groups = ARRAY_SIZE(cs47l85_pin_groups),
    };
